use std::collections::HashMap;

use crate::items::{Item, Weapon, Food};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Room {
    Bridge,
    UpperCorridor,
    MessHall,
    Kitchen,
}

impl Room {
    pub fn get_name(&self) -> &'static str {
        match self {
            Room::Bridge => "Bridge",
            Room::UpperCorridor => "Upper Corridor",
            Room::MessHall => "Mess Hall",
            Room::Kitchen => "Kitchen"
        }
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            Room::Bridge => "The control centre of the ship. Through the front window you can see into the darkness of space.",
            Room::UpperCorridor => "A corridor connecting the bridge to the rest of the ship.",
            Room::MessHall => "Where the crew eat their meals. A holo-screen in the corner is playing a game of half-G volleyball.",
            Room::Kitchen => "An immaculately clean kitchen area. All the appliances are electric - no open flames are allowed on the ship."
        }
    }
}

#[derive(Debug)]
pub struct RoomState {
    pub room: Room,
    pub items: Vec<Item>,
    pub connections: Vec<Room>
}

#[derive(Debug)]
pub struct RoomGraph {
    rooms: HashMap<Room, RoomState>
}

impl RoomGraph {
    pub fn get_state<'a>(&'a self, room: Room) -> &'a RoomState {
        self.rooms.get(&room).unwrap()
    }

    pub fn get_state_mut<'a>(&'a mut self, room: Room) -> &'a mut RoomState {
        self.rooms.get_mut(&room).unwrap()
    }
}

pub fn init_rooms() -> RoomGraph {
    let bridge = RoomState {
        room: Room::Bridge,
        items: vec![Item::Weapon(Weapon {
            name: "Captain's blaster",
            description: "An energy weapon which the captain keeps by his command chair in case of emergency",
            damage: 5,
        })],
        connections: vec![Room::UpperCorridor],
    };

    let upper_corridor = RoomState {
        room: Room::UpperCorridor,
        items: Vec::new(),
        connections: vec![Room::Bridge, Room::MessHall],
    };

    let mess_hall = RoomState {
        room: Room::MessHall,
        items: Vec::new(),
        connections: vec![Room::UpperCorridor, Room::Kitchen]
    };

    let kitchen = RoomState {
        room: Room::Kitchen,
        items: vec![Item::Food(Food {
            name: "Bread roll",
            description: "A soft white bread roll. It's tasty, but not substantial.",
            heals_for: 5
        })],
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