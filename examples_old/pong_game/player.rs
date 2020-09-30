use storm::*;
use storm::cgmath::{Vector2, Vector3};
use storm::math::*;

use crate::pong_game::Ball; 
//Writing this for record keepign
pub struct Player {
    pub box_token: SpriteDescription,
}

impl Player {
    pub fn new(
        box_token: SpriteDescription,
    ) -> Player {
        Player {
            box_token,
        }
    }

    pub fn overlaps_box(&self, ball: &Ball) -> bool {
        let my_aabb = AABB2D::new(
            self.box_token.pos.x,
            self.box_token.pos.y,
            self.box_token.pos.x + self.box_token.size.x as f32,
            self.box_token.pos.y + self.box_token.size.y as f32 ,
        );

        let thier_aabb = AABB2D::new(
            ball.ball_token.pos.x,
            ball.ball_token.pos.y,
            ball.ball_token.pos.x + ball.ball_token.pos.x,
            ball.ball_token.pos.y + ball.ball_token.pos.y,
        );

        return my_aabb.intersects(&thier_aabb);
    }
}
