use crate::items::Item;
use crate::config;
use crate::menu::{Menu, Screen, OptionList};
use crate::rooms::{Room, RoomGraph, init_rooms};

#[derive(Debug)]
pub struct Player {
    room: Room,
    inventory: Vec<Item>,
    health: usize,
    max_heath: usize,

    pub room_graph: RoomGraph,
}

#[derive(Debug)]
enum Action {
    CheckState,
    GoToRoom(Room),
    UseItem(usize),
    PickUpItem(usize),
}

impl Player {
    pub fn print_room(&self, menu: &mut impl Menu) {
        let screen = Screen {
            title: &format!("You are in the {}.", self.room.get_name()),
            content: self.room.get_description(),
        };
        
        menu.show_screen(screen);
    }

    fn choose_action(&self, menu: &mut impl Menu) -> Action {
        // Prepare empty list of options and their string representations
        let mut options = vec![Action::CheckState];
        let mut options_str = vec!["Check how you're doing".to_string()];

        let room_state = self.room_graph.get_state(self.room);

        for connection in &room_state.connections {
            options.push(Action::GoToRoom(*connection));
            options_str.push(format!("Go to the {}", connection.get_name()));
        }

        for (i, item) in room_state.items.iter().enumerate() {
            options.push(Action::PickUpItem(i));
            options_str.push(format!("Pick up the {} - {}", item.get_name(), item.get_description()));
        }

        for (i, item) in self.inventory.iter().enumerate() {
            if let Item::Food(_) = item {
                options.push(Action::UseItem(i));
                options_str.push(format!("Eat your {}", item.get_name()));
            }
        }

        let option_list = OptionList::new(&options_str, "What do you do?");

        let choice = menu.show_option_list(option_list);

        options.swap_remove(choice)
    }

    pub fn take_action(&mut self, menu: &mut impl Menu) {
        let action = self.choose_action(menu);

        match action {
            Action::CheckState => self.print_state(menu),
            Action::GoToRoom(r) => self.room = r,
            Action::UseItem(i) => self.use_item(menu, i),
            Action::PickUpItem(i) => self.pick_up_item(i),
        }

    }

    fn print_state(&self, menu: &mut impl Menu) {
        // TODO: show weapon durability if I add that

        let screen = Screen {
            title: "You take a moment to rest and check your body for injuries",
            content: &format!("You are at {}/{} HP", self.health, self.max_heath),
        };

        menu.show_screen(screen);
    }

    fn use_item(&mut self, menu: &mut impl Menu, i: usize) {
        match &self.inventory[i] {
            Item::Food(f) => {
                let prev_health = self.health;
                self.health = self.max_heath.min(prev_health + f.heals_for);

                let screen = Screen {
                    title: &format!("You ate your {}", f.name),
                    content: &format!("You are healed by {} HP.\nYou are now at {}/{} HP.", self.health - prev_health, self.health, self.max_heath),
                };

                menu.show_screen(screen);

                self.inventory.remove(i);
            },
            Item::Weapon(_) => {
                panic!("Weapons cannot be used outside of combat")
            }
        }
    }

    fn pick_up_item(&mut self, i: usize) {
        let room_state = self.room_graph.get_state_mut(self.room);
        let item = room_state.items.remove(i);
        self.inventory.push(item);
    }
}

pub fn init_player() -> Player {
    Player {
        room: Room::Bridge,
        inventory: Vec::new(),
        health: config::PLAYER_START_HEALTH,
        max_heath: config::PLAYER_START_MAX_HEALTH,

        room_graph: init_rooms(),
    }
}

