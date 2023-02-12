use std::iter;
use std::io::Write;

use super::{Menu, Error, OptionList};

pub struct Tui;

impl Tui {
    pub(super) fn new() -> Self {Self}
}


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
