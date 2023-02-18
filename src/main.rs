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
use config::MAX_TURNS;
use menu::{Screen, Menu};
use player::Player;
use rooms::Room;

/// The screen to show at the beginning of the game
const INTRO_SCREEN: Screen = Screen {
    title: "Welcome Soldier",
    content: 
"The year is 2168. You are a fighter pilot for the Arnithian Galactic Republic. You were sent out in your experimental time-bending t-Jet to protect a cargo vessel, but an engine malfunction left you irradiated and stranded in space. \
You wake up in a cell, confused and disoriented. You peer as far out of your cell as you can. There's someone in the room opposite you, but they're not looking at you. \
You try the cell door. It's locked, obviously, but not the control panel looks insecure. You pull off the screen and jump two wires inside. The door slides open. \
You keep your eyes on the person in the other room, but they don't seem to notice. Suddenly they look up and see you standing there. They rush out and before you know it you're bleeding out on the floor, and then
You wake up in a cell, confused and disoriented. You peer as far out of your cell as you can. There's someone in the room opposite you, but they're not looking at you. \
You hot-wire the door, but then you're more cautious. You duck down below the level of the door and prepare for your final moments... again.",
};

/// The screen to show when the time loop resets
const LOOP_SCREEN: Screen = Screen {
    title: "You wake up in a cell",
    content: "Well, here we go again... You break open the door and hope you don't get shot this time."
};

/// The screen to show when the player reaches their max turns
const MAX_TURNS_SCREEN: Screen = Screen {
    title: "\"Now boarding: ISPD agents\"",
    content: "You groan. There's no way you're getting out of this alive. "
};

fn main() {
    let mut menu = menu::init().unwrap();
    let menu = &mut menu;

    menu.show_screen(INTRO_SCREEN);

    // The outer time loop
    'time_loop: loop {
        let mut player = Player::init();

        let mut turn_number = 0;

        player.print_room(menu);

        // The inner gameplay loop
        loop {

            if turn_number >= MAX_TURNS {
                menu.show_screen(MAX_TURNS_SCREEN);
                continue 'time_loop;
            }

            turn_number += 1;

            if let Some(enemy) = player.get_room_state_mut().enemy.take() {
                let battle_result = battle(&mut player, enemy, &mut turn_number, menu);

                match battle_result {
                    BattleResult::PlayerWin => (),
                    BattleResult::PlayerLoss => {
                        menu.show_screen(LOOP_SCREEN);
                        continue 'time_loop;
                    },
                    BattleResult::MaxTurnsReached => {
                        menu.show_screen(MAX_TURNS_SCREEN);
                        continue 'time_loop;
                    }
                }
            }

            player.take_passive_action(menu);

            if matches!(player.room, Room::Escape) {
                player.show_win_screen(menu);
                break 'time_loop;
            }
        }
    }
}
