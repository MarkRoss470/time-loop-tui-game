#![warn(
    clippy::all,
    clippy::pedantic,
)]

use menu::{OptionList, Menu};

use crate::menu::Screen;

#[macro_use]
mod menu;

fn main() {
 let mut menu = menu::init().unwrap();
 
 let options = [
     "An option".to_string(),
     "Another option".to_string(),
     "A third option".to_string(),
 ];
 
 let option_list = OptionList::new(&options, "Select an option");
 let user_choice = menu.show_option_list(option_list);    
 
 let screen = Screen {
     title: "The result",
     content: &format!("You picked '{}'", options[user_choice]),
 };
 
 menu.show_screen(screen);
}
