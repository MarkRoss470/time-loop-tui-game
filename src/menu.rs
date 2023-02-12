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
mod unix;
#[cfg(unix)]
use unix::Tui;

/// Fallback implementation of the [Menu] trait for platforms which don't support ANSI escape codes
#[cfg(not(unix))]
mod fallback;
#[cfg(not(unix))]
use fallback::Tui;

pub fn init() -> impl Menu {
    Tui::new()
}