//! Functionality related to rooms

use std::collections::HashMap;

use crate::{combat::Enemy, items::Item};

/// One of the game's rooms.
/// This does not store the room's state, and is only an identifier.
/// For the state of a room, use [`RoomState`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Room {
    /// The bridge
    Bridge,
    /// The corridor on the upper floor
    UpperCorridor,
    /// The strategy room
    StrategyRoom,
    /// The cells where the player starts
    Cells,
    /// The mess hall
    MessHall,
    /// The kitchen
    Kitchen,
    /// The stairwell connecting the two floors
    Stairwell,

    /// The crew area
    CrewArea,
    /// The store room
    StoreRoom,
    /// The corridor on the lower floor
    LowerCorridor,
    /// The wash room
    WashRoom,
    /// The crew's bunks
    Bunks,
    /// The engine room
    EngineRoom,

    /// The escape craft
    EscapeCraft,
    /// The room which triggers winning the game
    Escape,
}

impl Room {
    /// Get the name of a room
    pub const fn get_name(self) -> &'static str {
        match self {
            Self::Bridge => "Bridge",
            Self::UpperCorridor => "Upper Corridor",
            Self::StrategyRoom => "Strategy Room",
            Self::Cells => "Cells",
            Self::MessHall => "Mess Hall",
            Self::Kitchen => "Kitchen",
            Self::Stairwell => "Stairwell",

            Self::CrewArea => "Crew Area",
            Self::StoreRoom => "Store Room",
            Self::LowerCorridor => "Lower Corridor",
            Self::WashRoom => "Wash Room",
            Self::Bunks => "Bunks",
            Self::EngineRoom => "Engine Room",

            Self::EscapeCraft => "Escape Craft",
            Self::Escape => "",
        }
    }

    /// Get a short description of a room
    pub const fn get_description(self) -> &'static str {
        match self {
            Self::Bridge => "The control centre of the ship. Through the front window you can see into the darkness of space.",
            Self::UpperCorridor => "A corridor connecting the bridge to the rest of the ship.",
            Self::StrategyRoom => "Where important tactical decisions are made. Before you arrived, the most important decision since since leaving the front lines had been what galactic time zone to use.",
            Self::Cells => "Where they keep prisoners such as yourself. The ship is on a skeleton crew on its way to pick up troops and the security isn't up to scratch, so you managed to force open the door.",
            Self::MessHall => "Where the crew eat their meals. A holo-screen in the corner is playing a game of half-G volleyball.",
            Self::Kitchen => "An immaculately clean kitchen area. All the appliances are electric - no open flames are allowed on the ship.",
            Self::Stairwell => "A stairwell. There's not much to do, but out the window you can see the ship's engines pushing you forward into your captors' grip.",
            
            Self::CrewArea => "Where the soldiers relax after a long cycle. If there were any, that is. There's a darts board on the wall, but no darts anywhere.",
            Self::StoreRoom => "A small room with many shelves containing various things. The light is broken so you can only make out shapes close to the door.",
            Self::LowerCorridor => "A corridor connecting the crew area to the engine room.",
            Self::WashRoom => "A spotless wash room containing a few showers and a few toilets. This is a military vessel, so there's no need for privacy.",
            Self::Bunks => "The soldiers will sleep here when they are on board",
            Self::EngineRoom => "Where the ship's internals are serviced from. The actual engines are at the back of the ship, but this is where the boiler and the electrical breakers are.",

            Self::EscapeCraft => "A pod big enough for only two people. It has enough fuel to get you to safety, but only just.",
            Self::Escape => "",
        }
    }
}

/// A transition between two [`Room`]s
#[derive(Debug)]
pub struct RoomTransition {
    /// A message to display when moving
    pub message: &'static str,
    /// Which [`Room`] to go to
    pub to: Room,
    /// What option to show the player. If [`None`], it will default to the name of [Self::to]
    pub prompt_text: Option<&'static str>,
}

/// The state of a room
#[derive(Debug)]
pub struct RoomState {
    /// Which room this is the state of
    pub room: Room,
    /// What items are in the room for the [`Player`][crate::player::Player] to pick up
    pub items: Vec<Item>,
    /// An [`Enemy`], if there is one
    pub enemy: Option<Enemy>,
    /// Which other rooms the player can go to from this one
    pub connections: Vec<RoomTransition>,
}

/// The state of all rooms
#[derive(Debug)]
pub struct RoomGraph {
    /// A map from a [`Room`] to a [`RoomState`]
    pub rooms: HashMap<Room, RoomState>,
}

impl RoomGraph {
    /// Get a shared reference to the [`RoomState`] for a given [`Room`]
    pub fn get_state(&self, room: Room) -> &RoomState {
        self.rooms.get(&room).unwrap()
    }

    /// Get a mutable reference to the [`RoomState`] for a given [`Room`]
    pub fn get_state_mut(&mut self, room: Room) -> &mut RoomState {
        self.rooms.get_mut(&room).unwrap()
    }
}
