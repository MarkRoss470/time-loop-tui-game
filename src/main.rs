#![warn(
    clippy::all,
    clippy::pedantic,
)]

mod menu;
mod player;
mod rooms;

use player::init_player;

fn main() {
    let mut menu = menu::init().unwrap();
    let menu = &mut menu;

    let mut player = init_player();

    loop {
        player.print_room(menu);
        player.take_action(menu);
    }
    
    
}
