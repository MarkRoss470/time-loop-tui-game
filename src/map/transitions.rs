//! Room transitions

use crate::rooms::{Room, RoomTransition};

/// Reduces boilerplate when defining [`RoomTransition`]s.
/// Defines a constant with a visibility of `pub(super)` with a given name, start and destination rooms, and a description.
macro_rules! room_transition {
    ($name: ident, $from: ident, $to: ident, $message: expr) => {
        pub(super) const $name: RoomTransition = RoomTransition {
            message: $message,
            to: Room::$to,
            prompt_text: None,
        };
    };
}

room_transition!(BRIDGE_TO_UPPER_CORRIDOR, Bridge, UpperCorridor, "You walk out into the corridor and the door to the bridge slides closed behind you.");

room_transition!(UPPER_CORRIDOR_TO_BRIDGE, UpperCorridor, Bridge, "You walk up to a large metal door and it splits into three pieces and retracts into the walls and ceiling.");
room_transition!(UPPER_CORRIDOR_TO_STRATEGY_ROOM, UpperCorridor, StrategyRoom, "You peer through a window and see the skipper. They don't move. You go in.");
room_transition!(UPPER_CORRIDOR_TO_CELLS, UpperCorridor, Cells, "You return to where it all starts.");
room_transition!(UPPER_CORRIDOR_TO_MESS_HALL, UpperCorridor, MessHall, "You walk towards the door opposite the bridge. With all these identical doors, you wonder how anyone finds their way around.");

room_transition!(STRATEGY_ROOM_TO_UPPER_CORRIDOR, StrategyRoom, UpperCorridor, "You leave the strategy room, trying not to think about what happened there.");

room_transition!(CELLS_TO_UPPER_CORRIDOR, Cells, UpperCorridor, "You sneak through the busted door and hope nobody notices you.");

room_transition!(MESS_HALL_TO_UPPER_CORRIDOR, MessHall, UpperCorridor, "You walk back away from the mess hall. You'd like to watch the game, but there's no time.");
room_transition!(MESS_HALL_TO_KITCHEN, MessHall, Kitchen, "You stroll into the kitchen. You smell sweet potato soup, but you know it's synthetic. It's been at least six scores since you've had food that was actually grown on a planet.");
room_transition!(MESS_HALL_TO_STAIRWELL, MessHall, Stairwell, "You jog over to the stairwell. If there's anyone downstairs, they've surely heard you by now.");

room_transition!(KITCHEN_TO_MESS_HALL, Kitchen, MessHall, "You walk back out into the mess hall, craving real food.");

room_transition!(STAIRWELL_TO_MESS_HALL, Stairwell, MessHall, "You feel you have unfinished business upstairs, and you go back up.");
room_transition!(STAIRWELL_TO_CREW_AREA, Stairwell, CrewArea, "You cautiously approach the bottom of the stairs. You walk out into an empty room. It feels like there should be people here, but there aren't.");

room_transition!(CREW_AREA_TO_STAIRWELL, CrewArea, Stairwell, "You walk up the stairs, taking in the view as you go.");
room_transition!(CREW_AREA_TO_STORE_ROOM, CrewArea, StoreRoom, "You walk into the store room, and the light is far too dim. It's been broken for scores, but there are no replacements on board.");
room_transition!(CREW_AREA_TO_LOWER_CORRIDOR, CrewArea, LowerCorridor, "You head down another corridor and peek into the rooms on either side. It's unnerving how there's nobody here.");

room_transition!(STORE_ROOM_TO_CREW_AREA, StoreRoom, CrewArea, "You turn to go out the door, and have to squint because of the light");

room_transition!(LOWER_CORRIDOR_TO_CREW_AREA, LowerCorridor, CrewArea, "You go back to the crew area. You see the escape pod on your left and dream of being the first person ever to escape from an enemy craft");
room_transition!(LOWER_CORRIDOR_TO_WASH_ROOM, LowerCorridor, WashRoom, "As you walk into the wash room, you look at yourself in the mirror. You haven't showered in six cycles, and it shows.");
room_transition!(LOWER_CORRIDOR_TO_BUNKS, LowerCorridor, Bunks, "You walk into the empty bunks and think about how much you want to take a nap. When this is all over, you'll have the best sleep of your life.");
room_transition!(LOWER_CORRIDOR_TO_ENGINE_ROOM, LowerCorridor, EngineRoom, "The door to the engine room slides up. You see lots of wires, pipes, and tanks. That's what a spaceship is supposed to look like.");

room_transition!(BUNKS_TO_LOWER_CORRIDOR, Bunks, LowerCorridor, "You leave the bunks, fighting the urge to go back and lie down.");

room_transition!(WASH_ROOM_TO_LOWER_CORRIDOR, WashRoom, LowerCorridor, "You leave the wash room and now the rest of the ship looks positively grubby in comparison.");

room_transition!(ENGINE_ROOM_TO_LOWER_CORRIDOR, EngineRoom, LowerCorridor, "You leave the engine room and it becomes even more apparent to you just how soulless the ship is.");