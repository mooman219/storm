use std::collections::HashMap;
use std::io;

use storm::cgmath::Vector2;

use tactics::battle::Battlefield;
use tactics::party_manager::Character;

pub struct MovementMode {
    character_name: String,
}

impl MovementMode {
    pub fn new() -> MovementMode {
        MovementMode {
            character_name: String::from(""),
        }
    }

    //
    pub fn set_character_name(&mut self, character_name: String) {
        self.character_name = character_name;
    }

    //
    pub fn execute_movement_mode(&self, battlefield: &mut Battlefield) {
        let pos = battlefield.get_location_of_character(&self.character_name);

        if pos.is_none() {
            return;
        }
        let pos = pos.unwrap();
        let size = battlefield.get_battlefield_size();

        let mut possibles = vec![pos];

        if pos.y > 0 {
            possibles.push(Vector2::new(pos.x, pos.y - 1));
        }

        if pos.x > 0 {
            possibles.push(Vector2::new(pos.x - 1, pos.y));
        }

        if pos.x < size.x - 1 {
            possibles.push(Vector2::new(pos.x + 1, pos.y));
        }

        if pos.y < size.y - 1 {
            possibles.push(Vector2::new(pos.x, pos.y + 1));
        }

        if possibles.len() == 0 {
            println!(
                "{} has no tiles to move to, please change characters",
                self.character_name
            );
            return;
        }

        for _tiles in possibles {}
    }

    pub fn set_up_movement_mode(player_character: &HashMap<String, Character>) -> Option<String> {
        //we look at all characters that have some speed left this turn
        //show them to the player
        //they pick the one they want to move, and then ask them to
        //enter the tile we want them to move to

        let mut keep_looping = true;
        while keep_looping {
            println!("Which character would you like to move");
            let mut choices = vec![];
            let mut count = 1;

            for (k, _) in player_character {
                println!("{}. {}", count, k);
                choices.push(k);
                count += 1;
            }
            println!("Type Exit/Quit to stop making command");

            let mut input_raw = String::new();
            let input;
            match io::stdin().read_line(&mut input_raw) {
                Ok(_n) => {
                    input = input_raw.trim();
                    let result = input.parse::<usize>();
                    match result {
                        Ok(index) => {
                            //due to index being a usize, we only need to check that it is atleast 0
                            if index - 1 < choices.len() && index > 0 {
                                return Some(choices[index - 1].clone());
                            //                                println!("{}", choices[index - 1]);
                            } else {
                                println!("{} is not within the correct values, please enter a valid index", index);
                            }
                        },
                        Err(e) => {
                            if input == "Exit" || input == "Quit" || input == "exit" || input == "quit" {
                                keep_looping = false;
                            }
                            println!("ODIN ERROR {} the input was not a number", e);
                        },
                    }
                },
                Err(e) => {
                    panic!("HEMIDALL ERROR {} with player input in the battlefield", e);
                },
            }
        }
        None
    }
}
