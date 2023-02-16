#![warn(
    clippy::missing_docs_in_private_items,
    missing_debug_implementations,
    clippy::all,
    clippy::pedantic,
    //clippy::nursery,
)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]

//! A text-based adventure game

mod menu;
mod player;
mod rooms;
mod items;
mod config;
mod combat;

use combat::{battle, BattleResult};
use player::Player;

fn main() {
    let mut menu = menu::init().unwrap();
    let menu = &mut menu;

    let mut player = Player::init();
    
    let mut turn_number = 0;

    loop {
        turn_number += 1;
        player.print_room(menu);

        if let Some(enemy) = player.get_room_state_mut().enemy.take() {
            let battle_result = battle(&mut player, enemy, &mut turn_number, menu);

            if battle_result == BattleResult::PlayerLoss {
                return;
            }
        }

        player.take_passive_action(menu);
    }
    
    
}
