use std::io::Write;
use std::{io::StdoutLock, iter};

use super::{Error, Menu, OptionList};

/// A struct which implements [`Menu`] for any platform
pub struct Tui;

impl Menu for Tui {
    fn new() -> Result<Self, std::io::Error> {
        Ok(Self)
    }

    fn try_show_option_list_cancellable(
        &mut self,
        list: OptionList,
    ) -> Result<Option<usize>, Error> {
        let mut stdout = std::io::stdout().lock();

        let num_options = list.options.len() + 1;
        let max_width = num_options.to_string().len();

        let options_text: String = list
            .options
            .iter() // Get the strings as an iterator
            .chain(iter::once(&"Cancel".to_string())) // Add the quit message
            .enumerate() // Get the indices of the items
            .map(|(i, s)| format!("{: >max_width$}) {}\n", i + 1, s)) // Convert each item to a string with numbers right aligned
            .collect();

        writeln!(stdout, "{}", list.prompt)?;
        writeln!(stdout, "{options_text}")?;

        let choice = number_input(num_options, &mut stdout)?;

        writeln!(stdout)?;

        match choice {
            u if u == num_options => Ok(None),
            u => Ok(Some(u - 1)),
        }
    }

    fn try_show_option_list(&mut self, list: OptionList) -> Result<usize, Error> {
        let mut stdout = std::io::stdout().lock();

        let num_options = list.options.len();
        let max_width = num_options.to_string().len();

        let options_text: String = list
            .options
            .iter() // Get the strings as an iterator
            .enumerate() // Get the indices of the items
            .map(|(i, s)| format!("{: >max_width$}) {}\n", i + 1, s)) // Convert each item to a string with numbers right aligned
            .collect();

        writeln!(stdout, "{}", list.prompt)?;
        writeln!(stdout, "{options_text}")?;

        let choice = number_input(num_options, &mut stdout)?;

        writeln!(stdout)?;

        // Input is 1-based but return value is 0-based, so subtract 1
        Ok(choice - 1)
    }

    fn try_show_screen(&mut self, screen: super::Screen) -> Result<(), Error> {
        let mut stdout = std::io::stdout().lock();

        writeln!(stdout, "{}", screen.title)?;
        writeln!(stdout, "{}", screen.content)?;
        writeln!(stdout)?;

        Ok(())
    }
}

/// Gets an integer input from the user from 1 to a maximum value (inclusive). Will get the user to retype their input until a valid value is entered.
fn number_input(max: usize, stdout: &mut StdoutLock) -> Result<usize, Error> {
    loop {
        write!(stdout, "Enter your selection from 1 to {max}: ")?;
        stdout.flush()?;

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;

        let selection = buf.trim_end();
        match selection.parse() {
            Ok(u) => match u {
                0 => writeln!(stdout, "Value can't be 0")?,
                u if u <= max => return Ok(u),
                _ => writeln!(stdout, "Value too large")?,
            },
            Err(_) => writeln!(stdout, "Not a valid integer")?,
        }
    }
}
