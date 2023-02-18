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

mod combat;
mod config;
mod items;
mod map;
mod menu;
mod player;
mod rooms;

use combat::{battle, BattleResult};
use player::Player;
use rooms::Room;

fn main() {
    let mut menu = menu::init().unwrap();
    let menu = &mut menu;

    // TODO: print intro

    // The outer time loop
    'time_loop: loop {
        let mut player = Player::init();

        let mut turn_number = 0;

        player.print_room(menu);

        // The inner gameplay loop
        loop {
            turn_number += 1;

            if let Some(enemy) = player.get_room_state_mut().enemy.take() {
                let battle_result = battle(&mut player, enemy, &mut turn_number, menu);

                if battle_result == BattleResult::PlayerLoss {
                    continue 'time_loop;
                }
            }

            player.take_passive_action(menu);

            if matches!(player.room, Room::Escape) {
                // TODO: print 'you win'
                break 'time_loop;
            }
        }
    }
}
