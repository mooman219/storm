use storm::cgmath::{Vector2, Vector3};
use storm::render::message::*;

//Writing this for record keepign
pub struct Ball {
    pub ball_token: QuadReference,
    pub ball_position: Vector3<f32>,
    pub ball_shape: Vector2<f32>,
}

impl Ball {
    pub fn new(ball_token: QuadReference, ball_position: Vector3<f32>, ball_shape: Vector2<f32>) -> Ball {
        Ball {
            ball_token,
            ball_position,
            ball_shape,
        }
    }
}
