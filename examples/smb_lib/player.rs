use storm::math::aabb::*;
use storm::cgmath::*;

use storm::render::message::*;
use storm::render::color::*;


const GRAVITY_VELOCTY_MAX: f32 = 0.98f32;
const GRAVITY_ACCEL_RATE: f32 = 0.0001f32;

struct Textures {
    main: TextureReference,
    other: TextureReference,
}

pub struct Player {
    aabb: AABB2D,
    quad_ref: QuadReference,
    player_textures: Textures,
    velocity: Vector2<f32>,
    gravity: f32
}

impl Player {
    pub fn new(pos: Vector3<f32>, scale: Vector2<f32>, render: &mut RenderMessenger) -> Player {
        let player_textures = Textures {
            main: render.texture_create("./examples/testgame/1.png"),
            other: render.texture_create("./examples/testgame/2.png"),
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
            gravity: 0.0
        }
    }

    fn update_gravity(&mut self) {
        self.gravity += GRAVITY_ACCEL_RATE;
        if self.gravity > GRAVITY_VELOCTY_MAX {
            self.gravity = GRAVITY_VELOCTY_MAX;
        }
        self.velocity -= Vector2::new(0.0, self.gravity);
    }

    pub fn tick(&mut self, _delta: f32) {
        self.update_gravity();
    }

    pub fn update_pos(&mut self, others_boxes: &Vec<AABB2D>) {
        self.aabb.slide(&self.velocity, others_boxes);
    }

    pub fn render(&mut self, render: &mut RenderMessenger) {
        render.quad_update(
            self.quad_ref,
            self.aabb.min.extend(0.0),
            self.aabb.max - self.aabb.min,
            Color::new(1f32, 1f32, 1f32, 1f32),
            self.player_textures.main
        );
    }
}