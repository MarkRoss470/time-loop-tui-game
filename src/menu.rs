use std::time::Duration;

/// The default delay between characters used by the [print_scroll!] and [print_scroll_multiline!] macros
#[allow(dead_code)]
pub const PRINT_SCROLL_DEFAULT_DELAY: Duration = Duration::from_millis(1000 / 30);

/// A wrapper around [print_scroll] which passes the given arguments through [format!] first.\
/// The macro behaves like [println!] except the characters will be printed one at a time.
#[allow(unused_macros)]
macro_rules! print_scroll {
    ($($args:expr),*) => {{
        use $crate::menu::PRINT_SCROLL_DEFAULT_DELAY;
        use $crate::menu::print_scroll;
        print_scroll(&format!($($args),*), PRINT_SCROLL_DEFAULT_DELAY).expect("print_scroll should have succeeded")
    }};
}

/// A wrapper around [print_scroll_multiline] which passes the given arguments through [format!] first.\
#[allow(unused_macros)]
macro_rules! print_scroll_multiline {
    ($($args:expr),*) => {{
        use $crate::menu::PRINT_SCROLL_DEFAULT_DELAY;
        use $crate::menu::print_scroll_multiline;
        print_scroll_multiline(&format!($($args),*), PRINT_SCROLL_DEFAULT_DELAY).expect("print_scroll_multiline should have succeeded")
    }};
}

/// Implementations of text menu functions for unix platforms using ANSI control codes to provide fancy multi-line text and cancelling of scrolling text
#[cfg(unix)]
#[allow(dead_code)]
mod menu_unix {
    use std::io::{Read, StdinLock, StdoutLock, Write};

    use std::os::fd::AsRawFd;
    use std::time::Duration;

    use nix::libc::timeval;
    use nix::sys::{select::FdSet, time::TimeVal};
    use raw_tty::TtyModeGuard;

    // ANSI control codes for moving the cursor around
    const CURSOR_UP: &str = "\x1b[A";
    const CURSOR_DOWN: &str = "\x1b[B";
    const CURSOR_FORWARD: &str = "\x1b[C";
    const CURSOR_BACK: &str = "\x1b[D";

    /// Sleep for the provided delay, but quitting if the user presses a key.\ 
    /// If the user does press a key, the cursor's position will be reset as if they didn't
    /// 
    /// ### Params
    /// * stdin: a lock to stdin to check for key presses on
    /// * stdout: a lock to stdout to print ANSI control codes to correct the position of the cursor after the user types a character
    /// * delay: how long to sleep for
    /// 
    /// ### Returns
    /// Either a [std::io::Error] or a bool representing whether or not the user pressed a key
    fn read_char_with_timeout(
        stdin: &mut StdinLock,
        stdout: &mut StdoutLock,
        delay: Duration,
    ) -> Result<bool, std::io::Error> {
        // Create an FdSet containing only stdin
        let mut read_fd_set = FdSet::new();
        read_fd_set.insert(stdin.as_raw_fd());

        // Create a TimeVal from the provided delay
        let seconds = delay.as_secs_f64();
        let mut time_val: TimeVal = timeval {
            tv_sec: seconds.floor() as i64,
            tv_usec: (seconds.fract() * 1_000_000_f64) as i64,
        }.into();

        // Call select syscall
        let num_ready = nix::sys::select::select(
            None,
            &mut read_fd_set, // Wait until stdin ready to read
            None,            // No files to wait for write access
            None,            // No files to wait for errors
            &mut time_val     // How long to wait for
        )?;

        if num_ready != 0 {
            // Read the characters that the user typed as bytes
            let mut buf = [0_u8; 16];
            let num_bytes = stdin.read(&mut buf)?;

            // Convert the bytes to a string
            let mut buf = std::str::from_utf8(&buf[0..num_bytes]).unwrap().chars();

            // If the character was a newline, move the cursor up to put it back in the right place
            if buf.next() == Some('\n') {
                write!(stdout, "{CURSOR_UP}")?;
            }
            // If the character wasn't a newline, move the cursor back
            else {
                write!(stdout, "{CURSOR_BACK}")?;
            }

            assert_eq!(buf.next(), None, "Only one char should have been typed");

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Gets a lock to stdin and a [TtyModeGuard] at the same time.\
    /// The mode guard is set up using the stdin lock and is set to raw mode
    fn lock_stdin_raw_mode<'a>() -> Result<(StdinLock<'a>, TtyModeGuard), std::io::Error> {
        // Get a lock to stdin
        let stdin = std::io::stdin().lock();

        // Construct a TtyModeGuard and set it to raw mode
        let mut stdin_guard = TtyModeGuard::new(stdin.as_raw_fd())?;
        stdin_guard.set_raw_mode()?;

        Ok((stdin, stdin_guard))
    }

    /// Prints multiple lines of text one character at a time, with a delay between each character.\
    /// Unless specifying a non-default time, use the [print_scroll_multiline!] macro instead.
    ///
    /// ### Params
    /// * text: the string to print
    /// * delay: how long to wait between each character
    ///
    /// ### Returns
    /// An `Ok` value if writing to stdout succeeded or a [std::io::Error] if it failed
    pub fn print_scroll_multiline(text: &str, mut delay: Duration) -> Result<(), std::io::Error> {
        // Lock stdout
        let mut stdout = std::io::stdout().lock();

        // Get stdin in raw mode
        let (mut stdin, stdin_guard) = lock_stdin_raw_mode()?;

        // Get the length of the longest line
        let max_line_length = text.lines().map(|l| l.len()).max().unwrap();
        // Get the lines as a Vec of iterators over chars
        // This prevents the need for getting a new chars iterator for each column
        let mut lines: Vec<_> = text.lines().map(|l| l.chars()).collect();

        // Print enough newlines to fit the text
        write!(stdout, "{}", "\n".repeat(lines.len()))?;

        for i in 0..max_line_length {
            // Reset cursor position to bottom left corner
            write!(stdout, "{}", CURSOR_BACK.repeat(i.saturating_sub(1)))?;

            // Move cursor to top of the current column
            write!(stdout, "{}", CURSOR_UP.repeat(lines.len()))?;
            write!(stdout, "{}", CURSOR_FORWARD.repeat(i))?;

            // Iterate over lines
            for line in &mut lines {
                // Get the character for this column, if there is one
                match line.next() {
                    // If there is no character for this line, move the cursor down
                    None => write!(stdout, "{CURSOR_DOWN}")?,
                    // If there is a character, print it (which moves the cursor forward), then move the cursor back to undo that forward move and down to the next line
                    Some(c) => write!(stdout, "{c}{CURSOR_BACK}{CURSOR_DOWN}")?,
                }
            }

            // Only perform complicated sleep operation if there is actually any time to wait
            if !delay.is_zero() {
                // Flush stdout so that the characters appear on the screen
                stdout.flush()?;

                // Sleep while reading from stdin
                // If a character was typed, set delay to zero
                let char_was_typed = read_char_with_timeout(&mut stdin, &mut stdout, delay)?;
                if char_was_typed {
                    delay = Duration::ZERO;
                }
            }
        }

        // Drop the stdin guard to reset the tty and turn off raw mode
        drop(stdin_guard);

        // Print a final newline
        println!();

        Ok(())
    }

    /// Prints the given string one character at a time, with a delay between each character.\
    /// Unless specifying a non-default time, use the [print_scroll!] macro instead.
    ///
    /// ### Params
    /// * text: the string to print
    /// * delay: how long to wait between each character
    ///
    /// ### Returns
    /// An `Ok` value if writing to stdout succeeded or a [std::io::Error] if it failed
    pub fn print_scroll(text: &str, mut delay: Duration) -> Result<(), std::io::Error> {
        // Lock stdout
        let mut stdout = std::io::stdout().lock();

        // Get stdin in raw mode
        let (mut stdin, stdin_guard) = lock_stdin_raw_mode()?;

        // Loop through the characters in the string
        for c in text.chars() {
            // Write to stdout and flush it so that the character appears on screen
            write!(stdout, "{c}")?;
            stdout.flush()?;

            // Only perform complicated sleep operation if there is actually any time to wait
            if !delay.is_zero() {
                // Flush stdout so that the characters appear on the screen
                stdout.flush()?;

                // Sleep while reading from stdin
                // If a character was typed, set delay to zero
                let char_was_typed = read_char_with_timeout(&mut stdin, &mut stdout, delay)?;
                if char_was_typed {
                    delay = Duration::ZERO;
                }
            }
        }

        // Drop the stdin guard to reset the tty and turn off raw mode
        drop(stdin_guard);

        // Print the last newline
        println!();
        Ok(())
    }
}

#[cfg(unix)]
pub use menu_unix::print_scroll;
#[cfg(unix)]
pub use menu_unix::print_scroll_multiline;

/// Fallback implementations of text menu functions for non-unix platforms
#[cfg(not(unix))]
#[allow(dead_code)]
mod menu_non_unix {
    use std::{io::Write, thread::sleep, time::Duration};

    /// Prints the given string one character at a time, with a delay between each character.\
    /// Unless specifying a non-default time, use the [print_scroll!] macro instead.
    ///
    /// ### Params
    /// * text: the string to print
    /// * delay: how long to wait between each character
    ///
    /// ### Returns
    /// An `Ok` value if writing to stdout succeeded or a [std::io::Error] if it failed
    pub fn print_scroll(text: &str, delay: Duration) -> Result<(), std::io::Error> {
        // TODO: cancelling of scrolling text on non-unix platforms

        let mut stdout = std::io::stdout().lock();

        for c in text.chars() {
            write!(stdout, "{c}")?;
            sleep(delay);
        }

        Ok(())
    }

    /// Prints multiple lines of text one character at a time, with a delay between each character.\
    /// Unless specifying a non-default time, use the [print_scroll_multiline   !] macro instead.
    ///
    /// ### Params
    /// * text: the string to print
    /// * delay: how long to wait between each character
    ///
    /// ### Returns
    /// An `Ok` value if writing to stdout succeeded or a [std::io::Error] if it failed
    pub fn print_scroll_multiline(text: &str, mut delay: Duration) -> Result<(), std::io::Error> {
        // TODO: multi-line scrolling text on non-unix platforms

        print_scroll(text, delay)
    }
}

#[cfg(not(unix))]
pub use menu_non_unix::print_scroll;
#[cfg(not(unix))]
pub use menu_non_unix::print_scroll_multiline;
