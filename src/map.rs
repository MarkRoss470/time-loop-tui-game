//! Functions for initialising the map for each time loop

use std::collections::HashMap;

mod weapons;
mod food;

use crate::{rooms::{RoomGraph, Room, RoomState}};

/// Initialise a new [`RoomGraph`]
pub fn init() -> RoomGraph {
    let bridge = RoomState {
        room: Room::Bridge,
        items: vec![weapons::captains_blaster()],
        enemy: None,
        connections: vec![Room::UpperCorridor],
    };

    let upper_corridor = RoomState {
        room: Room::UpperCorridor,
        items: vec![],
        enemy: None,
        connections: vec![Room::Bridge, Room::MessHall],
    };

    let cells = RoomState {
        room: Room::Cells,
        items: vec![],
        enemy: None,
        connections: vec![Room::UpperCorridor],
    };

    let mess_hall = RoomState {
        room: Room::MessHall,
        items: vec![],
        enemy: None,
        connections: vec![Room::UpperCorridor, Room::Kitchen, Room::Stairwell]
    };

    let kitchen = RoomState {
        room: Room::Kitchen,
        items: vec![food::bread_roll()],
        enemy: None,
        connections: vec![Room::MessHall]
    };

    let stairwell = RoomState {
        room: Room::Stairwell,
        items: vec![],
        enemy: None,
        connections: vec![Room::MessHall, Room::CrewArea],
    };



    let crew_area = RoomState {
        room: Room::CrewArea,
        items: vec![],
        enemy: None,
        connections: vec![Room::Stairwell, Room::StoreRoom, Room::LowerCorridor],
    };

    let store_room = RoomState {
        room: Room::StoreRoom,
        items: vec![],
        enemy: None,
        connections: vec![Room::CrewArea],
    };

    let lower_corridor = RoomState {
        room: Room::LowerCorridor,
        items: vec![],
        enemy: None,
        connections: vec![Room::CrewArea, Room::WashRoom, Room::Bunks, Room::EngineRoom],
    };

    let bunks = RoomState {
        room: Room::Bunks,
        items: vec![],
        enemy: None,
        connections: vec![Room::LowerCorridor],
    };

    let wash_room = RoomState {
        room: Room::WashRoom,
        items: vec![],
        enemy: None,
        connections: vec![Room::LowerCorridor],
    };

    let engine_room = RoomState {
        room: Room::EngineRoom,
        items: vec![],
        enemy: None,
        connections: vec![Room::LowerCorridor],
    };

    RoomGraph {
        rooms: HashMap::from([
            (Room::Bridge, bridge),
            (Room::Cells, cells),
            (Room::UpperCorridor, upper_corridor),
            (Room::MessHall, mess_hall),
            (Room::Kitchen, kitchen),
            (Room::Stairwell, stairwell),

            (Room::CrewArea, crew_area),
            (Room::StoreRoom, store_room),
            (Room::LowerCorridor, lower_corridor),
            (Room::Bunks, bunks),
            (Room::WashRoom, wash_room),
            (Room::EngineRoom, engine_room),
        ])
    }
}