//! A module controlling IO. IO operations should be conducted via the [Menu] trait.
//! The [init] function will provide a value which implements this trait on the current platform.
//! On unix platforms, a TUI interface will be shown, while on other platforms a less advanced fallback implementation will be used.
//! 
//! ```rust
//! let mut menu = menu::init().unwrap();
//!
//! let options = [
//!     "An option".to_string(),
//!     "Another option".to_string(),
//!     "A third option".to_string(),
//! ];
//!
//! let option_list = OptionList::new(&options, "Select an option");
//! let user_choice = menu.show_option_list(option_list);    
//!
//! let screen = Screen {
//!     title: "The result",
//!     content: &format!("You picked '{}'", options[user_choice]),
//! };
//!
//! menu.show_screen(screen);
//! ```

/// The list of options for a user to choose from
pub struct OptionList<'a> {
    /// A list of options for the player to choose from
    pub options: &'a [String],
    pub prompt: &'a str,
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
        }
    }
}

/// A screen of text that can be shown to the user
pub struct Screen<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

/// An error which can occur while displaying a menu. Some variants will only occur on specific platforms.
#[derive(Debug)]
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
mod unix;
#[cfg(unix)]
use unix::Tui;

/// Fallback implementation of the [Menu] trait for platforms which don't support ANSI escape codes
#[cfg(not(unix))]
mod fallback;
#[cfg(not(unix))]
use fallback::Tui;

pub fn init() -> Result<impl Menu, std::io::Error> {
    Tui::new()
}