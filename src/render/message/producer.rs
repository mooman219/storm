use bounded_spsc_queue::Producer;
use render::message::RenderFrame;
use std::mem;
use render::geometry::quad::*;
use render::geometry::triangle::*;
use render::message::*;
use render::vertex::shape::*;
use render::color::*;
use cgmath::*;

pub struct RenderProducer {
    frame_producer: Producer<RenderFrame>,
    frame: RenderFrame,
}

impl RenderProducer {
    pub fn new(frame_producer: Producer<RenderFrame>) -> RenderProducer {
        RenderProducer {
            frame_producer: frame_producer,
            frame: RenderFrame::new(),
        }
    }

    pub fn create_rect(&mut self, pos: Vector2<f32>, size: Vector2<f32>, color: Color) {
        self.frame.create_quad.push(CreateQuadMessage {
            quad: Quad::new(
                ShapeVertex::new(pos.x, pos.y + size.y, color),
                ShapeVertex::new(pos.x, pos.y, color),
                ShapeVertex::new(pos.x + size.x, pos.y + size.y, color),
                ShapeVertex::new(pos.x + size.x, pos.y, color),
            ),
        });
    }

    pub fn create_triangle(&mut self, triangle: Triangle<ShapeVertex>) {
        self.frame
            .create_triangle
            .push(CreateTriangleMessage { triangle: triangle });
    }

    pub fn send(&mut self) {
        let mut frame = RenderFrame::new();
        mem::swap(&mut frame, &mut self.frame);
        self.frame_producer.push(frame);
    }
}
