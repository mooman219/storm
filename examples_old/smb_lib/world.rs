use storm::math::aabb::*;
use storm::cgmath::*;

use storm::render::message::*;
use storm::render::color::*;

use crate::smb_lib::Player;

struct Textures {
    main: TextureReference,
    other: TextureReference,
}

pub struct World {
    bounding_boxes: Vec<AABB2D>,
    quad_referances: Vec<QuadReference>,
    world_textures: Textures,
    player: Player
}

impl World {
    pub fn new(render: &mut RenderMessenger) -> World  {

        let world_textures = Textures {
            main: render.texture_create("./examples/testgame/1.png"),
            other: render.texture_create("./examples/testgame/2.png"),
        };

        let mut bb = vec![];
        let mut qr = vec![];

        for i in 0..10 {
            let start_x = -0.5 + (i as f32 * 0.2);
            let aabb = AABB2D::new(start_x, -1.0, start_x + 0.2, -0.8);
            let quad_ref = render.quad_create(
                aabb.min.extend(0.0),
                aabb.max - aabb.min, 
                Color::new(1f32, 1f32, 1f32, 1f32),
                world_textures.other
            );
            bb.push(aabb);
            qr.push(quad_ref);
        }

        let pos = Vector3::new(1.0, 1.0, -0.125f32);
        let size = Vector2::new(0.2f32, 0.2f32);

        World {
            player: Player::new(pos, size, render),
            bounding_boxes: bb,
            quad_referances: qr,
            world_textures
        }
    }

    pub fn tick(&mut self, delta: f32) {
        self.player.tick(delta);
        self.player.update_pos(&self.bounding_boxes);
    }

    pub fn render(&mut self, render: &mut RenderMessenger) {
        self.player.render(render);
    }
}