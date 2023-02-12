/// The pattern which is shown at the top of the screen
pub(super) const BORDER_PATTERN_HORIZONTAL: &str = "=-";
/// The pattern which is shown at the bottom of the screen
pub(super) const BORDER_PATTERN_VERTICAL: &str = "\\/";

/// The character to be printed in the top-left of the screen
pub(super) const TOP_LEFT_CORNER: char = '/';
/// The character to be printed in the top-right of the screen
pub(super) const TOP_RIGHT_CORNER: char = '\\';
/// The character to be printed in the bottom-left of the screen
pub(super) const BOTTOM_LEFT_CORNER: char = '\\';
/// The character to be printed in the bottom-right of the screen
pub(super) const BOTTOM_RIGHT_CORNER: char = '/';

/// The offset of content from the left hand side of the screen
pub(super) const LEFT_OFFSET: u16 = 3;
/// The offset of content from the top of the screen
pub(super) const TOP_OFFSET: u16 = 2;
/// The offset of content from the bottom of the screen
pub(super) const BOTTOM_OFFSET: u16 = 2;
/// The offset of content from the right hand side of the screen
pub(super) const RIGHT_OFFSET: u16 = 2;

/// The smallest size a segment will be when wrapping text
pub(super) const TEXT_WRAPPING_MIN_SEGMENT_SIZE: usize = 5;

/// The target framerate
pub(super) const FPS: u64 = 30;
pub(super) const MS_PER_FRAME: u64 = 1000 / FPS;

/// The target number of characters to print per second when scrolling text
pub(super) const CHARS_PER_SECOND: u64 = 50;
pub(super) const MS_PER_CHAR: u64 = 1000 / CHARS_PER_SECOND;