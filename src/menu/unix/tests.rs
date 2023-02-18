#![cfg(test)]

use super::text_layout::TextLayout;

/// Test normal text formatting
#[test]
fn test_normal_text_formatting() {
    let normal_text = "AAAA ".repeat(20);
    let layout = TextLayout::new(&normal_text, 50);

    assert_eq!(layout.lines[0].content.trim_end(), "AAAA ".repeat(10).trim_end());
    assert!(!layout.lines[0].dash_at_end);
    assert_eq!(layout.lines[1].content.trim_end(), "AAAA ".repeat(10).trim_end());
    assert!(!layout.lines[1].dash_at_end);
}


/// Test formatting of more advanced characters
#[test]
fn test_formatting_with_diacritics(){
    let normal_text = "ÄÄÄÄ ".repeat(20);
    let layout = TextLayout::new(&normal_text, 50);

    assert_eq!(layout.lines[0].content.trim_end(), "ÄÄÄÄ ".repeat(10).trim_end());
    assert!(!layout.lines[0].dash_at_end);
    assert_eq!(layout.lines[1].content.trim_end(), "ÄÄÄÄ ".repeat(10).trim_end());
    assert!(!layout.lines[1].dash_at_end);
}

/// Test formatting of characters which are not 1 column wide
#[test]
fn test_formatting_with_emojis() {
    let normal_text = "😀😀😀😀 ".repeat(20);
    let layout = TextLayout::new(&normal_text, 50);

    assert_eq!(layout.lines[0].content.trim_end(), "😀😀😀😀 ".repeat(5).trim_end());
    assert!(!layout.lines[0].dash_at_end);
    assert_eq!(layout.lines[1].content.trim_end(), "😀😀😀😀 ".repeat(5).trim_end());
    assert!(!layout.lines[1].dash_at_end);
    assert_eq!(layout.lines[2].content.trim_end(), "😀😀😀😀 ".repeat(5).trim_end());
    assert!(!layout.lines[2].dash_at_end);
    assert_eq!(layout.lines[3].content.trim_end(), "😀😀😀😀 ".repeat(5).trim_end());
    assert!(!layout.lines[3].dash_at_end);
}


/// Test formatting of a mix of characters
#[test]
fn test_formatting_with_special_chars() {
    let normal_text = "AB̈😀 ".repeat(20);
    let layout = TextLayout::new(&normal_text, 50);

    assert_eq!(layout.lines[0].content.trim_end(), "AB̈😀 ".repeat(10).trim_end());
    assert!(!layout.lines[0].dash_at_end);
    assert_eq!(layout.lines[1].content.trim_end(), "AB̈😀 ".repeat(10).trim_end());
    assert!(!layout.lines[1].dash_at_end);
}

/// Test line breaks
#[test]
fn test_line_wrapping() {
    let normal_text = "A".repeat(100);
    let layout = TextLayout::new(&normal_text, 50);

    assert_eq!(layout.lines[0].content, "A".repeat(49));
    assert!(layout.lines[0].dash_at_end);
    assert_eq!(layout.lines[1].content, "A".repeat(49));
    assert!(layout.lines[1].dash_at_end);
    assert_eq!(layout.lines[2].content, "AA");
    assert!(!layout.lines[2].dash_at_end);
}

/// Test line breaks with special characters
#[test]
fn test_line_wrapping_with_special_chars() {
    let normal_text = "AB̈😀".repeat(100);
    let layout = TextLayout::new(&normal_text, 101);

    assert_eq!(layout.lines[0].content, "AB̈😀".repeat(25));
    assert!(layout.lines[0].dash_at_end);
    assert_eq!(layout.lines[1].content, "AB̈😀".repeat(25));
    assert!(layout.lines[1].dash_at_end);
    assert_eq!(layout.lines[2].content, "AB̈😀".repeat(25));
    assert!(layout.lines[2].dash_at_end);
    assert_eq!(layout.lines[3].content, "AB̈😀".repeat(25));
    assert!(!layout.lines[3].dash_at_end);

}