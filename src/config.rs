//! Configuration constants for the game

use crate::{combat::Health, rooms::Room};

/// How much health the player should start with
pub const PLAYER_START_HEALTH: Health = Health::new(10);
/// What the player's max health should be at the start of the game
pub const PLAYER_START_MAX_HEALTH: Health = Health::new(10);
/// Which room the player should start in
pub const STARTING_ROOM: Room = Room::Cells;

/// The maximum number of turns the player can take before the loop resets
pub const MAX_TURNS: usize = 30;
