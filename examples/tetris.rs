extern crate storm;

use storm::cgmath::*;
use storm::game::*;
use storm::input::message::*;
use storm::log::LevelFilter;
use storm::render::color;
use storm::render::color::*;
use storm::render::message::*;
use storm::time::clock::*;

const BLOCK_SCALE: f32 = 0.15;

/// Run with: cargo run --example testgame --release
fn main() {
    storm::log::set_max_level(LevelFilter::Trace);
    storm::run::<Tetris>();
}



pub struct Textures {
    main: TextureReference,
}

pub struct Tetris {
    render: RenderMessenger,
    textures: Textures,
    clock: Clock,
    translation: Vector2<f32>,
    tetris_board: Vec<Vec<MoveableSquare>>,
    tetris_live_board: Vec<Vec<color::Color>>,
    live_cluster: Vec<Vector2<usize>>,
    internal_timer: u64
}

impl Tetris {

    //will return true if the cluster has stopped(end of the board/into another cluster)
    pub fn update_live_cluster(&mut self) -> bool {
        let new_live_cluster = vec![self.live_cluster[0] - Vector2::new(0, 1),
                                        self.live_cluster[1] - Vector2::new(0, 1),
                                        self.live_cluster[2] - Vector2::new(0, 1)];
        self.live_cluster = new_live_cluster;
        return false;
    }

    pub fn update(&mut self) {
        self.internal_timer += 1;
        if self.internal_timer % 100 == 0 {
            for point in &self.live_cluster {
                self.tetris_live_board[point.y][point.x] = color::TRANSPARENT;
            }
            let _ = self.update_live_cluster();
            for point in &self.live_cluster {
                self.tetris_live_board[point.y][point.x] = color::RED;
            }
        }
    }
}


impl Game for Tetris {
    fn new(mut render: RenderMessenger) -> Self {
          let textures = Textures {
            main: render.texture_create("./examples/tetris/block.png"),
        };

        let mut test_sqaure = vec![];
        for y in 0..28 {
            test_sqaure.push(vec![]);
            for x in 0..10 {
                test_sqaure[y].push(
                        MoveableSquare::new(
                            &mut render, Vector3::new((BLOCK_SCALE * x as f32) - 1.0 , (BLOCK_SCALE * y as f32) - 2.0, 0.0),
                            textures.main
                            )
                    );
            }
        }

        let mut tetris_live_board = vec![];
        for y in 0..28 {
            tetris_live_board.push(vec![]);
            for _ in 0..10 {
                tetris_live_board[y].push(
                    color::TRANSPARENT
                );
            }
        }

        tetris_live_board[27][0] = color::ORANGE;
        tetris_live_board[26][0] = color::ORANGE;
        tetris_live_board[27][1] = color::ORANGE;
        

        let mut game = Tetris {
            render: render,
            clock: Clock::new(144),
            textures,
            translation: Vector2::new(0f32, 0f32),
            tetris_board: test_sqaure,
            tetris_live_board,
            live_cluster: vec![Vector2::new(0, 27), Vector2::new(0, 26), Vector2::new(1, 27)],
            internal_timer: 0
        };
        game.render.texture_create("./examples/tetris/block.png");
        game.render.window_title("Tetris");
        game.render.send();
        game
    }

    fn input(&mut self, _event: InputFrame) {
      
    }

    fn tick(&mut self) {
        let delta = self.clock.get_delta();

        self.update();
 
        for y in 0..28 {
            for x in 0..10 {
                self.tetris_board[y][x].update(delta, &mut self.render, &self.tetris_live_board[y][x]);
            }
        }
        self.render.translate(self.translation);

        self.render.send();
        self.clock.tick();
    }
}

pub struct MoveableSquare {
    pos: Vector3<f32>,
    size: Vector2<f32>,
    index: QuadReference,
    texture: TextureReference,
}

impl MoveableSquare {
    pub fn new(render: &mut RenderMessenger, pos: Vector3<f32>, texture: TextureReference) -> MoveableSquare {
//        let pos = Vector3::new(-0.5f32, -0.5f32, 0.125f32);
        let size = Vector2::new(BLOCK_SCALE, BLOCK_SCALE);
        let index = render.quad_create(pos, size, color::ORANGE, DEFAULT_TEXTURE);
        MoveableSquare {
            pos: pos,
            size: size,
            index: index,
            texture
        }
    }

    pub fn generate_index(&mut self, render: &mut RenderMessenger) {
    }

    pub fn update(&mut self, delta: f32, render: &mut RenderMessenger, render_color: &color::Color) {
        render.quad_update(
            self.index,
            self.pos,
            self.size,
            *render_color,
            self.texture,
        );
    }
}
