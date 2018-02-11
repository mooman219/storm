pub mod entity;
pub mod state;
pub mod universe;
pub mod world;

use bounded_spsc_queue::Producer;
use render::geometry::triangle::*;
use render::vertex::shape::*;
use render::message::*;
use render::message::producer::*;
use render::color;
use cgmath::*;

pub fn game_loop(frame_producer: Producer<RenderFrame>) {
    let mut render_producer = RenderProducer::new(frame_producer);
    render_producer.create_rect(
        Vector2::new(-1f32, -1f32),
        Vector2::new(2f32, 2f32),
        color::GREEN,
    );
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
