//! Configuration constants for the game

use crate::combat::Health;

/// How much health the player should start with
pub const PLAYER_START_HEALTH: Health = Health::new(10);
/// What the player's max health should be at the start of the game
pub const PLAYER_START_MAX_HEALTH: Health = Health::new(10);