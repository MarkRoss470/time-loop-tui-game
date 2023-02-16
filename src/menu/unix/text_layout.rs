//! Contains functionality for splitting text over multiple lines

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::consts::*;

/// One line of text in the formatted output
#[derive(Debug)]
pub(super) struct TextLine<'a> {
    /// The line's content
    pub(super) content: &'a str,
    /// Whether the line's content cuts off in the middle of a word and a dash should be rendered
    pub(super) dash_at_end: bool,
    /// Length measured in graphemes
    pub(super) length: usize,
}

/// The formatted layout some text
#[derive(Debug)]
pub(super) struct TextLayout<'a> {
    /// The maximum length of a line. This is measured in columns using [unicode_width::UnicodeWidthStr], not in characters or graphemes.
    pub(super) max_width: usize,
    /// The formatted lines
    pub(super) lines: Vec<TextLine<'a>>,
}

impl<'a> TextLayout<'a> {
    /// Adds a source line to the layout. The line will be wrapped, so it may span multiple render lines
    fn add_source_line(&mut self, line: &'a str) {
        
        // The x position of the end of the current render line 
        let mut x = 0;
        // Points to the first char in the current render line
        let mut current_render_line_start = 0;
        // Points past the end of the last char in the current render line
        let mut current_render_line_end = 0;
        // Whether the current word is at the first word on a new line
        let mut is_line_start = true;

        for word in line.split(' ') {
            // The display width of the word
            let width = word.width();

            // If the word fits on the current line
            if x + width <= self.max_width {
                // At the start of the line, current_render_line_end points to the first letter, where usually it would point to the first space
                // In this situation, don't add the width of the space
                if is_line_start {
                    current_render_line_end += word.len();
                    is_line_start = false;
                } else {
                    // + 1 to account for the space between the words
                    current_render_line_end += word.len() + 1;
                }
                
                // + 1 to account for the space between the words
                x += width + 1;
                continue;
            }



            // The remaining width on the current line
            let width_left = self.max_width - (x - 1);

            // Whether a hyphen is needed to print the word - if the word is longer than the line width
            let needs_hyphen = width > width_left * 2;

            // If the size of the upper segment would be big enough
            let first_segment_long_enough = width_left >= TEXT_WRAPPING_MIN_SEGMENT_SIZE;
            // If the size of the lower segment would be big enough
            let last_segment_long_enough = (width - width_left) >= TEXT_WRAPPING_MIN_SEGMENT_SIZE;
            // Whether the word could be hyphenated
            let could_hyphenate = first_segment_long_enough && last_segment_long_enough;

            // Whether to hyphenate the word
            let render_hyphen = needs_hyphen || could_hyphenate;
            // Whether to move to a new line
            let move_to_new_line = x != 0 && !could_hyphenate;

            // If the word should be hyphenated
            if render_hyphen {
                // The index that the word starts at
                let word_start_index;

                // If the word should be printed on a new line
                if move_to_new_line {
                    let content = &line[current_render_line_start..current_render_line_end];
                    self.lines.push(TextLine {
                        content,
                        dash_at_end: false,

                        length: content.graphemes(true).count(),
                    });

                    current_render_line_end += 1;
                    current_render_line_start = current_render_line_end;
                    x = 0;
                    is_line_start = true;

                    word_start_index = current_render_line_end;
                } else {
                    word_start_index = current_render_line_end + 1;
                }

                // Loop through the graphemes
                for (i, g) in word.grapheme_indices(true) {
                    let g_width = g.width();

                    x += g_width;

                    // If the grapheme would go over the end of the line, hyphenate and go to the next line
                    if x > self.max_width {
                        let content = &line[current_render_line_start..current_render_line_end];
                        self.lines.push(TextLine {
                            content,
                            dash_at_end: true,
                            length: content.graphemes(true).count(),
                        });

                        current_render_line_start = current_render_line_end;
                        x = g_width + 1;
                    }

                    current_render_line_end = word_start_index + i;
                }

                // Update end pointer to point past the end of the string
                current_render_line_end = word_start_index + word.len();
            } 
            // If the word does not need to be hyphenated
            else {

                // Move to new line
                let content = &line[current_render_line_start..current_render_line_end];
                self.lines.push(TextLine {
                    content,
                    dash_at_end: false,

                    length: content.graphemes(true).count(),
                });

                // Set current_render_line_start to the character after the space which current_render_line_end points to
                current_render_line_start = current_render_line_end + 1;
                current_render_line_end += word.len() + 1;

                x = width;

            }
        }

        // Add the rest of the characters on a new line
        let content = &line[current_render_line_start..current_render_line_end];
        self.lines.push(TextLine {
            content,
            dash_at_end: false,
            length: content.graphemes(true).count(),
        });

    }

    /// Creates a new [`TextLayout`] from a given str
    pub(super) fn new(text: &'a str, max_width: usize) -> Self {
        let mut s = Self {
            max_width,
            lines: Vec::new(),
        };

        for line in text.split('\n') {
            s.add_source_line(line);
        }

        s
    }
}
