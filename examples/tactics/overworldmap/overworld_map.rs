use rand;
use rand::distributions::{Range, Sample};
use std::io;
use storm::cgmath::{Vector2, Vector3};
use storm::render::message::*;
use storm::utility::slotmap::*;
use tactics::overworldmap::map_tile::{MapTile, TileType};

const MAP_X_SIZE: usize = 10;
const MAP_Y_SIZE: usize = 10;
const START_POSITIONS_MIN: usize = 5;
const MAP_TILE_WIDTH: usize = 10;
const MAP_TILE_HEIGHT: usize = 10;
//used in the the maps movement mode
#[derive(Debug)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

enum OverworldMapState {
    WaitingForPlayerInput,
}

pub struct OverworldMap {
    map: Vec<Vec<MapTile>>,    //this is the actual tiles that handle all logic with one tiles
    map_state: Vec<Vec<char>>, //this is a record of the previous printed state of the map
    //if this differs between two calls of layout, we must redraw the map
    tile_index_tokens: Vec<Vec<IndexToken>>, //this is the list of index token for drawing the map
    //we allocate them at the start of the game
    //and then use them to update on a draw that triggers when the map_state was updated
    party_position_on_map: Vector2<usize>,
}

impl OverworldMap {
    pub fn new() -> OverworldMap {
        OverworldMap {
            map: vec![],
            map_state: vec![],
            tile_index_tokens: vec![],
            party_position_on_map: Vector2::new(0, 0),
        }
    }

    //this will genreate a 2d array made up of Vectors, with map tiles
    pub fn generate_maps(render: &mut RenderMessenger) -> (Vec<Vec<MapTile>>, Vec<Vec<char>>, Vec<Vec<IndexToken>>) {
        let mut map = vec![];
        let mut map_state = vec![];
        let mut index_tokens = vec![];

        //this is a magic number for the number of MapTile enums, would love this to be compile time op
        let mut tile_type = Range::new(0, 4);
        let mut rng = rand::thread_rng();

        //we create 100 tiles
        for i in 0..MAP_X_SIZE {
            map.push(vec![]);
            map_state.push(vec![]);
            index_tokens.push(vec![]);

            for j in 0..MAP_Y_SIZE {
                //with this unhappy match statement, but what you going to do
                //like please tell me, I would love a more elegant way of doing this
                let tt = tile_type.sample(&mut rng);
                let tile_type;
                match tt {
                    0 => {
                        tile_type = TileType::Nothing;
                        //if when asked to draw again, the token it returns differs from the one that map_state[x][y] has, that means we must redraw
                        //that tile
                        //we record its position, and add it to a list of changed tiles, that we itereat over and have it update the rects we want
                    },
                    1 => {
                        tile_type = TileType::Battle;
                    },
                    2 => {
                        tile_type = TileType::PersonEncounter;
                    },
                    3 => {
                        tile_type = TileType::Shop;
                    },
                    _ => {
                        panic!("HELA ERROR: This is a problem with rusts random, soo a larger problem then");
                    },
                }

                let tile = MapTile::new(tile_type);
                let initial_state = tile.draw();
                let tag_color = tile.color();

                map[i].push(tile); //we add our new tile to its 2d array
                map_state[i].push(initial_state); //we have it set an intial state for the map state
                if i % 2 == 0 {
                    index_tokens[i].push(render.create_rect(
                        Vector3::new(i as f32 * 10.0, j as f32 * 10.0, 0f32),
                        Vector2::new(MAP_TILE_WIDTH as f32, MAP_TILE_HEIGHT as f32),
                        tag_color,
                    )); //we create a rect for it
                } else {
                    index_tokens[i].push(render.create_rect(
                        Vector3::new(i as f32 * 10.0, j as f32 * 10.0, 0f32),
                        Vector2::new(MAP_TILE_WIDTH as f32, MAP_TILE_HEIGHT as f32),
                        tag_color,
                    )); //we create a rect for it
                }
            }
        }
        render.send();
        return (map, map_state, index_tokens);
    }

    pub fn move_party_from_tile_to_tile(&mut self, direction: Vector2<usize>, do_subtract: bool) {
        self.map[self.party_position_on_map.x][self.party_position_on_map.y].party_left_tile();
        if !do_subtract {
            self.party_position_on_map += direction;
        } else {
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
        if current_party_position.y + 1 < MAP_Y_SIZE {
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
    pub fn start_new_game(&mut self, render: &mut RenderMessenger) -> bool {
        //go through an create the actual map
        let all_nessacary_maps = OverworldMap::generate_maps(render);

        self.map = all_nessacary_maps.0;
        self.map_state = all_nessacary_maps.1;
        self.tile_index_tokens = all_nessacary_maps.2;

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
            },
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
                            } else {
                                println!(
                                    "BALDER ERROR: {} is larger then the number of options, please try again",
                                    index
                                );
                            }
                        },
                        Err(_e) => {
                            println!("ODIN ERROR {} is not a number", input);
                        },
                    }
                },
                //I am just going to panic on an error here, not sure what the cases of the error are
                //and this is so early in the new game flow that getting back to it isnt hard
                Err(error) => {
                    panic!(
                        "{} HEMIDALL ERROR with the input for starting position selection",
                        error
                    );
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
                let shift_over_to_other_edge = match y_num_range.sample(&mut rng) % 2 == 0 {
                    true => 0,
                    false => y_length - 1,
                };
                return Vector2::new(x_num, shift_over_to_other_edge);
            }

            let shift_over_to_other_edge = match y_num_range.sample(&mut rng) % 2 == 1 {
                true => 0,
                false => x_length - 1,
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
            count += 1;
        }

        return possible_start_positions;
    }

    pub fn get_party_position(&self) -> Vector2<usize> {
        self.party_position_on_map
    }

    pub fn layout_map(&mut self, render: &mut RenderMessenger) {
        if self.map.len() == 0 {
            //we are making an assumption that you would only want to layout a full map
            println!("Warning: Trying to layout empty map, Daft Punk: Something about us");
            return;
        }

        for x in 0..self.map.len() {
            for y in 0..self.map[0].len() {
                let possible_update_token = self.map[x][y].draw();
                if possible_update_token != self.map_state[x][y] {
                    //this means that this tile has to be updated
                    self.map_state[x][y] = possible_update_token;

                    //if we detect a change, that means we have to have update things
                    let index_token = &self.tile_index_tokens[x][y];
                    render.update_rect(
                        index_token,
                        Vector3::new(
                            x as f32 * MAP_TILE_WIDTH as f32,
                            y as f32 * MAP_TILE_HEIGHT as f32,
                            0f32,
                        ),
                        Vector2::new(MAP_TILE_WIDTH as f32, MAP_TILE_HEIGHT as f32),
                        self.map[x][y].color(),
                    );
                }
            }
        }

        render.send();

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
