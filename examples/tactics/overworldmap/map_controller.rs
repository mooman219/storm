use tactics::overworldmap::OverworldMap;
use tactics::system::ExitCodes;
use std::io;
use storm::cgmath::Vector2;

pub enum MapControllerState {
    Movement,
    Menu,

}

pub struct MapController {
    map: OverworldMap,
    map_controller_state: MapControllerState
}

impl MapController {
    pub fn new() -> MapController {
        MapController {
            map: OverworldMap::new(),
            map_controller_state: MapControllerState::Menu
        }
    }

    pub fn new_map(&mut self) {
        self.map = OverworldMap::new();
        self.map.start_new_game();
        self.map.layout_map();
    }

    fn set_state(&mut self, new_state: MapControllerState) {
        self.map_controller_state = new_state;
    }

    pub fn update(&mut self) -> ExitCodes {
        self.map.layout_map();
       
        match self.map_controller_state {
            MapControllerState::Menu => {
                return self.menu();
            },
            MapControllerState::Movement => {
                self.movement_mode();
            }
        }
        ExitCodes::Ok
    }

    #[inline]
    pub fn menu(&mut self) -> ExitCodes{
        println!("Which mode would you like to be in");
        println!("1. Movement");
        println!("2. To Party Manager");
        println!("3. Exit");
        let mut input_raw = String::new();
        let input;
        match io::stdin().read_line(&mut input_raw) {
            Ok(_n) => {
                input = input_raw.trim();
            },
            Err(e) => {
                panic!("{} HEMIDALL ERROR with the input for movement direction selection", e);
            }
        }

        if input == "1" {
            self.set_state(MapControllerState::Movement);
            return ExitCodes::Ok;
        }
        else if input == "2" {
            return ExitCodes::ToPartyManagerController;
        }
        else if input == "3" {
            return ExitCodes::Exit;
        }
        
        ExitCodes::Exit
    }

    #[inline]
    pub fn movement_mode(&mut self) {
        let directions = OverworldMap::find_available_movement_direction_for_party(self.map.get_party_position());
        for dir in directions {
            println!("{:?}", dir);
        }
        println!("Exit");

        let mut input_raw = String::new();
        let input;
        match io::stdin().read_line(&mut input_raw) {
            Ok(_n) => {
                input = input_raw.trim();
            },
            Err(e) => {
                panic!("{} HEMIDALL ERROR with the input for movement direction selection", e);
            }
        }

        if input == "Up" {
            self.map.move_party_from_tile_to_tile(Vector2::new(0, 1), true);
        }
        else if input == "Down" {
            self.map.move_party_from_tile_to_tile(Vector2::new(0, 1), false);
        }
        else if input == "Left" {
            self.map.move_party_from_tile_to_tile(Vector2::new(1, 0), true);
        }
        else if input == "Right" {
            self.map.move_party_from_tile_to_tile(Vector2::new(1, 0), false);
        }
        else if input == "Exit" {
            self.set_state(MapControllerState::Menu);
        }
    }

    //there should be a load map function in the future
}