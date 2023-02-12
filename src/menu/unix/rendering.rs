use std::io::{Write, stdin};
use std::time::Duration;

use termion::{clear, color, cursor, style};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::menu::Error;
use super::*;

impl Tui {
    /// Moves the cursor to a specified position. The position is 0-based and relative to [`LEFT_OFFSET`] and [`TOP_OFFSET`].
    /// ### Panics
    /// * If either `x` or `y` do not fit in a u16
    fn move_cursor(&mut self, x: usize, y: usize) -> Result<(), std::io::Error> {
        let x: u16 = x.try_into().expect("Value of x should have fit in a u16");
        let y: u16 = y.try_into().expect("Value of y should have fit in a u16");
        write!(self.stdout, "{}", cursor::Goto(x + LEFT_OFFSET + 1, y + TOP_OFFSET + 1))?;
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

    /// Renders a list of items. Will cut off items with ellipses if they are too long
    /// 
    /// ### Params
    /// * items: the strings to render
    /// * scroll: the offset to render the list at if it is cut off. Should persist between calls for best UX.
    /// * selected: which item in the list is selected
    /// 
    /// ### Panics
    /// * If the terminal is too small, based on if [`get_size_checked`] fails
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
    
    /// Clears the screen and renders a border around the outside
    pub(super) fn new_frame(&mut self) -> Result<(), TuiError> {

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

        Ok(())
    }

    /// Renders a line of text, centred between [`LEFT_OFFSET`] and [`RIGHT_OFFSET`]. Will be cut off with an ellipsis if too long.
    /// 
    /// ### Panics
    /// * If the terminal is too small, based on if [`get_size_checked`] fails
    pub(super) fn render_text_centred(&mut self, text: &str, line: u16) -> Result<(), Error> {
        let (w, _) = get_size_checked().unwrap();
        let max_width = w - LEFT_OFFSET - RIGHT_OFFSET;

        let width = text.width().try_into().unwrap_or(u16::MAX);

        let total_gap = max_width.saturating_sub(width);
        let left_offset = total_gap / 2;

        write!(self.stdout, "{}", cursor::Goto(left_offset + LEFT_OFFSET + 1, line))?;
        self.render_text_with_max_width(text, max_width)?;


        Ok(())
    }

    /// Renders an empty screen with text saying 'terminal too small'.
    pub(super) fn render_too_small_error_screen(&mut self) -> Result<(), std::io::Error> {
        write!(self.stdout, "{}", clear::All)?;
        write!(self.stdout, "{}{}Terminal too small{}", cursor::Goto(1, 1), color::Fg(color::Red), color::Fg(color::Reset))?;

        Ok(())
    }

    /// Shows a TUI interface allowing the user to select an item from a list of options
    pub(super) fn choose_from_list(&mut self, items: &[&str], title: &str) -> Result<usize, Error> {
        
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
    /// * If the terminal is too small, based on if [`get_size_checked`] fails
    pub(super) fn render_graphemes_from_str<'a: 'b, 'b>(&mut self, text: &'a str, graphemes: usize, layout: &'b mut TextLayout<'a>) -> Result<(), Error> {
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