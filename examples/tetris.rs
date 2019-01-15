#![feature(rustc_private)]
extern crate rand;

extern crate storm;

use storm::cgmath::*;
use storm::game::*;
use storm::input::message::*;
use storm::log::LevelFilter;
use storm::render::color;
use storm::render::message::*;
use storm::time::clock::*;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

const BLOCK_SCALE: f32 = 0.15;

/// Run with: cargo run --example testgame --release
fn main() {
    storm::log::set_max_level(LevelFilter::Trace);
    storm::run::<Tetris>();
}

#[derive(Clone)]
enum TetrisBlock {
    S = 0,
    Z = 1,
    L = 2,
    ReL = 3,
    Sqaure = 4,
    Line = 5
}

impl Distribution<TetrisBlock> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrisBlock {
         match rng.gen_range(0, 6) {
            0 => TetrisBlock::S,
            1 => TetrisBlock::Z,
            2 => TetrisBlock::L,
            3 => TetrisBlock::ReL,
            4 => TetrisBlock::Sqaure,
            _ => TetrisBlock::Line
        }
    }
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
    internal_timer: u64,
    blocks: Vec<Vec<Vector2<usize>>>,
    block_colors: Vec<color::Color>,
    current_block: TetrisBlock,
    draw_color: color::Color
}

impl Tetris {

    pub fn spawn_new_live_cluster(&mut self) {
        //this is what I want to do 
        let new_block_type : TetrisBlock = rand::random();
        self.draw_color = self.block_colors[new_block_type.clone() as usize].clone();
        self.live_cluster = self.blocks[new_block_type.clone() as usize].clone();
        self.current_block = new_block_type;
    }

    //will return true if the cluster has stopped(end of the board/into another cluster)
    fn update_live_cluster(&mut self) -> bool {

        let new_live_cluster = vec![self.live_cluster[0] - Vector2::new(0, 1),
                                    self.live_cluster[1] - Vector2::new(0, 1),
                                    self.live_cluster[2] - Vector2::new(0, 1),
                                    self.live_cluster[3] - Vector2::new(0, 1)];

        let result = self.is_live_cluster_overlapping(&new_live_cluster);
        //if we do overlap, bail without updating the clusters positions
        if result {
            return true;
        }


        //we are not overlapping with another cluster
        //but we could be at the bottom of the board
        let mut one_is_zero = false;
        for point in &new_live_cluster {
            if point.y == 0 {
                one_is_zero = true;
            }
        }

        //we always update the to the new cluster at this point
        self.live_cluster = new_live_cluster;

        //but if we happen to be at the bottom we need to kick off new cluster
        if one_is_zero {
            return true;
        }

        //else just be like, time to move on
        return false;
    }

    //we call this before we set the new positions, but after we have erased the old ones
    //to see if any of the positions on the board are already occupied
    //if this is the case we stop
    fn is_live_cluster_overlapping(&mut self, new_positions: &Vec<Vector2<usize>>) -> bool {
        let mut any_overlap = false;
        for position in new_positions {
            if self.tetris_live_board[position.y][position.x] != color::TRANSPARENT {
                any_overlap = true;
            }            
        }
        return any_overlap;
    }

    pub fn update(&mut self) {
        self.internal_timer += 1;
        if self.internal_timer % 30 == 0 {
            
            //this is a really cheap way of making sure that we do not collide with ourselves during our checks
            for point in &self.live_cluster {
                self.tetris_live_board[point.y][point.x] = color::TRANSPARENT;
            }

            let result = self.update_live_cluster();

            //we undo the rease before
            for point in &self.live_cluster {
                self.tetris_live_board[point.y][point.x] = self.draw_color;
            }
            if result {
                self.spawn_new_live_cluster();
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

        let mut block_types = vec![];

        //WOULD LOVE TO DO THIS IN A DIFFERENT WAY
        //S
        block_types.push(vec![Vector2::new(5, 25), Vector2::new(6, 25), Vector2::new(6, 26), Vector2::new(7, 26)]);
        //Z
        block_types.push(vec![Vector2::new(5, 26), Vector2::new(6, 26), Vector2::new(6, 25), Vector2::new(7, 25)]);
        //L
        block_types.push(vec![Vector2::new(5, 27), Vector2::new(5, 26), Vector2::new(5, 25), Vector2::new(6, 25)]);
        //Reverse L
        block_types.push(vec![Vector2::new(5, 27), Vector2::new(5, 26), Vector2::new(5, 25), Vector2::new(4, 25)]);
        //Sqaure
        block_types.push(vec![Vector2::new(5, 27), Vector2::new(4, 27), Vector2::new(5, 26), Vector2::new(4, 26)]);
        //Line
        block_types.push(vec![Vector2::new(5, 27), Vector2::new(5, 26), Vector2::new(5, 25), Vector2::new(5, 24)]);

        let block_colors = vec![color::BLUE, color::RED, color::PURPLE, color::GREEN, color::YELLOW, color::ORANGE];

        let mut game = Tetris {
            render: render,
            clock: Clock::new(144),
            textures,
            translation: Vector2::new(0f32, 0f32),
            tetris_board: test_sqaure,
            tetris_live_board,
            live_cluster: vec![],
            internal_timer: 0,
            blocks: block_types,
            block_colors: block_colors,
            current_block: TetrisBlock::Sqaure,
            draw_color: color::TRANSPARENT
        };

        game.spawn_new_live_cluster();
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
