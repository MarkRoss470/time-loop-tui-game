#![warn(
    clippy::all,
    clippy::pedantic,
)]

use menu::{OptionList, Menu};

use crate::menu::Screen;

#[macro_use]
mod menu;

fn main() {
    // Initialise the menu
    let mut menu = menu::init();

    // Create a Screen showing some demo text
    let screen = Screen {
        title: "Hello this is an imaginative and useful title",
        content: "This is an very useful piece of text. It is not very long but hopefully it is long enough to demonstrate the line wrapping which took me too long to implement."
    };

    // Create a list of 30 demo options
    let options: Vec<_> = (1..=30).map(|i|format!("Demo option {i}")).collect();

    let option_list = OptionList::new(&options, "Pick a demo option from this demo option list.");

    menu.show_screen(screen);
    
    menu.show_option_list(option_list);
}
