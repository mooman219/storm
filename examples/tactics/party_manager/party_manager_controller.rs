use std::collections::HashMap;
use std::io;
use tactics::party_manager::{Caravan, Character, PartyView};
use tactics::system::ExitCodes;

enum PartyMangerControllerState {
    MenuState,
    PartyViewState,
}

pub struct PartyManagerController {
    party_manager_controller_state: PartyMangerControllerState,
    caravan: Caravan,
    party_view: PartyView,
}

impl PartyManagerController {
    pub fn new() -> PartyManagerController {
        //test character, will work on this flow more after battle is fleshed out
        let character = Character::new(String::from("Joachim Murat"), 5, 2, 2);
        let mut caravan = Caravan::new(character);

        for i in 0..10 {
            let character = Character::create_random_character();
            if !caravan.is_party_member(&character.name) {
                if i < 5 {
                    caravan.add_to_active_roster(character.name.clone());
                }
                caravan.add_party_member(character.name.clone(), character);
            }
        }

        PartyManagerController {
            party_manager_controller_state: PartyMangerControllerState::MenuState,
            caravan,
            party_view: PartyView::new(),
        }
    }

    fn set_state(&mut self, new_state: PartyMangerControllerState) {
        self.party_manager_controller_state = new_state;
    }

    pub fn update(&mut self) -> ExitCodes {
        match self.party_manager_controller_state {
            PartyMangerControllerState::MenuState => {
                println!("What do you want to in the party manager");
                println!("1. Exit to Map");
                println!("2. Exit game");
                println!("3. To Party View");

                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_n) => {
                        let input = input.trim();
                        if input == "1" {
                            return ExitCodes::ToMapController;
                        } else if input == "2" {
                            return ExitCodes::Exit;
                        } else if input == "3" {
                            self.set_state(PartyMangerControllerState::PartyViewState);
                        }
                    },
                    Err(e) => {
                        panic!("{} HEMIDALL ERROR with the input for party manager controller", e);
                    },
                }
            },
            PartyMangerControllerState::PartyViewState => {
                self.party_view();
            },
        }

        ExitCodes::Ok
    }

    #[inline]
    //this acts as the menu and controller for the party view, which is every character that the player has in their caravan
    pub fn party_view(&mut self) {
        self.party_view.draw(&self.caravan);
        println!("What do you want to do in Party View");
        println!("1. Exit to Party Manager");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let input = input.trim();
                if input == "1" {
                    self.set_state(PartyMangerControllerState::MenuState);
                }
            },
            Err(e) => {
                panic!(
                    "{} HEMIDALL ERROR with the input for party view party manager controller",
                    e
                );
            },
        }
    }

    pub fn clone_active_roster(&self) -> HashMap<String, Character> {
        self.caravan.create_clone_of_active_roster()
    }
}
