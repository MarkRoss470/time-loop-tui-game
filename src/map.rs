//! Functions for initialising the map for each time loop

use std::collections::HashMap;

mod actions;
mod enemies;
mod food;
mod transitions;
mod weapons;

pub use actions::RoomAction;

use crate::rooms::{Room, RoomGraph, RoomState};

use self::transitions::*;

/// Initialise a new [`RoomGraph`]
pub fn init() -> RoomGraph {
    // The bridge
    let bridge = RoomState::new(Room::Bridge, vec![BRIDGE_TO_UPPER_CORRIDOR])
        .add_item(weapons::intruders_blaster())
        .add_action(RoomAction::BridgeHackTheMainframe);

    // The upper corridor
    let upper_corridor = RoomState::new(
        Room::UpperCorridor,
        vec![
            UPPER_CORRIDOR_TO_BRIDGE,
            UPPER_CORRIDOR_TO_STRATEGY_ROOM,
            UPPER_CORRIDOR_TO_CELLS,
            UPPER_CORRIDOR_TO_MESS_HALL,
        ],
    );

    // The strategy room
    let strategy_room = RoomState::new(Room::StrategyRoom, vec![STRATEGY_ROOM_TO_UPPER_CORRIDOR])
        .with_enemy(enemies::skipper())
        .add_action(RoomAction::StrategyRoomTakeMaps);

    // The cells
    let cells = RoomState::new(Room::Cells, vec![CELLS_TO_UPPER_CORRIDOR])
        .add_action(RoomAction::CellsClimbIntoVents);

    // The mess hall
    let mess_hall = RoomState::new(
        Room::MessHall,
        vec![
            MESS_HALL_TO_UPPER_CORRIDOR,
            MESS_HALL_TO_KITCHEN,
            MESS_HALL_TO_STAIRWELL,
        ],
    )
    .with_enemy(enemies::cook())
    .add_action(RoomAction::MessHallWatchTheGame);

    // The kitchen
    let kitchen = RoomState::new(Room::Kitchen, vec![KITCHEN_TO_MESS_HALL])
        .add_item(food::bread_roll())
        .add_item(weapons::eating_knife());

    // The stairwell
    let stairwell = RoomState::new(
        Room::Stairwell,
        vec![STAIRWELL_TO_MESS_HALL, STAIRWELL_TO_CREW_AREA],
    );

    // The crew area
    let crew_area = RoomState::new(
        Room::CrewArea,
        vec![
            CREW_AREA_TO_STAIRWELL,
            CREW_AREA_TO_STORE_ROOM,
            CREW_AREA_TO_ESCAPE_POD,
            CREW_AREA_TO_LOWER_CORRIDOR,
        ],
    );

    // The store room
    let store_room = RoomState::new(Room::StoreRoom, vec![STORE_ROOM_TO_CREW_AREA])
        .add_action(RoomAction::StoreRoomFindChocolate);

    // The lower corridor
    let lower_corridor = RoomState::new(
        Room::LowerCorridor,
        vec![
            LOWER_CORRIDOR_TO_CREW_AREA,
            LOWER_CORRIDOR_TO_BUNKS,
            LOWER_CORRIDOR_TO_WASH_ROOM,
            LOWER_CORRIDOR_TO_ENGINE_ROOM,
        ],
    );

    // The bunks
    let bunks = RoomState::new(Room::Bunks, vec![BUNKS_TO_LOWER_CORRIDOR])
        .add_item(weapons::throwing_dart_set())
        .add_action(RoomAction::BunksGetDiary);

    // The wash room
    let wash_room = RoomState::new(Room::WashRoom, vec![WASH_ROOM_TO_LOWER_CORRIDOR])
        .add_item(weapons::shaving_razor());

    // The engine room
    let engine_room = RoomState::new(Room::EngineRoom, vec![ENGINE_ROOM_TO_LOWER_CORRIDOR])
        .with_enemy(enemies::mechanic())
        .add_action(RoomAction::EngineRoomTakeKeys)
        .add_item(weapons::wrench());

    let escape_pod = RoomState::new(Room::EscapePod, vec![ESCAPE_POD_TO_CREW_AREA])
        .add_action(RoomAction::EscapePodTakeOff);

    // Construct a room graph from all the rooms
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
            (Room::EscapePod, escape_pod),
        ]),
    }
}
