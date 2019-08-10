extern crate rand;
use storm::time::*;
use storm::*;
use storm::cgmath::*;
use std::ops::Add;
mod pokemon_mod;
use pokemon_mod::*;

use rand::{
    Rng,
};

/// Run with: cargo run --example pokemon --release
fn main() {
    Engine::start(
        WindowSettings {
            title: String::from("Storm: Texture"),
            display_mode: DisplayMode::Windowed {
                width: 1280,
                height: 1024,
                resizable: true,
            },
            vsync: Vsync::Disabled,
        },
        game,
    );
}

#[derive(Copy, Clone)]
enum TileSlot {
    Wall,
    Grass,
    Empty
}

impl TileSlot {

    pub fn color(&self) -> storm::color::RGBA8 {
        match self {
            TileSlot::Wall => {
                storm::color::MAGENTA
            },
            TileSlot::Grass => {
                storm::color::GREEN
            },
            TileSlot::Empty => {
                storm::color::WHITE
            }
        }
    }

    pub fn texture_index(&self) -> usize {
        match self {
            TileSlot::Wall => {
                0
            },
            TileSlot::Grass => {
                1
            },
            TileSlot::Empty => {
                2
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Pos {
    pub x: isize,
    pub y: isize
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Pos {
        Pos {
            x,
            y
        }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Copy)]
struct Tile {
    slots: [TileSlot; 5],
    number_of_tiles: usize
}

impl Tile {
    
    pub fn new() -> Tile {
        Tile {
            slots: [TileSlot::Empty; 5],
            number_of_tiles: 0
        }
    }

    pub fn add_tile_slot(&mut self, slot: TileSlot) -> bool {
        if self.number_of_tiles < 5 {

            self.slots[self.number_of_tiles] = slot;
            self.number_of_tiles += 1;
            return true;
        }
        else {
            return false;
        }
    }

    pub fn get_top_tile_slot(&self) -> &TileSlot {
        if self.number_of_tiles == 0 {
            return &self.slots[0];
        }
        return &self.slots[self.number_of_tiles - 1];
    }

    pub fn remove_top_tile(&mut self) -> TileSlot {
        self.number_of_tiles -= 1;
        let removed_tiles = self.slots[self.number_of_tiles];
        self.slots[self.number_of_tiles] = TileSlot::Empty;
        return removed_tiles;
    }

}

fn random_tile_slot() -> TileSlot {
    match rand::Rng::gen_range(&mut rand::thread_rng(), 0, 2) {
            0 => TileSlot::Grass,
            1 => TileSlot::Wall,
            2 => TileSlot::Empty,
            _ => TileSlot::Empty,
        }
}

fn game(mut engine: Engine) {

    let pokemon_1 = Pokemon::new(10, String::from("Foo"), 1, [BattleMove::new(10, String::from("HEALTH"), true, 1)]);
    let pokemon_2 = Pokemon::new(10, String::from("Bar"), 1, [BattleMove::new(10, String::from("HEALTH"), true, 1)]);
    let mut battle = Battle::new([pokemon_1, pokemon_2]);

    let mut clock = Clock::new(144);

    let mut world  = [[Tile::new(); 30];30];

    let mut player_position = Pos::new(15, 15);
    let mut player_draw_position = Vector3::new(0.0f32, 0.0f32, 1.0f32);
    let mut player_movement_vectors = Vector3::new(0.0f32, 0.0f32, 0.0f32);
    let textures = vec![engine.texture_create(include_bytes!("resources/PokemonSprites/36.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("resources/PokemonSprites/32.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("resources/PokemonSprites/17.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("resources/PokemonSprites/24.png").as_ref(), TextureFormat::PNG)];

    engine.window_clear_color(storm::color::BLACK);
    let screen = engine.batch_create(&BatchSettings::default());
    let mut sprites = Vec::new();
    
    let mut sprite = Sprite::default();
    sprite.texture = textures[0];
    sprite.size.x = sprite.size.x / 5;
    sprite.size.y = sprite.size.y / 5;


    for x in 0..30 {
        for y in 0..30 {
            sprite.pos.x = (x * 20) as f32 - 100.0f32;
            sprite.pos.y = (y * 20) as f32 - 400.0f32;
            
            world[x][y].add_tile_slot(TileSlot::Grass);
            sprite.color = storm::color::WHITE;;
            sprites.push(sprite);
        }
    }

    sprite.texture = textures[3];
    sprite.pos.x = (15 * 20) as f32 - 100.0f32;
    sprite.pos.y = (15 * 20) as f32 - 400.0f32;
    sprite.pos.z = 1.0f32;
    sprite.color = storm::color::WHITE;
    
    sprites.push(sprite);

    for x in 0..30 {
        world[x][0].add_tile_slot(TileSlot::Wall);
        world[x][29].add_tile_slot(TileSlot::Wall);
    }

    battle.tick_battle();

    engine.sprite_set(&screen, &sprites);
    let mut is_active = true;
    let mut movement_vector = Pos::new(0, 0);
    while is_active {

        while let Some(message) = engine.input_poll() {
            match message {
                InputMessage::CloseRequested => is_active = false,
                InputMessage::KeyPressed(key) => {
                    match key {
                        KeyboardButton::Left => {
                            movement_vector.x = -1;
                            player_movement_vectors.x = -1.0f32;
                            player_movement_vectors.y = 0.0f32;
                        },
                        KeyboardButton::Right => {
                            movement_vector.x = 1;
                            player_movement_vectors.x = 1.0f32;
                            player_movement_vectors.y = 0.0f32;
                        },
                        KeyboardButton::Up => {
                            movement_vector.y = 1;
                            player_movement_vectors.y = 1.0f32;
                            player_movement_vectors.x = 0.0f32;
                        },
                        KeyboardButton::Down => {
                            movement_vector.y = -1;
                            player_movement_vectors.y = -1.0f32;
                            player_movement_vectors.x = 0.0f32;
                        }
                        KeyboardButton::Q => {
                        },
                        KeyboardButton::E => {
                        },

                        KeyboardButton::Escape => is_active = false,
                        _ => {},
                    }
                },
                InputMessage::KeyReleased(key) => match key {
                    KeyboardButton::Left => {
                        movement_vector.x = 0;
                        player_movement_vectors.x = 0.0f32;
                    },
                    KeyboardButton::Right => {
                        movement_vector.x = 0;
                        player_movement_vectors.x = 0.0f32;
                    },
                    KeyboardButton::Up => {
                        movement_vector.y = 0;
                        player_movement_vectors.y = 0.0f32;
                    },
                    KeyboardButton::Down => {
                        movement_vector.y = 0;
                        player_movement_vectors.y = 0.0f32;
                    },
                    _ => {

                    }
                },
                _ => {

                }
            }
        }

        let desired_position = movement_vector + player_position;
        player_draw_position += player_movement_vectors;
        if desired_position.x < 0 || desired_position.x >= 30 || desired_position.y < 0 || desired_position.y >= 30 {

        }
        else {

            match world[desired_position.x as usize][ desired_position.y as usize].get_top_tile_slot() {
                TileSlot::Grass => {
                    //you get to move!!!
//                    let current_player_tile = world[player_position.x as usize][player_position.y as usize].remove_top_tile();
//                    world[desired_position.x as usize][desired_position.y as usize].add_tile_slot(current_player_tile);
//                    player_position = desired_position;
                },
                TileSlot::Wall => {
                    //reject current attempt at movement
                },
                TileSlot::Empty => {
                    //Rject movement
                }
            }
        }

        

        for x in 0..30 {
            for y in 0..30 {
                sprites[x * 30 + y].texture = textures[world[x][y].get_top_tile_slot().texture_index()];
            }
        }

        sprites[30 * 30].pos = player_draw_position;

        engine.sprite_set(&screen, &sprites);
    
        engine.window_commit();
        clock.tick();
    }
}
