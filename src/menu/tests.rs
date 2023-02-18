#![cfg(test)]

use std::collections::VecDeque;

use super::*;

#[derive(Debug, Clone, Default)]
pub struct MockMenu {
    last_screen: Option<(String, String)>,
    last_list: Option<(String, Vec<String>)>,
    numbers_to_produce: VecDeque<Option<usize>>,
}

impl Menu for MockMenu {
    fn new() -> Result<Self, std::io::Error> {
        Ok(MockMenu::default())
    }

    fn try_show_option_list(&mut self, list: OptionList) -> Result<usize, Error> {
        self.last_list = Some((list.prompt.to_string(), list.options.to_vec()));
        Ok(self.numbers_to_produce.pop_front().unwrap().unwrap())
    }

    fn try_show_option_list_cancellable(&mut self, list: OptionList) -> Result<Option<usize>, Error> {
        self.last_list = Some((list.prompt.to_string(), list.options.to_vec()));
        Ok(self.numbers_to_produce.pop_front().unwrap())
    }

    fn try_show_screen(&mut self, screen: Screen) -> Result<(), Error> {
        self.last_screen = Some((screen.title.to_string(), screen.content.to_string()));
        Ok(())
    }
}

