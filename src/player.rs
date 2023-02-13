use crate::{rooms::{Room, RoomGraph, init_rooms}, menu::{Menu, Screen, OptionList}};

pub struct Player {
    //inventory: Vec<Item>,

    room: Room,
    pub room_graph: RoomGraph,
}

impl Player {
    pub fn print_room(&self, menu: &mut impl Menu) {
        let screen = Screen {
            title: &format!("You are in the {}.", self.room.get_name()),
            content: self.room.get_description(),
        };
        
        menu.show_screen(screen);
    }

    pub fn take_action(&mut self, menu: &mut impl Menu) {

        enum Option {
            GoToRoom(Room),
            //UseItem(Item)
        }

        // Prepare empty list of options and their string representations
        let mut options_str = vec![];
        let mut options = vec![];

        let room_state = self.room_graph.get_state(self.room);
        
        for connection in &room_state.connections {
            options_str.push(format!("Go to the {}", connection.get_name()));
            options.push(Option::GoToRoom(*connection));
        }

        let option_list = OptionList::new(&options_str, "What do you do?");

        let choice = menu.show_option_list(option_list);

        match options[choice] {
            Option::GoToRoom(r) => self.room = r,
        }

    }
}

pub fn init_player() -> Player {
    Player {
        room: Room::Bridge,
        room_graph: init_rooms(),
    }
}

