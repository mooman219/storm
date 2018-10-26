use std::io;

use storm::input::message::*;
use storm::render::message::*;

use tactics::battle::BattleController;
use tactics::controller::Controller;
use tactics::overworldmap::MapController;
use tactics::party_manager::PartyManagerController;

enum SystemState {
    MainMenu,
    Battle,
    Map,
    Settings,
    PartyManager,
    Loading,
    Exit,
}

#[derive(Debug)]
pub enum ExitCodes {
    Ok, //call the same controller again
    ToMapController,
    ToPartyManagerController,
    //battlefield x size, battle y size, this will set up the battle field, and move to the battlecontroller
    ToBattle(usize, usize),
    Exit,
}

pub struct System {
    system_state: SystemState,
    map_controller: MapController,
    party_manager_controller: PartyManagerController,
    battle_controller: BattleController,
}

impl System {
    pub fn new() -> System {
        System {
            system_state: SystemState::MainMenu,
            map_controller: MapController::new(),
            party_manager_controller: PartyManagerController::new(),
            battle_controller: BattleController::new(),
        }
    }

    fn new_game(&mut self, render: &mut RenderMessenger) {
        render.set_scale(0.01f32);
        self.map_controller.new_map(render);
    }

    fn set_state(&mut self, new_state: SystemState) {
        self.system_state = new_state;
    }

    pub fn update(&mut self, render: &mut RenderMessenger) {
        loop {
            let mut exit_code = ExitCodes::Ok;
            match self.system_state {
                SystemState::MainMenu => {
                    println!("\t\tMAIN MENU\t\t");
                    println!("Input the action you want");
                    println!("1. New game");
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(_n) => {
                            let input = input.trim();
                            if input == "New game" {
                                self.new_game(render);
                                exit_code = ExitCodes::ToMapController;
                                //      exit_code = ExitCodes::ToBattle(10, 10);
                                //self.set_state(SystemState::PartyManager);
                            }
                        },
                        Err(e) => {
                            panic!("{} THOR error: the system main menu input was read in with error", e);
                        },
                    }
                },
                SystemState::Map => {
                    exit_code = self.map_controller.update(render);
                },
                SystemState::PartyManager => {
                    exit_code = self.party_manager_controller.update();
                },
                SystemState::Exit => {
                    break;
                },
                SystemState::Battle => {
                    exit_code = self.battle_controller.update();
                },
                _ => {},
            }
            self.process_exit_code(exit_code);
        }
    }

    //trying to keep the main loop unclutered
    #[inline]
    pub fn process_exit_code(&mut self, exit_code: ExitCodes) {
        match exit_code {
            ExitCodes::ToMapController => {
                self.set_state(SystemState::Map);
            },
            ExitCodes::ToPartyManagerController => {
                self.set_state(SystemState::PartyManager);
            },
            ExitCodes::Exit => {
                self.set_state(SystemState::Exit);
            },
            ExitCodes::ToBattle(x, y) => {
                self.battle_controller
                    .new_battle(x, y, self.party_manager_controller.clone_active_roster());
                self.set_state(SystemState::Battle);
            },
            ExitCodes::Ok => {
                //just removing the OK code from any pattern
            },
        }
    }

    pub fn handle_input(&mut self, input: InputFrame) {
        match self.system_state {
            SystemState::MainMenu => {
                //will get to this
            },
            SystemState::Map => {
                self.map_controller.input_handler(input);
            },
            SystemState::PartyManager => {},
            SystemState::Exit => {},
            SystemState::Battle => {},
            _ => {},
        }
    }
}
