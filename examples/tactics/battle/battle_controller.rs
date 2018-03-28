use tactics::battle::Battlefield;
use tactics::party_manager::Character;
use std::collections::HashMap;
use tactics::system::ExitCodes;
use tactics::battle::battle_commands::MovementMode;
use std::io;


//using this as a way to tracking who is currently up
enum CurrentTeam {
    None,
    Player,
    AI
}

enum BattleControllerState {
    MainState,
    MovementState    
}

pub struct BattleController {
    battlefield: Battlefield,//the data and functionality of the current battlefield
    player_characters: HashMap<String, Character>,//all character the player are using
    ai_characters: HashMap<String, Character>,//the AI is suing
    current_team: CurrentTeam,//who is going
    movement_mode: MovementMode,//Controller for move actions
    battle_controller_state: BattleControllerState
}

impl BattleController {
    
    pub fn new() -> BattleController {
        BattleController {
            battlefield: Battlefield::new(),
            player_characters: HashMap::new(), 
            ai_characters: HashMap::new(),
            current_team: CurrentTeam::Player,
            movement_mode: MovementMode::new(),
            battle_controller_state: BattleControllerState::MainState
        }
    }

    //this handles all nessacary code for setting up a battle
    //so
    //creating the battlefield itself
    //placing all the characters from both teams
    //genreating rewards
    //setting battle into initial state
    pub fn new_battle(&mut self, x_size: usize, y_size: usize, player_characters: HashMap<String, Character>) { 
        self.battlefield.initalize_new_battlefield(x_size, y_size);
        self.player_characters = player_characters;

        let mut count = 0usize;
        for (k, _) in &self.player_characters {
            println!("{}", k.clone());
            self.battlefield.place_character(k.clone(), 0, count);
            count += 1;
        }

        self.current_team = CurrentTeam::Player;
    }

    //this gets called each frame that the system is in battlefield state
    pub fn update(&mut self) -> ExitCodes {
        match self.battle_controller_state {
            BattleControllerState::MainState => {
                self.battlefield.draw();
                self.ask_for_player_input();
            },
            BattleControllerState::MovementState => {
                self.movement_mode.execute_movement_mode(&mut self.battlefield);
            }
        }


        panic!("AHHHHH, NOT THE BEES");
        ExitCodes::Ok
    }

    //use this function to handle all player input at the battle_controller level
    pub fn ask_for_player_input(&mut self) {
        println!("Which Action would you like ot preform");
        println!("1. Movement");

        let mut input_raw = String::new();
        let input;
        match io::stdin().read_line(&mut input_raw) {
            Ok(_n) => {
                input = input_raw.trim();
                if input == "1" {
                    let result = MovementMode::set_up_movement_mode(self.get_current_team());
                    match result {
                        Some(character_name) => {
                            self.movement_mode.set_character_name(character_name);
                        },
                        None => {
                            
                        }
                    }
                }
            },
            Err(e) => {
                panic!("{} HEMIDALL ERROR with player input in the battlefield", e);
            }
        }
    }

    //this is a way pf having a single call to get the current team
    //most code that needs the current team should use this
    //avoids having disambiguation code all over the place
    //and having some referance varible that we switch between all the time
    fn get_current_team(&self) -> &HashMap<String, Character>{
        match self.current_team {
            CurrentTeam::Player => {
                &self.player_characters
            },
            CurrentTeam::AI => {
                &self.ai_characters
            },
            CurrentTeam::None => {
                panic!("THIS IS NOT SUPPOSED TO BE CALLED, LOKI ERROR");
 //               &self.player_characters///THIS IS NEVER EVER SUPPOSED TO BE CALLED
            }
        }
    }
}