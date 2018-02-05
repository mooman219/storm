pub mod entity;
pub mod state;
pub mod universe;
pub mod world;

use bounded_spsc_queue::Producer;
use render::geometry::quad::*;
use render::geometry::triangle::*;
use render::vertex::shape::*;
use render::message::*;
use render::message::comm::*;
use render::color;

pub fn game_loop(frame_producer: Producer<RenderFrame>) {
    let mut render_producer = RenderProducer::new(frame_producer);
    render_producer.create_quad(CreateQuadMessage {
        quad: Quad::new(
            ShapeVertex::new(-0.5, 0.5, color::RED),
            ShapeVertex::new(-0.5, -0.5, color::BLUE),
            ShapeVertex::new(0.5, 0.5, color::RED),
            ShapeVertex::new(0.5, -0.5, color::GREEN),
        ),
    });
    render_producer.create_triangle(CreateTriangleMessage {
        triangle: Triangle::new(
            ShapeVertex::new(0.0, -0.5, color::RED),
            ShapeVertex::new(-1.0, -1.0, color::BLUE),
            ShapeVertex::new(1.0, -1.0, color::GREEN),
        ),
    });
    render_producer.create_triangle(CreateTriangleMessage {
        triangle: Triangle::new(
            ShapeVertex::new(0.0, 0.5, color::RED),
            ShapeVertex::new(1.0, 1.0, color::BLUE),
            ShapeVertex::new(-1.0, 1.0, color::GREEN),
        ),
    });
    render_producer.send();
}
