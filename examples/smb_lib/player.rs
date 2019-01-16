use std::collections::HashMap;
use storm::math::aabb::*;
use storm::cgmath::*;


use storm::render::message::*;
use storm::render::color::*;
use storm::render::color;

const GRAVITY_VELOCTY_MAX: f32 = 0.5f32;
const GRAVITY_ACCEL_RATE: f32 = 0.00005f32;
const WALL_GRAVITY_ACCEL_RATE: f32 = GRAVITY_ACCEL_RATE / 2.0;
const X_AXIS_VELOCITY : f32 = 0.02f32;
const JUMP_VELOCITY : f32 = 0.04f32;

struct Textures {
    main: TextureReference,
    _other: TextureReference,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum PlayerState {
    Idle,
    Jumping,
    Falling,
    OnWall
}

pub struct Player {
    aabb: AABB2D,
    quad_ref: QuadReference,
    player_textures: Textures,
    velocity: Vector2<f32>,
    gravity: f32,
    player_state: PlayerState,
    player_color: HashMap<PlayerState, Color>
}

impl Player {
    pub fn new(pos: Vector3<f32>, scale: Vector2<f32>, render: &mut RenderMessenger) -> Player {
        let player_textures = Textures {
            main: render.texture_create("./examples/testgame/1.png"),
            _other: render.texture_create("./examples/testgame/2.png"),
        };

        Player {
            aabb: AABB2D::new(pos.x, pos.y, pos.x + scale.x, pos.y + scale.y),
            quad_ref: render.quad_create(
                pos,
                scale,
                Color::new(1f32, 1f32, 1f32, 1f32),
                player_textures.main
            ),
            player_textures,
            velocity: Vector2::new(0.0, 0.0),
            gravity: 0.0,
            player_state: PlayerState::Falling,
            player_color: [(PlayerState::Idle, color::GREEN), (PlayerState::Falling, color::RED), (PlayerState::Jumping, color::ORANGE), (PlayerState::OnWall, color::YELLOW)].iter().cloned().collect()
        }
    }

    pub fn move_x_axis(&mut self, direction: f32) {
        self.velocity.x += direction * X_AXIS_VELOCITY;
    }

    pub fn jump(&mut self) {
        match self.player_state {
            PlayerState::Idle => {
                self.velocity.y = JUMP_VELOCITY;
                self.gravity = 0.0;
                self.player_state = PlayerState::Jumping;
            },
            PlayerState::OnWall => {
                self.velocity.y = JUMP_VELOCITY;
                self.gravity = 0.0;
                self.player_state = PlayerState::Jumping;
            }
            _ => {

            }
        }
    }

    fn update_gravity(&mut self, gravity_accel_rate: f32) {
        self.gravity += gravity_accel_rate;
        if self.gravity > GRAVITY_VELOCTY_MAX {
            self.gravity = gravity_accel_rate;
        }
        self.velocity -= Vector2::new(0.0, self.gravity);
    }

    pub fn tick(&mut self, _delta: f32) {
        match self.player_state {
            PlayerState::Idle => {

            },
            PlayerState::Falling => {
                self.update_gravity(GRAVITY_ACCEL_RATE);
            },
            PlayerState::Jumping => {
                if self.velocity.y <= 0.0 {
                    self.player_state = PlayerState::Falling;
                }
                self.update_gravity(GRAVITY_ACCEL_RATE);
            },
            PlayerState::OnWall => {
                self.update_gravity(WALL_GRAVITY_ACCEL_RATE);
            }
        }
    }

    //STOP TRYING TO MAKE THIS ONE FUNCTION WITH TICK, IT ISNT GOING OT HAPPEn
    pub fn update_pos(&mut self, others_boxes: &Vec<AABB2D>) {
        let result = self.aabb.slide(&self.velocity, others_boxes);
        if result {
            match self.player_state {
                PlayerState::Idle => {
                    //if you where "idle", you must be moving left to right? 
                },
                PlayerState::Jumping => {
                    //Can only mean that you hit something above to you or onto a wall
                    self.player_state = PlayerState::OnWall;
                },
                PlayerState::Falling => {
                    //hit the ground or a wall
                    self.player_state = PlayerState::Idle;
                    self.velocity.y = 0.0;
                },
                PlayerState::OnWall => {
                    //still on the wall, or hit the ground
                }
            }
        }   
    }

    pub fn render(&mut self, render: &mut RenderMessenger) {
        render.quad_update(
            self.quad_ref,
            self.aabb.min.extend(0.0),
            self.aabb.max - self.aabb.min,
            self.player_color[&self.player_state],
            self.player_textures.main
        );
    }
}