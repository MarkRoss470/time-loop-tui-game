//! Functions for initialising the map for each time loop

use std::collections::HashMap;

mod weapons;
mod food;
mod enemies;
mod transitions;

use crate::{rooms::{RoomGraph, Room, RoomState}};

use self::transitions::*;


/// Initialise a new [`RoomGraph`]
pub fn init() -> RoomGraph {
    let bridge = RoomState {
        room: Room::Bridge,
        items: vec![weapons::captains_blaster()],
        enemy: None,
        connections: vec![BRIDGE_TO_UPPER_CORRIDOR],
    };

    let upper_corridor = RoomState {
        room: Room::UpperCorridor,
        items: vec![],
        enemy: None,
        connections: vec![UPPER_CORRIDOR_TO_BRIDGE, UPPER_CORRIDOR_TO_STRATEGY_ROOM, UPPER_CORRIDOR_TO_CELLS, UPPER_CORRIDOR_TO_MESS_HALL],
    };

    let strategy_room = RoomState {
        room: Room::StrategyRoom,
        items: vec![],
        enemy: Some(enemies::skipper()),
        connections: vec![STRATEGY_ROOM_TO_UPPER_CORRIDOR],
    };

    let cells = RoomState {
        room: Room::Cells,
        items: vec![],
        enemy: None,
        connections: vec![CELLS_TO_UPPER_CORRIDOR],
    };

    let mess_hall = RoomState {
        room: Room::MessHall,
        items: vec![],
        enemy: Some(enemies::cook()),
        connections: vec![MESS_HALL_TO_UPPER_CORRIDOR, MESS_HALL_TO_KITCHEN, MESS_HALL_TO_STAIRWELL]
    };

    let kitchen = RoomState {
        room: Room::Kitchen,
        items: vec![food::bread_roll()],
        enemy: None,
        connections: vec![KITCHEN_TO_MESS_HALL]
    };

    let stairwell = RoomState {
        room: Room::Stairwell,
        items: vec![],
        enemy: None,
        connections: vec![STAIRWELL_TO_MESS_HALL, STAIRWELL_TO_CREW_AREA],
    };



    let crew_area = RoomState {
        room: Room::CrewArea,
        items: vec![],
        enemy: None,
        connections: vec![CREW_AREA_TO_STAIRWELL, CREW_AREA_TO_STORE_ROOM, CREW_AREA_TO_LOWER_CORRIDOR],
    };

    let store_room = RoomState {
        room: Room::StoreRoom,
        items: vec![],
        enemy: None,
        connections: vec![STORE_ROOM_TO_CREW_AREA],
    };

    let lower_corridor = RoomState {
        room: Room::LowerCorridor,
        items: vec![],
        enemy: None,
        connections: vec![LOWER_CORRIDOR_TO_CREW_AREA, LOWER_CORRIDOR_TO_BUNKS, LOWER_CORRIDOR_TO_WASH_ROOM, LOWER_CORRIDOR_TO_ENGINE_ROOM],
    };

    let bunks = RoomState {
        room: Room::Bunks,
        items: vec![weapons::throwing_dart_set()],
        enemy: None,
        connections: vec![BUNKS_TO_LOWER_CORRIDOR],
    };

    let wash_room = RoomState {
        room: Room::WashRoom,
        items: vec![],
        enemy: None,
        connections: vec![WASH_ROOM_TO_LOWER_CORRIDOR],
    };

    let engine_room = RoomState {
        room: Room::EngineRoom,
        items: vec![],
        enemy: Some(enemies::mechanic()),
        connections: vec![ENGINE_ROOM_TO_LOWER_CORRIDOR],
    };


    RoomGraph {
        rooms: HashMap::from([
            (Room::Bridge, bridge),
            (Room::UpperCorridor, upper_corridor),
            (Room::StrategyRoom, strategy_room),
            (Room::Cells, cells),
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