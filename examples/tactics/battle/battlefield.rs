use std::collections::HashMap;

use tactics::battle::BattlefieldTile;

use storm::cgmath::Vector2;

pub struct Battlefield {
    tiles: Vec<Vec<BattlefieldTile>>,
    character_positions: HashMap<String, Vector2<usize>> 
}

impl Battlefield {
    
    //battlefield sizes are fixed upon creation
    pub fn new() -> Battlefield {
        Battlefield {
            tiles: vec![],
            character_positions: HashMap::new()
        }

    }

    pub fn initalize_new_battlefield(&mut self, x_size: usize, y_size: usize) {
        let mut tiles  = vec![];

        for x in 0..x_size {
            tiles.push(vec![]);    
            for _ in 0..y_size {
                tiles[x].push(BattlefieldTile::new());
            }
        }
        self.tiles = tiles;
    }

    pub fn place_character(&mut self, name: String, x_position: usize, y_position: usize) -> bool {
        
        let tile = &mut self.tiles[x_position][y_position];

        if tile.is_empty() {
            //lets remove whatever the old positions might have been
            //nothing happens if there is no key of "name", so we do it for both cases of it there and not htere
            self.character_positions.remove(&name);
            //we then store the new postion into this hashmap
            //giving us an O(1) look up for any characters position on any battlefield
            self.character_positions.insert(name.clone(), Vector2::new(x_position, y_position));
            //we can ignore the result on this one because we are checking for is_empty, which is
            //the only reson this should fail
            let _ = tile.attempt_place_character_on_tile(name);
            return true;
        }
        false
    }

    //will try to look up the postion of the character with "character_name"
    //will return the grid position if yes
    //will return None if the character is not there
    pub fn get_location_of_character(&self, character_name: &String) -> Option<Vector2<usize>>{
        //NOTE: This is done this way due to the fact that .get will always return a Option<&T>, and not Option<T> which is what I want
        let tmp = self.character_positions.get(character_name);
        match tmp {
            Some(value) => {
                return Some(*value);
            }
            None => {
                return None;
            }
        }
    }

    //other then the place_character function, this must be the only way to
    //change a characters position on the map, it will enforce board rules
    pub fn move_characrter(&mut self, character_name: &String, _direction: Vector2<i32>) {
        let current = self.character_positions.remove(character_name);

        match current {
            Some(_current) => {
                //for now this is just a simple update of the characters
                //current positions, but this should be the primary way 
                //in which movement based events are triggered

            },
            //TODO: figure how when this might happen, and handle in gracefully
            None => {
                println!("{} is not a character in the ", character_name);
            }
        }
    }

    pub fn draw(&self) {
        println!("\t\tBATTLE FIELD\t\t");
        for row in &self.tiles {
            Battlefield::draw_header(self.tiles.len()); 
            for col in row {
                let icon = col.get_icon();

                //these character atleast from what I can tell, have extra right sided whitespace padding
                //to ensure that they line up in the grid, we simply omit the extra space that other icons 
                //are printed with
                if ((icon as u32) > 19967 && (icon as u32) < 40959)//CJK Unicode block 
                || ((icon as u32) > 4352 && (icon as u32) < 4607) //Hangul Jamo Unicode block
                || ((icon as u32) > 44032 && (icon as u32) < 55215) //Hangul Unicode Syllables
                {
                    print!("| {}", icon);
                    continue;
                }
                print!("| {} ", icon);
            }
            println!("|");
        }
        Battlefield::draw_header(self.tiles.len());
    }

    pub fn draw_header(x_length: usize) {
       for _ in 0..x_length {
           Battlefield::draw_top_or_bottom();
       }
       //formatting cleanup
       print!("+\n");
    }


    //the bottom and top of the sqaure are the same so only need one call
    pub fn draw_top_or_bottom() {
        print!("+ - ");
    }

    pub fn get_battlefield_size(&self) -> Vector2<usize> {
        Vector2::new(self.tiles.len(), self.tiles[0].len())
    }
}