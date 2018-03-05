use bounded_spsc_queue::Producer;
use cgmath::*;
use render::color::*;
use render::geometry::quad::*;
use render::geometry::triangle::*;
use render::message::*;
use render::message::RenderFrame;
use render::vertex::shape::*;
use std::mem;
use utility::slotmap::*;

pub struct RenderProducer {
    frame_producer: Producer<RenderFrame>,
    frame: RenderFrame,
    map_rect: IndexMap,
    map_triangle: IndexMap,
}

impl RenderProducer {
    pub fn new(frame_producer: Producer<RenderFrame>) -> RenderProducer {
        RenderProducer {
            frame_producer: frame_producer,
            frame: RenderFrame::new(),
            map_rect: IndexMap::new(),
            map_triangle: IndexMap::new(),
        }
    }

    pub fn create_rect(&mut self, pos: Vector2<f32>, size: Vector2<f32>, color: Color) -> IndexToken {
        self.frame
            .create_quad
            .push(CreateQuadMessage::new(Quad::new(
                ShapeVertex::new(pos.x, pos.y + size.y, color),
                ShapeVertex::new(pos.x, pos.y, color),
                ShapeVertex::new(pos.x + size.x, pos.y + size.y, color),
                ShapeVertex::new(pos.x + size.x, pos.y, color),
            )));
        self.map_rect.add()
    }

    pub fn create_triangle(&mut self, triangle: Triangle<ShapeVertex>) -> IndexToken {
        self.frame
            .create_triangle
            .push(CreateTriangleMessage::new(triangle));
        self.map_triangle.add()
    }

    pub fn set_translation(&mut self, translation: Vector3<f32>) {
        self.frame.translation = SetTranslationMessage::new(true, translation);
    }

    pub fn send(&mut self) {
        let mut frame = RenderFrame::new();
        mem::swap(&mut frame, &mut self.frame);
        self.frame_producer.push(frame);
    }
}
