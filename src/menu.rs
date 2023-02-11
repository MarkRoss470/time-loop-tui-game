/// The list of options for a user to choose from
pub struct OptionList<'a> {
    pub options: &'a [String],
    pub prompt: &'a str,

    /// This private member prevents code outside this module from using member initialisation.
    /// This forces code to use the provided [`new`][OptionList::new] method, which does validation on `options`
    _private: ()
}

impl<'a> OptionList<'a> {
    /// Constructs a new [`OptionList`] from a given list of options and a prompt.\
    /// 
    /// ### Panics
    /// If `options` is empty
    pub fn new(options: &'a [String], prompt: &'a str) -> Self {
        assert!(!options.is_empty(), "Options should not be empty");

        Self {
            options,
            prompt,

            _private: ()
        }
    }
}

pub struct Screen<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

/// An error which can occur while displaying a menu. Some variants will only occur on specific platforms.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Io(std::io::Error),
    IncompatibleCharacter,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::IncompatibleCharacter => write!(f, "an incompatible character was encountered")
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

/// A trait for displaying menus to the user
pub trait Menu {
    /// Show a list of options. Will return the index of the option the user selected
    fn show_option_list(&mut self, list: OptionList) -> usize {self.try_show_option_list(list).unwrap()}
    /// Fallible version of [`show_option_list`][Menu::show_option_list]
    fn try_show_option_list(&mut self, list: OptionList) -> Result<usize, Error>;

    /// Show a list of options, with a cancel option. Returns [`None`] if the user selects cancel, 
    /// or a [`Some`] value containing the index of the option the user selected
    fn show_option_list_cancellable(&mut self, list: OptionList) -> Option<usize>{self.try_show_option_list_cancellable(list).unwrap()}
    /// Fallible version of [`show_option_list_cancellable`][Menu::show_option_list_cancellable]
    fn try_show_option_list_cancellable(&mut self, list: OptionList) -> Result<Option<usize>, Error>;

    /// Show a screen
    fn show_screen(&mut self, screen: Screen) {self.try_show_screen(screen).unwrap()}
    /// Fallible version of [`try_show_screen`][Menu::show_screen]
    fn try_show_screen(&mut self, screen: Screen) -> Result<(), Error>;

}

/// Implementation of the [Menu] trait for unix platforms using the [termion] library
#[cfg(unix)]
pub mod unix {
    use std::io::{Read, StdinLock, Write, Stdout, stdin, BufWriter};
    use std::os::fd::AsRawFd;
    use std::time::Duration;

    use nix::libc::timeval;
    use nix::sys::select::{FdSet, select};
    use nix::sys::time::TimeVal;

    use termion::{clear, cursor, terminal_size, style, color};
    use termion::raw::{RawTerminal, IntoRawMode};
    use termion::screen::{IntoAlternateScreen, AlternateScreen};
    
    use unicode_segmentation::UnicodeSegmentation;
    use unicode_width::UnicodeWidthStr;

    use super::{OptionList, Menu, Error};

    const ANSI_UP: &str = "\x1b[A";
    const ANSI_DOWN: &str = "\x1b[B";

    /// The struct which implements [Menu] for unix platforms.\
    /// Holds a lock to stdout, so nothing else should be able to write to the console while this struct exists.
    pub struct Tui {
        stdout: BufWriter<AlternateScreen<RawTerminal<Stdout>>>,
    }

    /// A unix specific error which can occur while showing a menu
    #[derive(Debug)]
    enum TuiError {
        MenuError(Error),
        /// If the terminal is too small to fit the content
        TerminalTooSmall,
    }

    impl From<Error> for TuiError {
        fn from(value: Error) -> Self {
            Self::MenuError(value)
        }
    }

    impl From<std::io::Error> for TuiError {
        fn from(value: std::io::Error) -> Self {
            Self::MenuError(Error::Io(value))
        }
    }

    /// The pattern which is shown at the top of the screen
    const BORDER_PATTERN_HORIZONTAL: &str = "=-";
    /// The pattern which is shown at the bottom of the screen
    const BORDER_PATTERN_VERTICAL: &str = "\\/";

    /// The character to be printed in the top-left of the screen
    const TOP_LEFT_CORNER: char = '/';
    /// The character to be printed in the top-right of the screen
    const TOP_RIGHT_CORNER: char = '\\';
    /// The character to be printed in the bottom-left of the screen
    const BOTTOM_LEFT_CORNER: char = '\\';
    /// The character to be printed in the bottom-right of the screen
    const BOTTOM_RIGHT_CORNER: char = '/';

    /// The offset of content from the left hand side of the screen
    const LEFT_OFFSET: u16 = 3;
    /// The offset of content from the top of the screen
    const TOP_OFFSET: u16 = 2;
    /// The offset of content from the bottom of the screen
    const BOTTOM_OFFSET: u16 = 2;
    /// The offset of content from the right hand side of the screen
    const RIGHT_OFFSET: u16 = 2;

    /// The smallest size a segment will be when wrapping text
    const TEXT_WRAPPING_MIN_SEGMENT_SIZE: usize = 5;

    /// The target framerate
    const FPS: u64 = 30;
    const MS_PER_FRAME: u64 = 1000 / FPS;
    
    /// The target number of characters to print per second when scrolling text
    const CHARS_PER_SECOND: u64 = 50;
    const MS_PER_CHAR: u64 = 1000 / CHARS_PER_SECOND;

    #[derive(Debug)]
    struct TextLine<'a> {
        content: &'a str,
        dash_at_end: bool,
        /// Length measured in graphemes
        length: usize,
    }

    #[derive(Debug)]
    struct TextLayout<'a> {
        max_width: usize,
        lines: Vec<TextLine<'a>>,
    }

    impl<'a> TextLayout<'a> {
        /// Adds a source line to the layout. The line will be wrapped, so it may span multiple render lines
        fn add_source_line(&mut self, line: &'a str) {
            
            // The x position of the end of the current render line 
            let mut x = 0;
            // Points to the first char in the current render line
            let mut current_render_line_start = 0;
            // Points past the end of the last char in the current render line
            let mut current_render_line_end = 0;

            for word in line.split(' ') {
                // The display width of the word
                let width = word.width();

                // If the word fits on the current line
                if x + width <= self.max_width {
                    // At the start of the line, current_render_line_end points to the first letter, where usually it would point to the first space
                    // In this situation, don't add the width of the space
                    if current_render_line_end == current_render_line_start {
                        current_render_line_end += word.len();
                    } else {
                        // + 1 to account for the space between the words
                        current_render_line_end += word.len() + 1;
                    }
                    
                    // + 1 to account for the space between the words
                    x += width + 1;
                    continue;
                }

                // The remaining width on the current line
                let width_left = self.max_width - (x - 1);

                // Whether a hyphen is needed to print the word - if the word is longer than the line width
                let needs_hyphen = width > width_left * 2;

                // If the size of the upper segment would be big enough
                let first_segment_long_enough = width_left >= TEXT_WRAPPING_MIN_SEGMENT_SIZE;
                // If the size of the lower segment would be big enough
                let last_segment_long_enough = (width - width_left) >= TEXT_WRAPPING_MIN_SEGMENT_SIZE;
                // Whether the word could be hyphenated
                let could_hyphenate = first_segment_long_enough && last_segment_long_enough;

                // Whether to hyphenate the word
                let render_hyphen = needs_hyphen || could_hyphenate;
                // Whether to move to a new line
                let move_to_new_line = x != 0 && !could_hyphenate;

                // If the word should be hyphenated
                if render_hyphen {
                    // The index that the word starts at
                    let word_start_index;

                    // If the word should be printed on a new line
                    if move_to_new_line {
                        let content = &line[current_render_line_start..current_render_line_end];
                        self.lines.push(TextLine {
                            content,
                            dash_at_end: false,

                            length: content.graphemes(true).count(),
                        });
    
                        current_render_line_end += 1;
                        current_render_line_start = current_render_line_end;
                        x = 0;

                        word_start_index = current_render_line_end;
                    } else {
                        word_start_index = current_render_line_end + 1;
                    }

                    // Loop through the graphemes
                    for (i, g) in word.grapheme_indices(true) {
                        let g_width = g.width();

                        x += g_width;

                        // If the grapheme would go over the end of the line, hyphenate and go to the next line
                        if x > self.max_width {
                            let content = &line[current_render_line_start..current_render_line_end];
                            self.lines.push(TextLine {
                                content,
                                dash_at_end: true,
                                length: content.graphemes(true).count(),
                            });

                            current_render_line_start = current_render_line_end;
                            x = g_width + 1;
                        }

                        current_render_line_end = word_start_index + i;
                    }

                    // Update end pointer to point past the end of the string
                    current_render_line_end = word_start_index + word.len();
                } 
                // If the word does not need to be hyphenated
                else {

                    // Move to new line
                    let content = &line[current_render_line_start..current_render_line_end];
                    self.lines.push(TextLine {
                        content,
                        dash_at_end: false,

                        length: content.graphemes(true).count(),
                    });

                    // Set current_render_line_start to the character after the space which current_render_line_end points to
                    current_render_line_start = current_render_line_end + 1;
                    current_render_line_end += word.len() + 1;

                    x = width;

                }
            }

            // Add the rest of the characters on a new line
            let content = &line[current_render_line_start..current_render_line_end];
            self.lines.push(TextLine {
                content,
                dash_at_end: false,
                length: content.graphemes(true).count(),
            });

        }

        /// Creates a new [`TextLayout`] from a given str
        fn new(text: &'a str, max_width: usize) -> Self {
            let mut s = Self {
                max_width,
                lines: Vec::new(),
            };

            for line in text.split('\n') {
                s.add_source_line(line);
            }

            s
        }
    }

    /// Uses the unix select syscall to poll stdin for content without blocking.\
    /// Reads a maximum of 256 bytes, so should not be used for long input
    fn poll_stdin(stdin: &mut StdinLock) -> Result<Option<String>, std::io::Error> {
        // Create a new FdSet containing only stdin
        let mut fd_set = FdSet::new();
        fd_set.insert(stdin.as_raw_fd());

        // Create a TimeVal of 0 seconds
        let mut zero_time: TimeVal = timeval{
            tv_sec: 0,
            tv_usec: 0,
        }.into();

        // Call the select syscall
        let num_files = select (
            None,
            &mut fd_set,
            None,
            None,
            &mut zero_time
        )?;

        // If stdout was ready to read, get the data from it
        if num_files > 0 {
            let mut buf = [0_u8; 256];
            let num_bytes = stdin.read(&mut buf)?;
            let buf = std::str::from_utf8(&buf[..num_bytes]).unwrap();
            Ok(Some(buf.to_string()))
        } else {
            Ok(None)
        }
    }

    /// Gets the size of the terminal, or an error if the terminal is too small
    fn get_size_checked() -> Result<(u16, u16), TuiError> {
        let (w, h) = terminal_size()?;

        if w < LEFT_OFFSET + RIGHT_OFFSET + 50 || h < TOP_OFFSET + BOTTOM_OFFSET + 10 {
            Err(TuiError::TerminalTooSmall)
        } else {
            Ok((w, h))
        }
    }


    impl Tui {
        /// Constructs a new Tui with a new lock to stdout
        pub(super) fn new() -> Self {
            let stdout = std::io::stdout()
                .into_raw_mode().unwrap()
                .into_alternate_screen().unwrap();
            
            let stdout = BufWriter::new(stdout);

            Self {
                stdout,
            }
        }

        /// Moves the cursor to a specified position. The position is 0-based and relative to [`LEFT_OFFSET`] and [`TOP_OFFSET`].
        /// ### Panics
        /// * If either `x` or `y` do not fit in a u16
        fn move_cursor(&mut self, x: usize, y: usize) -> Result<(), std::io::Error> {
            let x: u16 = x.try_into().expect("Value of x should have fit in a u16");
            let y: u16 = y.try_into().expect("Value of y should have fit in a u16");
            write!(self.stdout, "{}", cursor::Goto(x + LEFT_OFFSET + 1, y + TOP_OFFSET + 1))?;
            Ok(())
        }

        /// Clears the screen and renders a border around the outside
        fn new_frame(&mut self) -> Result<(), TuiError> {

            let (w, h) = get_size_checked()?;

            // Clear screen
            write!(self.stdout, "{}", clear::All)?;

            // Go to top left corner
            write!(self.stdout, "{}", cursor::Goto(1, 1))?;
            // Print top left corner
            write!(self.stdout, "{TOP_LEFT_CORNER}")?;
            // Print top line
            write!(self.stdout, "{}", BORDER_PATTERN_HORIZONTAL.chars().cycle().take((w - 2) as usize).collect::<String>())?;
            // Print top right corner
            write!(self.stdout, "{TOP_RIGHT_CORNER}")?;

            // Go to one below top left corner
            write!(self.stdout, "{}", cursor::Goto(1, 2))?;
            // Print left line
            write!(self.stdout, "{}", BORDER_PATTERN_VERTICAL.chars().cycle().take(h as usize).map(|s|format!("{s}{}{}", cursor::Down(1), cursor::Left(1))).collect::<String>())?;

            // Go to one below top right corner
            write!(self.stdout, "{}", cursor::Goto(w, 2))?;
            // Print right line
            write!(self.stdout, "{}", BORDER_PATTERN_VERTICAL.chars().cycle().take(h as usize).map(|s|format!("{s}{}", cursor::Down(1))).collect::<String>())?;

            // Go to bottom left corner
            write!(self.stdout, "{}", cursor::Goto(1, h))?;
            // Print bottom left corner
            write!(self.stdout, "{BOTTOM_LEFT_CORNER}")?;
            // Print bottom line
            write!(self.stdout, "{}", BORDER_PATTERN_HORIZONTAL.chars().cycle().take((w - 2) as usize).collect::<String>())?;
            // Print bottom right corner
            write!(self.stdout, "{BOTTOM_RIGHT_CORNER}")?;

            // Hide the cursor
            write!(self.stdout, "{}", cursor::Hide)?;

            Ok(())
        }

        /// Renders a list of items. Will cut off items with ellipses if they are too long
        /// 
        /// ### Params
        /// * items: the strings to render
        /// * scroll: the offset to render the list at if it is cut off. Should persist between calls for best UX.
        /// * selected: which item in the list is selected
        /// 
        /// ### Panics
        /// * If the terminal is too small, based on if [`get_size_checked`][Tui::get_size_checked] fails
        fn render_list(&mut self, items: &[&str], scroll: &mut usize, selected: usize) -> Result<(), Error> {
            let num_items = items.len();

            let (w, h) = get_size_checked().unwrap();
            let max_lines = (h - TOP_OFFSET - BOTTOM_OFFSET) as usize;
            let max_width = w - LEFT_OFFSET - RIGHT_OFFSET - 1;

            // Calculate formatting
            let requires_scroll = num_items > max_lines;
            // These are only for calculating scroll, and will be recalculated afterward.
            let ellipsis_at_end = requires_scroll && *scroll + max_lines < num_items;
            let lines_to_render = if ellipsis_at_end {max_lines - 1} else {num_items};

            // If the screen is big enough for all the lines, set scroll to 0
            if !requires_scroll {*scroll = 0}
            // If the screen just got bigger, make sure the list still takes up the whole space
            else if *scroll > num_items - max_lines {*scroll = num_items - max_lines}
            // If the current selection is off the top of the screen, scroll up
            else if *scroll > selected {*scroll = selected}
            // If the current selection is off the bottom of the screen, scroll down
            else if *scroll + lines_to_render <= selected {*scroll = selected - lines_to_render + 1}

            // Recalculate formatting as scroll may have changed
            let ellipsis_at_end = requires_scroll && *scroll + max_lines < num_items;
            let num_lines_to_render = if ellipsis_at_end {max_lines - 1} else {num_items};

            let render_lines = items.iter()
                .enumerate() // Get the option numbers
                .skip(*scroll) // Skip the number of items needed to get the right scroll
                .take(num_lines_to_render) // Only render the number of lines needed
                .enumerate();
            
            // Render the lines
            for (screen_line_number, (option_number, line)) in render_lines {

                // Get whether this is the selected line
                self.move_cursor(0, screen_line_number)?;

                // If this is the currently selected line, highlight the option
                if option_number == selected {
                    write!(self.stdout, "{}", style::Invert)?;
                }

                // Write the line text
                self.render_text_with_max_width(line, max_width)?;

                // Undo any highlighting
                write!(self.stdout, "{}", style::NoInvert)?;
            }

            // If the 
            if ellipsis_at_end {
                self.move_cursor(0, num_lines_to_render)?;
                write!(self.stdout, "⋯")?;
            }

            Ok(())

        }

        /// Renders a line of text, centred between [`LEFT_OFFSET`] and [`RIGHT_OFFSET`]. Will be cut off with an ellipsis if too long.
        /// 
        /// ### Panics
        /// * If the terminal is too small, based on if [`get_size_checked`][Tui::get_size_checked] fails
        fn render_text_centred(&mut self, text: &str, line: u16) -> Result<(), Error> {
            let (w, _) = get_size_checked().unwrap();
            let max_width = w - LEFT_OFFSET - RIGHT_OFFSET;

            let width = text.width().try_into().unwrap_or(u16::MAX);

            let total_gap = max_width.saturating_sub(width);
            let left_offset = total_gap / 2;

            write!(self.stdout, "{}", cursor::Goto(left_offset + LEFT_OFFSET + 1, line))?;
            self.render_text_with_max_width(text, max_width)?;


            Ok(())
        }

        /// Renders a line of text with a maximum width, cut off by an ellipsis if too long. The text will be written at the current cursor position.
        fn render_text_with_max_width(&mut self, line: &str, max_width: u16) -> Result<(), Error> {
            let mut current_width: u16 = 0;
            
            for c in line.graphemes(true) {
                let width: u16 = c.width().try_into().map_err(|_|Error::IncompatibleCharacter)?;
                current_width += width;
                if current_width > max_width {
                    write!(self.stdout, "⋯")?;
                    break;
                }
        
                write!(self.stdout, "{c}")?;
            }

            Ok(())
        }

        /// Renders an empty screen with text saying 'terminal too small'.
        fn render_too_small_error_screen(&mut self) -> Result<(), std::io::Error> {
            write!(self.stdout, "{}", clear::All)?;
            write!(self.stdout, "{}{}Terminal too small{}", cursor::Goto(1, 1), color::Fg(color::Red), color::Fg(color::Reset))?;

            Ok(())
        }

        /// Shows a TUI interface allowing the user to select an item from a list of options
        fn choose_from_list(&mut self, items: &[&str], title: &str) -> Result<usize, Error> {
            
            let num_items = items.len();
            
            // Init the UI state
            let mut selected = 0;
            let mut scroll_offset = 0;

            // Lock stdin
            let mut stdin = stdin().lock();
            
            // Loop until the user chooses an option
            loop {
                // Show the frame and wait
                self.stdout.flush()?;
                std::thread::sleep(Duration::from_millis(MS_PER_FRAME));

                // Render the border, propagating errors
                if let Err(e) = self.new_frame() {
                    match e {
                        TuiError::TerminalTooSmall => {
                            self.render_too_small_error_screen()?;
                            continue
                        }
                        TuiError::MenuError(m) => return Err(m),
                    }
                };
                
                // Render the title
                self.render_text_centred(title, TOP_OFFSET)?;

                // Render the list items
                self.render_list(items, &mut scroll_offset, selected)?;

                // Handle user input
                if let Some(input) = poll_stdin(&mut stdin)? {
                    // Up arrow
                    if input == ANSI_UP && selected != 0 {
                        selected -= 1;
                    } 
                    // Down arrow
                    else if input == ANSI_DOWN && selected != num_items - 1 {
                        selected += 1;
                    } 
                    // Enter
                    else if input == "\r" || input == "\n" {
                        return Ok(selected)
                    }
                }
            }
        }
    
        /// Renders a given number of graphemes from a string.
        /// 
        /// ### Params:
        /// * text: the text to render from
        /// * graphemes: the number of characters to render
        /// * layout: a reference to cache the generated [`TextLayout`]
        /// 
        /// ### Panics:
        /// * If the terminal is too small, based on if [`get_size_checked`][Tui::get_size_checked] fails
        fn render_graphemes_from_str<'a: 'b, 'b>(&mut self, text: &'a str, graphemes: usize, layout: &'b mut TextLayout<'a>) -> Result<(), Error> {
            // Get the size of the terminal
            let (w, h) = get_size_checked().unwrap();

            // Calculate the maximum width and height
            let max_width = (w - LEFT_OFFSET - RIGHT_OFFSET - 1) as usize;
            let max_lines = (h - TOP_OFFSET - BOTTOM_OFFSET) as usize;

            // Regenerate layout if it was generated for a different width
            if layout.max_width != max_width {
                *layout = TextLayout::new(text, max_width);
            }

            // How many lines are needed to get the required number of graphemes
            let mut needed_lines: usize = 0;
            // How many graphemes there have been in the lines so far
            let mut graphemes_so_far = 0;
            // Whether all lines need to be rendered
            let mut render_all_lines = true;

            for line in &layout.lines {
                needed_lines += 1;
                // If this line contains the end of the required number of graphemes
                if graphemes_so_far + line.length > graphemes {
                    render_all_lines = false;
                    break
                }
                // Keep track of the number of graphemes
                graphemes_so_far += line.length;
            }

            // Calculate number of lines to skip, if any
            let lines_to_skip = needed_lines.saturating_sub(max_lines);
            let lines_to_render = needed_lines - lines_to_skip;

            for (screen_line, (layout_line, line)) in layout.lines.iter().enumerate().skip(lines_to_skip).take(lines_to_render).enumerate() {
                // If the whole line must be printed
                if render_all_lines || layout_line != needed_lines - 1 {
                    self.move_cursor(0, screen_line)?;
                    write!(self.stdout, "{}", line.content)?;

                    // Print dash for words split over multiple lines
                    if line.dash_at_end {
                        write!(self.stdout, "-")?;
                    }

                }
                // If this is the last line for this frame, print only the required number of graphemes
                else {
                    let (end_index, _) = line.content.grapheme_indices(true).nth(graphemes - graphemes_so_far).unwrap();
                    self.move_cursor(0, screen_line)?;
                    write!(self.stdout, "{}", &line.content[..end_index])?;
                }
            }

            Ok(())
        }

    }

    impl Menu for Tui {
        fn try_show_option_list(&mut self, list: OptionList<'_>) -> Result<usize, Error> {
            // Get options from list with numbers
            let items: Vec<_> = list.options.iter()
                .map(String::as_str)
                .collect();

            let choice = self.choose_from_list(&items, list.prompt)?;
            Ok(choice)
        }

        fn try_show_option_list_cancellable(&mut self, list: OptionList) -> Result<Option<usize>, Error> {
            // Get options from list, including cancel option
            let items: Vec<_> = list.options.iter()
                .map(String::as_str)
                .chain(std::iter::once("Cancel"))
                .collect();

            // Show list UI
            let selection = self.choose_from_list(&items, list.prompt)?;

            // Check whether the user pressed 'cancel'
            if selection == list.options.len() {
                Ok(None)
            } else {
                Ok(Some(selection))
            }
        }
        
        fn try_show_screen(&mut self, screen: super::Screen) -> Result<(), Error> {

            // Lock stdin
            let mut stdin = std::io::stdin().lock();
            // A cache for the layout so that it doesn't need to be regenerated every frame
            let mut layout = TextLayout::new(screen.content, 100);
            // The number of graphemes in the string
            let num_graphemes = screen.content.graphemes(true).count();

            // The number of milliseconds that have passed, used to compute how many graphemes to render
            let mut ms = 0;
            // Whether to render all graphemes in the string
            let mut render_all_graphemes = false;

            // Loop until the user quits
            loop {
                // Show the frame and wait
                self.stdout.flush()?;
                std::thread::sleep(Duration::from_millis(MS_PER_FRAME));
                ms += MS_PER_FRAME;

                // Calculate how many graphemes to render this frame
                let graphemes = if render_all_graphemes {
                    num_graphemes
                } else {
                    let graphemes = (ms / MS_PER_CHAR) as usize;
                    // If the scroll has reached the end of the string, set render_all_graphemes to true
                    // This means that the next character press will quit instead of trying to skip the scroll
                    if graphemes > num_graphemes {render_all_graphemes = true}
                    graphemes
                };

                match self.new_frame() {
                    Err(TuiError::TerminalTooSmall) => {
                        self.render_too_small_error_screen()?;
                        continue
                    }
                    Err(TuiError::MenuError(m)) => return Err(m),
                    Ok(()) => (),
                };

                self.render_graphemes_from_str(screen.content, graphemes, &mut layout)?;

                self.render_text_centred(screen.title, TOP_OFFSET)?;

                if poll_stdin(&mut stdin)?.is_some() {
                    // If the scroll has finished, break
                    if render_all_graphemes {break}
                    // Otherwise, skip the rest of the scroll
                    render_all_graphemes = true;
                }
            }

            Ok(())
        }

    }
}

#[cfg(not(unix))]
mod fallback {
    use std::iter;
    use std::io::Write;

    use super::{Menu, Error, OptionList};

    pub struct Tui;

    impl Menu for Tui {
        fn try_show_option_list_cancellable(&mut self, list: OptionList) -> Result<Option<usize>, Error> {
            let num_options = list.options.len() + 1;
            let max_width = num_options.to_string().len();

            let options_text: String = list.options
                .iter() // Get the strings as an iterator
                .chain(iter::once(&"Cancel".to_string())) // Add the quit message
                .enumerate() // Get the indices of the items
                .map(|(i, s)|format!("{: >max_width$}) {}\n", i + 1, s)) // Convert each item to a string with numbers right aligned
                .collect();

            println!("{}", list.prompt);
            println!("{options_text}");
            println!();

            match number_input(num_options)? {
                u if u == num_options => Ok(None),
                u => Ok(Some(u)) 
            }
        }

        fn try_show_option_list(&mut self, list: OptionList) -> Result<usize, Error> {
            let mut stdout = std::io::stdout().lock();
            
            let num_options = list.options.len();
            let max_width = num_options.to_string().len();

            let options_text: String = list.options
                .iter() // Get the strings as an iterator
                .enumerate() // Get the indices of the items
                .map(|(i, s)|format!("{: >max_width$}) {}\n", i + 1, s)) // Convert each item to a string with numbers right aligned
                .collect();

            writeln!(stdout, "{}", list.prompt)?;
            writeln!(stdout, "{options_text}")?;
            writeln!(stdout)?;
            
            number_input(num_options)
        }
    
        fn try_show_screen(&mut self, screen: super::Screen) -> Result<(), Error> {
            let mut stdout = std::io::stdout().lock();
            
            writeln!(stdout, "{}", screen.title)?;
            writeln!(stdout, "{}", screen.content)?;

            Ok(())
        }
    }

    fn number_input(max: usize) -> Result<usize, Error> {
        loop {
            print!("Enter your selection from 1 to {max}: ");
            std::io::stdout().flush()?;

            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf)?;

            let selection = buf.trim_end();
            match selection.parse() {
                Ok(u) => match u {
                    0 => println!("Value can't be 0"),
                    u if u <= max => return Ok(u),
                    _ => println!("Value too large")
                },
                Err(_) => println!("Not a valid integer")
            }

        }
    }

}

pub fn init() -> impl Menu {
    #[cfg(unix)]
    return unix::Tui::new();
    
    #[cfg(not(unix))]
    return fallback::Tui;
}