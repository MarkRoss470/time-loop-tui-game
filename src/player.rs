//! Functionality related to the [`Player`]'s state and actions

mod tests;

use crate::combat::{self, Health};
use crate::config::{self, STARTING_ROOM};
use crate::items::Item;
use crate::map;
use crate::menu::{Menu, OptionList, Screen};
use crate::rooms::{Room, RoomGraph, RoomState, RoomTransition};

/// The state of the player
#[derive(Debug)]
pub struct Player {
    /// Which [`Room`] the [`Player`] is in
    pub room: Room,
    /// The [`Player`]'s inventory
    pub inventory: Vec<Item>,
    /// The [`Player`]'s current health
    pub health: Health,
    /// The maximum health the [`Player`] can reach
    pub max_health: Health,
    /// The number of turns the user has left before the loop resets
    pub remaining_turns: usize,

    /// The current state of the rooms
    pub room_graph: RoomGraph,
}

/// An action the [`Player`] can take outside of a battle
#[derive(Debug)]
enum PassiveAction<'a> {
    /// Print the [`Player`]'s health
    CheckState,
    /// Go to a [`Room`] which is connected to the current one
    GoToRoom(&'a RoomTransition),
    /// Use the [`Item`] at the given index into the [player's inventory][Player::inventory]
    UseItem(usize),
    /// Add the [`Item`] at the given index into the [current room's inventory][RoomState::items] to the [player's inventory][Player::inventory]
    PickUpItem(usize),
    /// Carry out the [`RoomAction`][crate::map::RoomAction] at the given index into the [current room's actions][RoomState::actions]
    RoomAction(usize),
}

/// Prints a screen with the details of a [`RoomTransition`] and the player's new [`Room`]
fn print_room_transition(transition: &RoomTransition, menu: &mut impl Menu) {
    let screen = Screen {
        title: &format!("You go to the {}", transition.prompt_text.unwrap_or_else(||transition.to.get_name())),
        content: &format!(
            "{}\nYou are now in the {} - {}",
            transition.message,
            transition.to.get_name(),
            transition.to.get_description()
        ),
    };

    menu.show_screen(screen);
}

impl Player {
    /// Gets a shared reference to the current [`RoomState`]
    pub fn get_room_state(&self) -> &RoomState {
        self.room_graph.get_state(self.room)
    }

    /// Gets a mutable reference to the current [`RoomState`]
    pub fn get_room_state_mut(&mut self) -> &mut RoomState {
        self.room_graph.get_state_mut(self.room)
    }

    /// Prints a screen describing the current [`RoomState`]
    pub fn print_room(&self, menu: &mut impl Menu) {
        let screen = Screen {
            title: &format!("You are in the {}.", self.room.get_name()),
            content: self.room.get_description(),
        };

        menu.show_screen(screen);
    }

    /// Gets a [`String`] representing the number of turns left.
    /// 1 turn = 20 sec
    fn get_remaining_time(&self) -> String {
        let mins = self.remaining_turns / 3;
        let secs = self.remaining_turns % 3 * 20;
        format!("{mins}:{secs:0<2}")
    }

    /// Asks the user what [`PassiveAction`] to perform given the [`Player`]'s inventory and the current [`RoomState`]
    fn choose_passive_action(&self, menu: &mut impl Menu) -> PassiveAction {
        // Init lists of options and their string representations
        let mut options = vec![PassiveAction::CheckState];
        let mut options_str = vec!["Check how you're doing".to_string()];

        let room_state = self.get_room_state();

        for connection in &room_state.connections {
            options.push(PassiveAction::GoToRoom(connection));
            options_str.push(format!(
                "Go to the {}",
                connection.prompt_text.unwrap_or_else(||connection.to.get_name())
            ));
        }

        for (i, item) in room_state.items.iter().enumerate() {
            options.push(PassiveAction::PickUpItem(i));
            options_str.push(format!(
                "Pick up the {} - {}",
                item.get_name(),
                item.get_description()
            ));
        }

        for (i, action) in room_state.actions.iter().enumerate() {
            options.push(PassiveAction::RoomAction(i));
            options_str.push(action.get_description().to_string());
        }

        for (i, item) in self.inventory.iter().enumerate() {
            match item {
                Item::Food(f) => {
                    options.push(PassiveAction::UseItem(i));
                    options_str.push(format!("Eat your {}", f.name));
                }
                Item::CaptainsDiary(_) => {
                    options.push(PassiveAction::UseItem(i));
                    options_str.push("Read the captain's diary".to_string());
                }
                _ => ()
            }
        }

        let prompt = format!("{} - What do you do?", self.get_remaining_time());
        let option_list = OptionList::new(&options_str, &prompt);

        let choice = menu.show_option_list(option_list);

        options.swap_remove(choice)
    }

    /// Gets a [`PassiveAction`] from the user and carries it out
    pub fn take_passive_action(&mut self, menu: &mut impl Menu) {
        self.remaining_turns -= 1;
        
        let action = self.choose_passive_action(menu);

        match action {
            PassiveAction::CheckState => self.print_state(menu),
            PassiveAction::GoToRoom(r) => {
                print_room_transition(r, menu);
                self.room = r.to;
            }
            PassiveAction::UseItem(i) => self.use_item(menu, i),
            PassiveAction::PickUpItem(i) => self.pick_up_item_from_room(i),
            PassiveAction::RoomAction(i) => {
                let action = self.get_room_state_mut().actions.remove(i); // Take action out of vec to avoid multiple mutable references
                let result = action.execute(self);

                if let Some(message) = result.message {
                    menu.show_screen(message);
                }

                if result.show_again {
                    self.get_room_state_mut().actions.insert(i, action); // Put action back if needed
                }
            }
        }
    }

    /// Prints the [`Player`]'s room and health
    fn print_state(&self, menu: &mut impl Menu) {
        let screen = Screen {
            title: "You take a moment to rest and check your body for injuries",
            content: &format!(
                "You are in the {} - {}\nYou are at {}/{} HP\nYou have:\n{}• {} to get off the ship\n",
                self.room.get_name(),
                self.room.get_description(),
                self.health,
                self.max_health,
                self.inventory
                    .iter()
                    .map(|item| format!("• {} - {}\n", item.get_name(), item.get_description()))
                    .collect::<String>(),
                self.get_remaining_time()
            ),
        };

        menu.show_screen(screen);
    }

    /// Uses the [`Item`] at the given index into the [`Player`]'s inventory
    fn use_item(&mut self, menu: &mut impl Menu, i: usize) {
        match &mut self.inventory[i] {
            Item::Food(f) => {
                let prev_health = self.health;
                self.health.heal_to_max(f.heals_for, self.max_health);

                let screen = Screen {
                    title: &format!("You ate your {}", f.name),
                    content: &format!(
                        "You are healed by {} HP.\nYou are now at {}/{} HP.",
                        self.health - prev_health,
                        self.health,
                        self.max_health
                    ),
                };

                menu.show_screen(screen);

                self.inventory.remove(i);
            }
            Item::CaptainsDiary(ref mut page) => {
                let screen;
                (screen, *page) = match *page {
                    0 => (Screen {
                        title: "You read the last page: 15/08/2168 - Found someone in the cold",
                        content: "Found a body in the cold. Ship was all busted up so we thought we'd be holding a funeral but turns out they had a pulse. \
They've been out for a few hours now, but still breathing. They look harmless but they're wearing an Arnithian military uniform so we're treating them like a prisoner."
                    }, 1),
                    
                    1 => (Screen {
                        title: "You start at the beginning: 01/01/2168 - Another revolution, another revelation",
                        content: "It occurred to me during the festivities that I didn't know why a revolution was called that. Turns out it's based on roughly the time Earth took to complete one orbit of its star. \
Early humans called it a 'year' and it was an important unit of time due to the seasonal nature of their planet."
                    }, 2),

                    2 => (Screen {
                        title: "03/02/2168 - Big setback today, the Arnithians seem to have ultra-powerful fighter jet engines",
                        content: "They can go from 0-60 in half a second - the engineers reckon that needs 200kN of thrust but the engines don't look much different. Can't imagine what that would do to a pilot's neck"
                    }, 3),

                    3 => (Screen {
                        title: "17/02/2168 - More bad news about the new jets",
                        content: "They seem to have some automated dodging capabilities. We can't hit them with anything. Their comms seems to be on all different frequencies and the patterns are all over the place - much faster than normal."
                    }, 4),

                    4 => (Screen {
                        title: "05/04/2168 - Home is in sight",
                        content: "The commander says I can have a few scores off after we pick up the new recruits. Sounds good to me."
                    }, 5),

                    5 => (Screen {
                        title: "01/06/2168 - Something about the new jets seems off",
                        content: "They're messing up all our instruments - according to my scanner, the pilot of one of them was 90 degrees! I'm really not sure what's going on - maybe they're all remote and it's to mess with our heads.\
In any case, whatever they're doing is working. We need to step up our game before they start deploying these everywhere."
                    }, 6),

                    6 => (Screen {
                        title: "20/06/2168 - Going home sooner than I thought",
                        content: "The commander moved forward picking up the recruits because we need more officers on the front lines. I'll be leaving in a few cycles, but it'll only be three of us"
                    }, 7),

                    7 => (Screen {
                        title: "10/07/2168 - Had to confiscate the darts",
                        content: "Juuran keeps throwing them at the windows. I know they're double reinforced but from my point of view there's no point in testing that. Besides, it makes a damn annoying noise. \
I've not hidden them but Juuran knows there'll be trouble if they take them."
                    }, 8),

                    p => (Screen {
                        title: "There's no more pages",
                        content: "How disappointing."
                    }, p)
                };

                menu.show_screen(screen);
            }
            _ => panic!("Only food items can be used outside of combat")
        }
    }

    /// Removes an [`Item`] from the current [`RoomState`] at the specified index and adds it to the [player's inventory][Player::inventory]
    fn pick_up_item_from_room(&mut self, i: usize) {
        let room_state = self.get_room_state_mut();
        let item = room_state.items.remove(i);
        self.pick_up_item(item);
    }

    /// Add an item to the [player's inventory][Player::inventory]
    pub fn pick_up_item(&mut self, item: Item) {
        // TODO: max inventory size
        self.inventory.push(item);
    }

    /// Get the user to choose a [combat action][combat::Action] to perform
    pub fn choose_combat_action(&self, menu: &mut impl Menu) -> combat::Action {
        // Init lists of options and their string representations
        let mut options = vec![
            combat::Action::Nothing,
            combat::Action::DodgeLeft,
            combat::Action::DodgeRight,
        ];
        let mut options_str = vec![
            "Do nothing".to_string(),
            "Dodge to the left".to_string(),
            "Dodge to the right".to_string(),
        ];

        // Add actions for items
        for (i, item) in self.inventory.iter().enumerate() {
            match item {
                Item::Food(f) => {
                    options.push(combat::Action::EatFood(i));
                    options_str.push(format!("Eat your {}", f.name));
                }
                Item::Weapon(w) => {
                    options.push(combat::Action::AttackStraight(i));
                    options_str.push(format!("Attack with your {}", w.name));
                }
                _ => (),
            }
        }

        // Get the user to pick an option
        let prompt = format!("{} - What do you do?", self.get_remaining_time());
        let list = OptionList::new(&options_str, &prompt);
        let choice = menu.show_option_list(list);

        // If the action was an attack, get the user to pick which direction to aim it
        if let combat::Action::AttackStraight(i) = options[choice] {
            let options = &[
                "Attack Left".to_string(),
                "Attack Straight".to_string(),
                "Attack Right".to_string(),
            ];
            let list = OptionList::new(options, "Which way do you attack?");

            let direction = menu.show_option_list(list);

            match direction {
                0 => combat::Action::AttackLeft(i),
                1 => combat::Action::AttackStraight(i),
                2 => combat::Action::AttackRight(i),
                _ => unreachable!(),
            }
        } else {
            options.swap_remove(choice)
        }
    }

    /// Get a [`String`] describing the [`Player`] performing a [combat action][combat::Action]
    pub fn describe_combat_action(&self, action: combat::Action) -> String {
        use combat::Action::*;

        match action {
            AttackLeft(w) => format!(
                "You attack to the left with your {}",
                self.inventory[w].get_name()
            ),
            AttackRight(w) => format!(
                "You attack to the right with your {}",
                self.inventory[w].get_name()
            ),
            AttackStraight(w) => format!(
                "You attack in front of you with your {}",
                self.inventory[w].get_name()
            ),
            EatFood(f) => format!("You attempt to eat your {}", self.inventory[f].get_name()),

            DodgeLeft => "You dodge to the left".to_string(),
            DodgeRight => "You dodge to the right".to_string(),
            Nothing => "You do nothing".to_string(),
        }
    }

    /// Shows the player a win screen
    pub fn show_win_screen(&self, menu: &mut impl Menu) {
        if self.inventory.iter().any(|item|matches!(item, Item::Food(_))) {
            menu.show_screen(Screen {
                title: "Freedom at long last",
                content: "Or maybe not so long - it's only been a few minutes, after all. You buckle in for the long ride and allow yourself to relax, finally. You won't get back to New Arnith for a cycle and a half, but at least you brought some food."
            });
        } else {
            menu.show_screen(Screen {
                title: "Freedom at long last",
                content: "Or maybe not so long - it's only been a few minutes, after all. You buckle in for the long ride and allow yourself to relax, finally."
            });
        }
    }
}

impl Player {
    /// Initialise a new [`Player`]
    pub fn init() -> Self {
        Self {
            room: STARTING_ROOM,
            inventory: Vec::new(),
            health: config::PLAYER_START_HEALTH,
            max_health: config::PLAYER_START_MAX_HEALTH,
            remaining_turns: config::MAX_TURNS,

            room_graph: map::init(),
        }
    }
}
