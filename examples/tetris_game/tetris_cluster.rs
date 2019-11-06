use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Pos {
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

pub struct TetrisCluster {
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

#[derive(Copy, Clone)]
pub enum TetrisBlockType {
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

    pub fn random_tetris_block() -> TetrisBlockType {
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
}
