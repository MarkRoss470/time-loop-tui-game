#[macro_use]
mod menu;

fn main() {
    print_scroll_multiline!("This text spans\nmultiple lines\nAnd it's kinda long but not very");
    //print_scroll!("Hello world this is a long message to give me time to interrupt it");
}
