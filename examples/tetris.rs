extern crate rand;
use std::ops::Add;
use storm::time::*;
use storm::*;

use rodio::Source;
use std::fs::File;
use std::io::BufReader;

use rand::Rng;

/// Run with: cargo run --example tetris --release
fn main() {
    Engine::start(
        WindowSettings {
            title: String::from("Storm: Tetris"),
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
enum TetrisBlockType {
    L,
    S,
    Z,
    T,
    ReverseL,
    Square,
    Line,
    Empty,
}

impl TetrisBlockType {
    pub fn get_offsets(&self) -> [Pos; 4] {
        match self {
            TetrisBlockType::L => {
                return [
                    Pos {
                        x: 0,
                        y: 1,
                    },
                    Pos {
                        x: 0,
                        y: 0,
                    },
                    Pos {
                        x: 0,
                        y: -1,
                    },
                    Pos {
                        x: 1,
                        y: -1,
                    },
                ];
            }
            TetrisBlockType::S => {
                return [
                    Pos {
                        x: 1,
                        y: 0,
                    },
                    Pos {
                        x: 0,
                        y: 0,
                    },
                    Pos {
                        x: 0,
                        y: -1,
                    },
                    Pos {
                        x: -1,
                        y: -1,
                    },
                ];
            }
            TetrisBlockType::Z => {
                return [
                    Pos {
                        x: -1,
                        y: 0,
                    },
                    Pos {
                        x: 0,
                        y: 0,
                    },
                    Pos {
                        x: 0,
                        y: -1,
                    },
                    Pos {
                        x: 1,
                        y: -1,
                    },
                ];
            }
            TetrisBlockType::T => {
                return [
                    Pos {
                        x: 0,
                        y: 0,
                    },
                    Pos {
                        x: 1,
                        y: 0,
                    },
                    Pos {
                        x: -1,
                        y: 0,
                    },
                    Pos {
                        x: 0,
                        y: -1,
                    },
                ];
            }
            TetrisBlockType::ReverseL => {
                return [
                    Pos {
                        x: 0,
                        y: 1,
                    },
                    Pos {
                        x: 0,
                        y: 0,
                    },
                    Pos {
                        x: 0,
                        y: -1,
                    },
                    Pos {
                        x: -1,
                        y: -1,
                    },
                ];
            }
            TetrisBlockType::Square => {
                return [
                    Pos {
                        x: 0,
                        y: 0,
                    },
                    Pos {
                        x: 1,
                        y: 0,
                    },
                    Pos {
                        x: 0,
                        y: -1,
                    },
                    Pos {
                        x: 1,
                        y: -1,
                    },
                ];
            }
            TetrisBlockType::Line => {
                return [
                    Pos {
                        x: 0,
                        y: 1,
                    },
                    Pos {
                        x: 0,
                        y: 0,
                    },
                    Pos {
                        x: 0,
                        y: -1,
                    },
                    Pos {
                        x: 0,
                        y: -2,
                    },
                ];
            }
            TetrisBlockType::Empty => {
                return [Pos {
                    x: 0,
                    y: 0,
                }; 4];
            }
        }
    }

    pub fn color(&self) -> storm::color::RGBA8 {
        match self {
            TetrisBlockType::L => {
                return storm::color::BLUE;
            }
            TetrisBlockType::S => {
                return storm::color::GREEN;
            }
            TetrisBlockType::Z => {
                return storm::color::ORANGE;
            }
            TetrisBlockType::T => {
                return storm::color::RED;
            }
            TetrisBlockType::ReverseL => {
                return storm::color::MAGENTA;
            }
            TetrisBlockType::Square => {
                return storm::color::RGBA8::new_raw(125, 125, 0, 255);
            }
            TetrisBlockType::Line => {
                return storm::color::PURPLE;
            }
            TetrisBlockType::Empty => {
                return storm::color::WHITE;
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Pos {
        Pos {
            x,
            y,
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

struct TetrisCluster {
    pub block_type: TetrisBlockType,
    pub current_position: Pos,
    pub offsets: [Pos; 4],
}

impl TetrisCluster {
    pub fn new(current_position: Pos, block_type: TetrisBlockType) -> TetrisCluster {
        TetrisCluster {
            block_type,
            current_position,
            offsets: block_type.get_offsets(),
        }
    }

    pub fn generate_offsets(&mut self, direction: i32) -> [Pos; 4] {
        match self.block_type {
            TetrisBlockType::Square => {
                return self.offsets;
            }
            _ => {}
        }

        let mut new_offsets = [Pos::new(0, 0); 4];

        for (count, offset) in self.offsets.iter_mut().enumerate() {
            if direction == 1 {
                let old_x = offset.x;
                let old_y = offset.y;
                new_offsets[count].x = old_y;
                new_offsets[count].y = old_x * -1;
            } else {
                let old_x = offset.x;
                let old_y = offset.y;
                new_offsets[count].x = old_y * -1;
                new_offsets[count].y = old_x;
            }
        }

        return new_offsets;
    }
}

fn random_tetris_block() -> TetrisBlockType {
    match rand::Rng::gen_range(&mut rand::thread_rng(), 0, 7) {
        0 => TetrisBlockType::S,
        1 => TetrisBlockType::Z,
        2 => TetrisBlockType::L,
        3 => TetrisBlockType::ReverseL,
        4 => TetrisBlockType::Square,
        5 => TetrisBlockType::T,
        _ => TetrisBlockType::Line,
    }
}

fn game(mut engine: Engine) {
    let device = rodio::default_output_device().unwrap();

    let file = File::open("./examples/resources/tetris.ogg").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    rodio::play_raw(&device, source.convert_samples());

    let mut board = [[TetrisBlockType::Empty; 10]; 40];

    //  let mut current_block_cluster :

    let mut clock = Clock::new(144);

    let texture_1 = engine.texture_create(include_bytes!("resources/3.png").as_ref(), TextureFormat::PNG);

    engine.window_clear_color(storm::color::BLACK);
    let screen = engine.batch_create(&BatchSettings::default());
    let mut sprites = Vec::new();

    let mut sprite = Sprite::default();
    //sprite.texture = texture_1;
    sprite.size.x = sprite.size.x / 5;
    sprite.size.y = sprite.size.y / 5;

    for x in 0..10 {
        for y in 0..40 {
            sprite.pos.x = (x * 20) as f32 - 100.0f32;
            sprite.pos.y = (y * 20) as f32 - 400.0f32;
            sprites.push(sprite);
        }
    }
    let mut strings = Vec::new();
    let mut text = Text::default();

    {
        text.set_string("Score: 0");
        text.color = color::WHITE;
        text.pos.x = 125.0;
        text.pos.y += 375.0;
        strings.push(text);
        // Assign the strings we want to draw to a batch.
        engine.text_set(&screen, &strings);
    }

    let mut score = 0;
    let mut curret_cluster = TetrisCluster::new(Pos::new(4, 38), random_tetris_block());

    let position = curret_cluster.current_position;

    for offset in curret_cluster.offsets.iter() {
        let block_pos = position + *offset;
        board[block_pos.y as usize][block_pos.x as usize] = curret_cluster.block_type;
    }

    engine.sprite_set(&screen, &sprites);
    let mut is_active = true;
    let mut generate_new_cluster = false;
    let mut update_count = 0;
    while is_active {
        if generate_new_cluster {
            curret_cluster = TetrisCluster::new(Pos::new(4, 38), random_tetris_block());
            generate_new_cluster = false;
        }

        let mut movement_vector = Pos::new(0, -1);
        let mut lateral_move = false;

        let mut rotation_direction = 0;
        while let Some(message) = engine.input_poll() {
            match message {
                InputMessage::CloseRequested => is_active = false,
                InputMessage::KeyPressed(key) => match key {
                    KeyboardButton::Left => {
                        movement_vector.x = -1;
                        movement_vector.y = 0;
                        lateral_move = true;
                    }
                    KeyboardButton::Right => {
                        movement_vector.x = 1;
                        movement_vector.y = 0;
                        lateral_move = true;
                    }
                    KeyboardButton::Q => {
                        rotation_direction = -1;
                        movement_vector.x = 0;
                        movement_vector.y = 0;
                    }
                    KeyboardButton::E => {
                        rotation_direction = 1;
                        movement_vector.x = 0;
                        movement_vector.y = 0;
                    }

                    KeyboardButton::Escape => is_active = false,
                    _ => {}
                },
                _ => {}
            }
        }
        if update_count == 20 || lateral_move || rotation_direction != 0 {
            let mut position = curret_cluster.current_position;

            //test the set of board positions under the current ones, are they occupied/the end of the board
            //first we need to erase of current postions so we don't set off the check
            for offset in curret_cluster.offsets.iter() {
                let block_pos = position + *offset;
                board[block_pos.y as usize][block_pos.x as usize] = TetrisBlockType::Empty;
            }

            //check if we can do the move
            let mut all_empty = true;
            let mut hit_edge = false;

            let use_offsets;
            if rotation_direction == 0 {
                use_offsets = curret_cluster.offsets;
            } else {
                use_offsets = curret_cluster.generate_offsets(rotation_direction);
            }

            for offset in use_offsets.iter() {
                let block_pos = position + *offset + movement_vector;
                if block_pos.x < 0 || block_pos.y < 0 || block_pos.x == 10 || block_pos.y == 40 {
                    hit_edge = true;
                    continue;
                }
                match board[block_pos.y as usize][block_pos.x as usize] {
                    TetrisBlockType::Empty => {}
                    _ => {
                        all_empty = false;
                    }
                }
            }

            if hit_edge == false && all_empty {
                curret_cluster.offsets = use_offsets;
            }

            if hit_edge || (movement_vector.y != 0 && all_empty == false) {
                movement_vector = Pos::new(0, 0);
            }
            //if we passed the check, update the position of the block
            if all_empty {
                position = position + movement_vector;
                curret_cluster.current_position = position;
            } else if hit_edge == false {
                generate_new_cluster = true;
            }

            //write the postion back into the board into either the new or old place
            for offset in curret_cluster.offsets.iter() {
                let block_pos = position + *offset;
                board[block_pos.y as usize][block_pos.x as usize] = curret_cluster.block_type;
                if block_pos.y == 0 {
                    generate_new_cluster = true;
                }
            }
            if update_count == 20 {
                update_count = 0;
            }
        } else {
            update_count += 1;
        }

        for x in 0..10 {
            for y in 0..40 {
                let index = x * 40 + y;
                sprites[index].color = board[y][x].color();
            }
        }

        if generate_new_cluster == true {
            let mut row = 0;
            while row != 40 {
                let mut has_empty_slot = false;
                for x in 0..10 {
                    match board[row][x] {
                        TetrisBlockType::Empty => {
                            has_empty_slot = true;
                        }
                        _ => {}
                    }
                }

                if has_empty_slot == false {
                    for x in 0..10 {
                        board[row][x] = TetrisBlockType::Empty;
                    }
                    let mut shift_row = row + 1;
                    while shift_row != 40 {
                        for x in 0..10 {
                            board[shift_row - 1][x] = board[shift_row][x];
                            board[shift_row][x] = TetrisBlockType::Empty;
                        }
                        shift_row += 1;
                    }
                    score += 100;
                } else {
                    row += 1;
                }
            }
            strings[0].set_string(&("Score".to_string() + " : " + &score.to_string()));
            engine.text_set(&screen, &strings);
        }

        engine.sprite_set(&screen, &sprites);
        engine.window_commit();
        clock.tick();
    }
}
