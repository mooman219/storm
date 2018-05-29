use std::collections::HashMap;
use std::io;
use std::usize;

use storm::cgmath::Vector2;
use storm::input::message::*;
use storm::render::message::*;

use tactics::overworldmap::OverworldMap;
use tactics::system::ExitCodes;
use tactics::Controller;

pub enum MapControllerState {
    Movement,
    Menu,
    ExitGame,
    ExitToPartyManager,
    WaitingForInput,
}

pub struct MapController {
    map: OverworldMap,
    map_controller_state: MapControllerState,
    action_delegate: Option<fn(&mut MapController, InputFrame) -> ()>,
}

impl MapController {
    pub fn new() -> MapController {
        MapController {
            map: OverworldMap::new(),
            map_controller_state: MapControllerState::Menu,
            action_delegate: None,
        }
    }

    pub fn new_map(&mut self, render: &mut RenderProducer) {
        self.map = OverworldMap::new();
        self.map.start_new_game(render);
    }

    fn set_state(&mut self, new_state: MapControllerState) {
        self.map_controller_state = new_state;
    }

    //update is used for three main purposes
    //1. Resolve the state as much as possible, moving from special states to waiting states
    //2. Tick events that need to be ticked every frame, think like an NPC that is always moving
    //3. By the exit code query the state of the game
    pub fn update(&mut self, render: &mut RenderProducer) -> ExitCodes {
        self.map.layout_map(render);

        match self.map_controller_state {
            MapControllerState::Menu => {
                self.menu();
            },
            MapControllerState::WaitingForInput => {
                //just spin until we hear input
                return ExitCodes::Ok;
            },
            MapControllerState::Movement => {
                //display the input options, set the movement input handler, drop into WaitinForInput
                //  self.action_delegate = MapController::
                self.set_state(MapControllerState::WaitingForInput);
            },
            MapControllerState::ExitGame => {
                //if the users input was to leave, report that up the chain, system will handle it from here
                return ExitCodes::Exit;
            },
            MapControllerState::ExitToPartyManager => {
                //The game is meant to transition from
                return ExitCodes::ToPartyManagerController;
            },
        }
        ExitCodes::Ok
    }

    ///////////////////////////////
    ///     Menu functions     ///
    /////////////////////////////
    #[inline]
    //let the player what they can do, set the correct input handler, then tell the controller to wait for input
    pub fn menu(&mut self) {
        println!("Which mode would you like to be in");
        println!("1. Movement");
        println!("2. To Party Manager");
        println!("3. Exit");

        self.action_delegate = Some(MapController::menu_input_handler);

        self.set_state(MapControllerState::WaitingForInput);
    }
    pub fn menu_input_handler(&mut self, input_frame: InputFrame) {
        match input_frame {
            InputFrame::KeyPressed(key) => {
                //which key does what matches up with the numbers in the prints in the menu function
                if key == Key::Key1 {
                    self.set_state(MapControllerState::Movement);
                } else if key == Key::Key2 {
                    self.set_state(MapControllerState::ExitToPartyManager);
                } else if key == Key::Key3 {
                    self.set_state(MapControllerState::ExitGame);
                }
            },
            _ => {},
        }
        self.action_delegate = None;
    }
    ///////////////////////////////
    ///  end of Menu function  ///
    /////////////////////////////

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
            },
        }

        if input == "Up" {
            self.map.move_party_from_tile_to_tile(Vector2::new(0, 1), true);
        } else if input == "Down" {
            self.map.move_party_from_tile_to_tile(Vector2::new(0, 1), false);
        } else if input == "Left" {
            self.map.move_party_from_tile_to_tile(Vector2::new(1, 0), true);
        } else if input == "Right" {
            self.map.move_party_from_tile_to_tile(Vector2::new(1, 0), false);
        } else if input == "Exit" {
            self.set_state(MapControllerState::Menu);
        }
    }

    //there should be a load map function in the future
}

impl Controller for MapController {
    fn input_handler(&mut self, input: InputFrame) {
        if self.action_delegate.is_some() {
            //the game works toward resolvesing itself to such a such that it can then wait on input
            //it does this but setting the as_easy varible in map_controller
            //and then calling that fucntion when any input is heard
            let function_handle = self.action_delegate.unwrap();
            function_handle(self, input);
            //      self.as_easy = None;
        }
    }
}
