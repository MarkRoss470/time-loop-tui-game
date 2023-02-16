//! Functionality related to rooms

use std::collections::HashMap;

use crate::{items::{Item, Weapon, Food}, combat::{Enemy, Damage, Health}};

/// One of the game's rooms.
/// This does not store the room's state, and is only an identifier.
/// For the state of a room, use [`RoomState`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Room {
    /// The bridge
    Bridge,
    /// The corridor on the upper floor
    UpperCorridor,
    /// The mess hall
    MessHall,
    /// The kitchen
    Kitchen,
}

impl Room {
    /// Get the name of a room
    pub const fn get_name(self) -> &'static str {
        match self {
            Self::Bridge => "Bridge",
            Self::UpperCorridor => "Upper Corridor",
            Self::MessHall => "Mess Hall",
            Self::Kitchen => "Kitchen"
        }
    }

    /// Get a short description of a room
    pub const fn get_description(self) -> &'static str {
        match self {
            Self::Bridge => "The control centre of the ship. Through the front window you can see into the darkness of space.",
            Self::UpperCorridor => "A corridor connecting the bridge to the rest of the ship.",
            Self::MessHall => "Where the crew eat their meals. A holo-screen in the corner is playing a game of half-G volleyball.",
            Self::Kitchen => "An immaculately clean kitchen area. All the appliances are electric - no open flames are allowed on the ship."
        }
    }
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
    pub connections: Vec<Room>
}

/// The state of all rooms
#[derive(Debug)]
pub struct RoomGraph {
    /// A map from a [`Room`] to a [`RoomState`]
    rooms: HashMap<Room, RoomState>
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

/// Initialise a new [`RoomGraph`]
pub fn init() -> RoomGraph {
    let bridge = RoomState {
        room: Room::Bridge,
        items: vec![Item::Weapon(Weapon {
            name: "Captain's blaster",
            description: "An energy weapon which the captain keeps by his command chair in case of emergency",

            straight_damage: Damage::new(5),
            dodge_damage: Damage::new(5),
            speed: 2
        })],
        enemy: None,
        connections: vec![Room::UpperCorridor],
    };

    let upper_corridor = RoomState {
        room: Room::UpperCorridor,
        items: Vec::new(),
        enemy: None,
        connections: vec![Room::Bridge, Room::MessHall],
    };

    let mess_hall = RoomState {
        room: Room::MessHall,
        items: Vec::new(),
        enemy: Some(Enemy {
            name: "Placeholder enemy",
            description: "I just want to see if the logic works",
            inventory: vec![Item::Weapon(Weapon {
                name: "Placeholder weapon",
                description: "Just testing",

                straight_damage: Damage::new(10),
                dodge_damage: Damage::new(5),
                speed: 5
            })],

            health: Health::new(10),
            max_health: Health::new(10)
        }),
        connections: vec![Room::UpperCorridor, Room::Kitchen]
    };

    let kitchen = RoomState {
        room: Room::Kitchen,
        items: vec![Item::Food(Food {
            name: "Bread roll",
            description: "A soft white bread roll. It's tasty, but not substantial.",
            heals_for: Damage::new(5)
        })],
        enemy: None,
        connections: vec![Room::MessHall]
    };

    RoomGraph {
        rooms: HashMap::from([
            (Room::Bridge, bridge),
            (Room::UpperCorridor, upper_corridor),
            (Room::MessHall, mess_hall),
            (Room::Kitchen, kitchen),
        ])
    }
}