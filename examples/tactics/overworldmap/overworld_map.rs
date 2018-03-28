use tactics::overworldmap::map_tile::{MapTile, TileType};
use rand::distributions::{Range, Sample};
use rand;
use std::io;
use storm::cgmath::Vector2;

const MAP_X_SIZE: usize = 10;
const MAP_Y_SIZE: usize = 10;
const START_POSITIONS_MIN: usize = 5;

//used in the the maps movement mode
#[derive(Debug)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right
}

enum OverworldMapState {
    WaitingForPlayerInput
}

pub struct OverworldMap {
    map: Vec<Vec<MapTile>>,
    party_position_on_map: Vector2<usize>
}

impl OverworldMap {

    pub fn new() -> OverworldMap {
        OverworldMap {
            map : vec![],
            party_position_on_map: Vector2::new(0, 0),
        }
    }

    //this will genreate a 2d array made up of Vectors, with map tiles
    pub fn generate_map() -> Vec<Vec<MapTile>> {
        let mut map = vec![];

        //this is a magic number for the number of MapTile enums, would love this to be compile time op
        let mut tile_type = Range::new(0, 4);
        let mut rng = rand::thread_rng();
        
        //we create 100 tiles
        for i in 0..MAP_X_SIZE {
            map.push(vec![]);
            for _ in 0..MAP_Y_SIZE {
                //with this unhappy match statement, but what you going to do
                //like please tell me, I would love a more elegant way of doing this
                let tt = tile_type.sample(&mut rng);
                match tt {
                    0 => {
                        let tile = MapTile::new(TileType::Nothing);
                        map[i].push(tile);
                    },
                    1 => {
                        let tile = MapTile::new(TileType::Battle);
                        map[i].push(tile);
                    },
                    2 => {
                        let tile = MapTile::new(TileType::PersonEncounter);
                        map[i].push(tile);
                    },
                    3 => {
                        let tile = MapTile::new(TileType::Shop);
                        map[i].push(tile);
                    },
                    _ => {
                        panic!("HELA ERROR: This is a problem with rusts random, soo a larger problem then");
                    }
                }
            }
        }
        return map;
    }

    pub fn move_party_from_tile_to_tile(&mut self, direction: Vector2<usize>, do_subtract: bool) {
        
        self.map[self.party_position_on_map.x][self.party_position_on_map.y].party_left_tile();
        if !do_subtract {
            self.party_position_on_map += direction;
        }
        else {
            self.party_position_on_map -= direction;
        }
        self.map[self.party_position_on_map.x][self.party_position_on_map.y].flip_tile();
        self.map[self.party_position_on_map.x][self.party_position_on_map.y].party_on_tile();
    }

    pub fn find_available_movement_direction_for_party(pos: Vector2<usize>) -> Vec<MovementDirection> {
        let mut directions = vec![];
        let current_party_position = pos;
        
        //above the current tile
        if (current_party_position.y as i32) - 1 >= 0 {
            directions.push(MovementDirection::Up);
        }

        //below
        if current_party_position.y + 1  < MAP_Y_SIZE {
            directions.push(MovementDirection::Down);
        }

        //to the right
        if current_party_position.x + 1 < MAP_X_SIZE {
            directions.push(MovementDirection::Right);
        }

        //to the left
        if (current_party_position.x as i32) - 1 >= 0 {
            directions.push(MovementDirection::Left);
        }

        directions
    }

    //we call this start_new_game because after this we are offically in the new game
    //it will eventually have input of the party, and whatever user attributes
    pub fn start_new_game(&mut self) -> bool {
        //go through an create the actual map
        self.map = OverworldMap::generate_map();
        self.layout_map();

        //ask the user where they want to start
        let start_position = OverworldMap::prompt_for_start_position();

        //make sure they gave us one
        match start_position {
            Some(pos) => {
                //and so they start their jounrey with one speck of light in darkness
                self.map[pos.x][pos.y].flip_tile();
                self.map[pos.x][pos.y].party_on_tile();
                self.party_position_on_map = pos;

                return true;
            },
            None => {
                println!("You need to have a start position to start the game, aborting new game startup");
                return false;
            }
        }
    }

    //this will be called at the start of a game to have a player select which place on the board they will start
    fn prompt_for_start_position() -> Option<Vector2<usize>> {
        println!("Please pick a following position to start");
        println!("Enter the number you would like to start at");
        
      
        let possible_start_positions = OverworldMap::genreate_random_start_positions();

        let mut count = 0;
        
        for start in &possible_start_positions {
            println!("{}. {}, {}", count, start.x, start.y);
            count += 1;
        }

        let mut done = false;
        while !done {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_n) => {
                    let input = input.trim();
                    //lazy programmer helpers
                    if input == "quit" || input == "exit" {
                        done = true;
                    }
                    let result = input.parse::<usize>();
                    match result {
                        Ok(index) => {
                            if index < possible_start_positions.len() {
                                return Some(possible_start_positions[index]);
                            }
                            else {
                                println!("BALDER ERROR: {} is larger then the number of options, please try again", index);
                            }
                        },
                        Err(_e) => {
                            println!("ODIN ERROR {} is not a number", input);
                        }
                    }
                }
                //I am just going to panic on an error here, not sure what the cases of the error are
                //and this is so early in the new game flow that getting back to it isnt hard
                Err(error) => {
                    panic!("{} HEMIDALL ERROR with the input for starting position selection", error);
                },
            }
        }

        None
    }

    pub fn genreate_random_start_positions() -> Vec<Vector2<usize>> {
        let mut rng = rand::thread_rng();

        let mut num_range = Range::new(0, START_POSITIONS_MIN);
        let range_top = num_range.sample(&mut rng) + START_POSITIONS_MIN;
        let mut possible_start_positions = vec![];

        //this will return a position in the form of either ([0...x_length), 0), or (0..[0..y_legth))
        //provides no gurantee that it is unique
        let mut random_start = move |x_length: usize, y_length: usize| -> Vector2<usize> {
//            let mut rng = rand::thread_rng();
            let mut x_num_range = Range::new(0, x_length);
            let mut y_num_range = Range::new(0, y_length);

            let x_num = x_num_range.sample(&mut rng);
            let y_num = y_num_range.sample(&mut rng);

            //cheapo random binary choice
            //can only be 0, or 1
            let di = x_num_range.sample(&mut rng) % 2;
            //are we adding a starting positiong along the x-axis, or y-axis
            if di == 0 {
                //some number of the positions should be on the other edge of the map, this is repeated below for the y-axis starts
                let shift_over_to_other_edge = match y_num_range.sample(&mut rng) % 2 == 0{
                    true => {
                        0
                    },
                    false => {
                        y_length - 1
                    } 
                };
                return Vector2::new(x_num, shift_over_to_other_edge);
            }

            let shift_over_to_other_edge = match y_num_range.sample(&mut rng) % 2 == 1{
                true => {
                    0
                },
                false => {
                    x_length - 1
                }
            };
            return Vector2::new(shift_over_to_other_edge, y_num);
        };


        let mut count = 0;
        while count != range_top {
            
            let possible_new = random_start(MAP_X_SIZE, MAP_Y_SIZE);

            //no use in having duplicate start positions 
            if possible_start_positions.contains(&possible_new) {
                continue;
            }

            possible_start_positions.push(possible_new);
            count+=1;
        }

        return possible_start_positions;
    }

    pub fn get_party_position(&self) -> Vector2<usize> {
        self.party_position_on_map
    }

    pub fn layout_map(&self) {
        println!("\n\n\t\t THE MAP \t\t");
        for row in 0..MAP_Y_SIZE {
            OverworldMap::draw_header();
            self.draw_row(row);
        }
        //cleanup formatting
        OverworldMap::draw_header();
    }
    
    #[inline]
    pub fn draw_header() {
       for _ in 0..MAP_X_SIZE {
           OverworldMap::draw_top_or_bottom();
       } 
       //formatting cleanup
       print!("+\n");
    }
    
    #[inline]
    pub fn draw_row(&self, row: usize) {
        for col_num in 0..MAP_X_SIZE {
            print!("| {} ", self.map[col_num][row].draw());
        }
        //formatting cleanup
        println!("|");
    }

    //the bottom and top of the sqaure are the same so only need one call
    #[inline]
    pub fn draw_top_or_bottom() {
        print!("+ - ");
    }
}