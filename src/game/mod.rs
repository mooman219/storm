pub mod entity;
pub mod state;
pub mod universe;
pub mod world;

use bounded_spsc_queue::Producer;
use cgmath::*;
use render::color;
use render::geometry::triangle::*;
use render::message::*;
use render::message::producer::*;
use render::vertex::shape::*;

pub fn game_loop(frame_producer: Producer<RenderFrame>) {
    let mut render_producer = RenderProducer::new(frame_producer);
    for x in 0..3 {
        let offset = (x as f32) * 0.5f32;
        render_producer.create_rect(
            Vector2::new(-1f32 + offset, -1f32),
            Vector2::new(0.5f32, 0.5f32),
            color::GREEN,
        );
        render_producer.create_rect(
            Vector2::new(0f32 + offset, 0f32),
            Vector2::new(0.5f32, 0.5f32),
            color::RED,
        );
        render_producer.create_rect(
            Vector2::new(-0.5f32 + offset, -0.5f32),
            Vector2::new(0.5f32, 0.5f32),
            color::BLUE,
        );
    }
    render_producer.create_triangle(Triangle::new(
        ShapeVertex::new(0.0, -0.5, color::RED),
        ShapeVertex::new(-1.0, -1.0, color::BLUE),
        ShapeVertex::new(1.0, -1.0, color::GREEN),
    ));
    render_producer.create_triangle(Triangle::new(
        ShapeVertex::new(0.0, 0.5, color::RED),
        ShapeVertex::new(1.0, 1.0, color::BLUE),
        ShapeVertex::new(-1.0, 1.0, color::GREEN),
    ));
    render_producer.send();
}
