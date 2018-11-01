use storm::cgmath::{Vector2, Vector3};
use storm::utility::indexmap::*;
use storm::math::aabb::AABB2D;

use pong::Ball;
//Writing this for record keepign
pub struct Player {
    pub box_token: IndexToken,
    pub box_position: Vector3<f32>,
    pub box_shape: Vector2<f32>,
}

impl Player {
    pub fn new(box_token: IndexToken, box_position: Vector3<f32>, box_shape: Vector2<f32>) -> Player {
        Player {
            box_token,
            box_position,
            box_shape
        }
    }

    pub fn overlaps_box(&self, ball: &Ball) -> bool {
        let my_aabb = AABB2D::new(self.box_position.x, self.box_position.y, 
                                  self.box_position.x + self.box_shape.x, 
                                  self.box_position.y + self.box_shape.y);
        
        let thier_aabb = AABB2D::new(ball.ball_position.x,  ball.ball_position.y, 
                                     ball.ball_position.x + ball.ball_shape.x, 
                                     ball.ball_position.y + ball.ball_shape.y);
        
        return my_aabb.intersects(&thier_aabb);
    }
}