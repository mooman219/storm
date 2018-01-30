pub mod frame;

use bounded_spsc_queue::Consumer;
use bounded_spsc_queue::Producer;
use render::message::frame::RenderFrame;
use std::mem;
use render::buffer::shape_buffer::ShapeBuffer;
use render::shape::*;
use render::shape::quad::*;
use render::shape::triangle::*;
use render::vertex::pos2::*;

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

    pub fn create_quad(&mut self) -> u32 {
        0u32
    }

    pub fn send(&mut self) {
        let mut frame = RenderFrame::new();
        mem::swap(&mut frame, &mut self.frame);
        self.frame_producer.push(frame);
    }
}

pub struct RenderConsumer {
    frame_consumer: Consumer<RenderFrame>,
    triangle_buffer: ShapeBuffer<Triangle<Pos2Vertex>>,
    quad_buffer: ShapeBuffer<Quad<Pos2Vertex>>,
}

impl RenderConsumer {
    pub fn new(frame_consumer: Consumer<RenderFrame>) -> RenderConsumer {
        RenderConsumer {
            frame_consumer: frame_consumer,
            triangle_buffer: Triangle::new_shape_buffer(),
            quad_buffer: Quad::new_shape_buffer(),
        }
    }

    pub fn tick(&mut self) {
        let frame = self.frame_consumer.try_pop();
    }
}
