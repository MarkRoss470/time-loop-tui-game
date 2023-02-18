//! Contains the [`RoomAction`] type and related functionality

use crate::{menu::Screen, player::Player, items::Item, rooms::{Room, RoomTransition}};

/// An action that can be performed in a room
#[derive(Debug)]
pub enum RoomAction {
    /// Take the maps in the [`StrategyRoom`][Room::StrategyRoom]
    StrategyRoomTakeMaps,
    /// Take the key in the [`EngineRoom`][Room::EngineRoom]
    EngineRoomTakeKeys,
    /// Take off in the escape pod
    EscapePodTakeOff,
}

/// The result of a [`RoomAction`]
#[derive(Debug)]
pub struct RoomActionResult<'a> {
    /// A message to show the player
    pub message: Option<Screen<'a>>,
    /// Whether this action should be listed on future turns in this room
    pub show_again: bool,
}

impl<'a> RoomActionResult<'a> {
    /// Creates a new [`RoomActionResult`] from the given optional message and whether to show the action again 
    fn new(message: Option<Screen<'a>>, show_again: bool) -> Self {
        Self{ message, show_again }
    }
}

impl RoomAction {
    /// Gets the text which will be shown to the player when they are picking an action
    pub fn get_description(&self) -> &'static str {
        match self {
            Self::StrategyRoomTakeMaps => "Take the drive from the computer",
            Self::EngineRoomTakeKeys => "Check out the cabinet in the corner",
            Self::EscapePodTakeOff => "Take off"
        }
    }
    /// Runs the action
    /// 
    /// ### Params:
    /// * `player`: the [`Player`]'s state. This is used to e.g. add items to their inventory
    pub fn execute(&self, player: &mut Player) -> RoomActionResult {
        match self {
            Self::StrategyRoomTakeMaps => {
                player.pick_up_item(Item::Maps);

                let screen = Screen {
                    title: "You take the drive",
                    content: "You take the drive, and read its description - 'Galactic Maps 2168 Edition'",
                };
                RoomActionResult::new(Some(screen), false)
            }
            Self::EngineRoomTakeKeys => {
                let crew_area_state = player.room_graph.get_state_mut(Room::CrewArea);

                let escape_pod_index = crew_area_state
                    .connections
                    .iter()
                    .position(|t|t.prompt_text == Some("Escape Pod")) 
                    .unwrap();

                crew_area_state.connections[escape_pod_index] = RoomTransition {
                    message: "You walk up to the door, the same as any other. This time, it detects the key card in your pocket and slides open. \
It clearly hasn't opened in scores and makes a grating sound. You would worry if there were anyone left alive.",
                    prompt_text: None,
                    to: Room::EscapePod
                };
                
                player.pick_up_item(Item::EscapePodKeys);

                let screen = Screen {
                    title: "You look through the drawers",
                    content: "You search every drawer. You don't find anything interesting until you get to the second-last one, which has a key card in it. You flip it over and it is labelled 'escape pod'.",
                };
                RoomActionResult::new(Some(screen), false)
            }
            Self::EscapePodTakeOff => {
                let has_maps = player.inventory.iter().any(|item|matches!(&item, Item::Maps));

                if !has_maps {
                    let screen = Screen {
                        title: "You try to launch, but there's an error.",
                        content: "\"Maps out of date: pod cannot launch without in-date maps\". You try to override the message but you can't figure it out."
                    };
                    return RoomActionResult::new(Some(screen), true);
                }

                let screen = Screen {
                    title: "You plug in the maps and blast off",
                    content: "It's a bit anticlimactic at first but then the thrusters kick in and you feel yourself shuddering home."
                };

                player.room = Room::Escape;

                RoomActionResult::new(Some(screen), false)
            }
        }
    }
}