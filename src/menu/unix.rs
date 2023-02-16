use std::io::{Read, StdinLock, Write, Stdout, BufWriter};
use std::os::fd::AsRawFd;
use std::time::Duration;

use nix::libc::timeval;
use nix::sys::select::{FdSet, select};
use nix::sys::time::TimeVal;

use termion::{terminal_size, cursor};
use termion::raw::{RawTerminal, IntoRawMode};
use termion::screen::{IntoAlternateScreen, AlternateScreen};

use unicode_segmentation::UnicodeSegmentation;

use super::{OptionList, Menu, Error};

mod consts;
mod text_layout;
mod rendering;

use consts::*;
use text_layout::*;

/// The ANSI escape to move the cursor 1 line up
const ANSI_UP: &str = "\x1b[A";
/// The ANSI escape to move the cursor 1 line down
const ANSI_DOWN: &str = "\x1b[B";

/// The struct which implements [`Menu`] for unix platforms.\
/// Holds a lock to stdout, so nothing else should be able to write to the console while this struct exists.
pub struct Tui {
    /// A lock to stdout.
    /// A [`BufWriter`] is used to prevent flickering, as the output will only be written once per frame.
    stdout: BufWriter<AlternateScreen<RawTerminal<Stdout>>>,
}

/// A unix specific error which can occur while showing a menu
#[derive(Debug)]
enum TuiError {
    /// An [`Error`] which should be handled. 
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

impl Drop for Tui {
    fn drop(&mut self) {
        // Can't return a Result from drop, so unwrap Result values

        // Show the cursor
        write!(self.stdout, "{}", cursor::Show).unwrap();
        self.stdout.flush().unwrap();
    }
}

impl Menu for Tui {
    fn new() -> Result<Self, std::io::Error> {
        let mut stdout = std::io::stdout()
            .into_raw_mode().unwrap()
            .into_alternate_screen().unwrap();

        // Hide the cursor
        write!(stdout, "{}", cursor::Hide)?;

        let stdout = BufWriter::new(stdout);

        Ok(Self {
            stdout,
        })
    }

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

