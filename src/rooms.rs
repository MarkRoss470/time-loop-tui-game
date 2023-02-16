use std::collections::HashMap;

use crate::{items::{Item, Weapon, Food}, combat::{Enemy, Damage, Health}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Room {
    Bridge,
    UpperCorridor,
    MessHall,
    Kitchen,
}

impl Room {
    pub const fn get_name(self) -> &'static str {
        match self {
            Self::Bridge => "Bridge",
            Self::UpperCorridor => "Upper Corridor",
            Self::MessHall => "Mess Hall",
            Self::Kitchen => "Kitchen"
        }
    }

    pub const fn get_description(self) -> &'static str {
        match self {
            Self::Bridge => "The control centre of the ship. Through the front window you can see into the darkness of space.",
            Self::UpperCorridor => "A corridor connecting the bridge to the rest of the ship.",
            Self::MessHall => "Where the crew eat their meals. A holo-screen in the corner is playing a game of half-G volleyball.",
            Self::Kitchen => "An immaculately clean kitchen area. All the appliances are electric - no open flames are allowed on the ship."
        }
    }
}

#[derive(Debug)]
pub struct RoomState {
    pub room: Room,
    pub items: Vec<Item>,
    pub enemy: Option<Enemy>,
    pub connections: Vec<Room>
}

#[derive(Debug)]
pub struct RoomGraph {
    rooms: HashMap<Room, RoomState>
}

impl RoomGraph {
    pub fn get_state(&self, room: Room) -> &RoomState {
        self.rooms.get(&room).unwrap()
    }

    pub fn get_state_mut(&mut self, room: Room) -> &mut RoomState {
        self.rooms.get_mut(&room).unwrap()
    }
}

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