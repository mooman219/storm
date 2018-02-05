use bounded_spsc_queue::Consumer;
use bounded_spsc_queue::Producer;
use render::message::RenderFrame;
use std::mem;
use render::buffer::geometry::*;
use render::display::*;
use render::geometry::*;
use render::geometry::quad::*;
use render::geometry::triangle::*;
use render::message::*;
use render::vertex::shape::*;
use render::shader::shape::*;

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

    pub fn create_quad(&mut self, message: CreateQuadMessage) {
        self.frame.create_quad.push(message);
    }

    pub fn create_triangle(&mut self, message: CreateTriangleMessage) {
        self.frame.create_triangle.push(message);
    }

    pub fn send(&mut self) {
        let mut frame = RenderFrame::new();
        mem::swap(&mut frame, &mut self.frame);
        self.frame_producer.push(frame);
    }
}

pub struct RenderConsumer {
    display: Display,
    frame_consumer: Consumer<RenderFrame>,
    shape_shader: ShapeShader,
    triangle_buffer: GeometryBuffer<Triangle<ShapeVertex>>,
    quad_buffer: GeometryBuffer<Quad<ShapeVertex>>,
}

impl RenderConsumer {
    pub fn new(mut display: Display, frame_consumer: Consumer<RenderFrame>) -> RenderConsumer {
        // Initialization
        display.enable_clear_color();
        display.clear_color(0.0, 0.0, 0.0, 1.0);
        println!("Info: OpenGL version {}", display.get_version_string());

        // Return the composed consumer
        RenderConsumer {
            display: display,
            frame_consumer: frame_consumer,
            shape_shader: ShapeShader::new(),
            triangle_buffer: Triangle::new_geometry_buffer(),
            quad_buffer: Quad::new_geometry_buffer(),
        }
    }

    pub fn handle_create_quad(&mut self, messages: &mut Vec<CreateQuadMessage>) {
        for message in messages.drain(..) {
            self.quad_buffer.add(message.quad);
        }
        self.quad_buffer.sync();
    }

    pub fn handle_create_triangle(&mut self, messages: &mut Vec<CreateTriangleMessage>) {
        for message in messages.drain(..) {
            self.triangle_buffer.add(message.triangle);
        }
        self.triangle_buffer.sync();
    }

    pub fn tick(&mut self) {
        // Frame processing
        match self.frame_consumer.try_pop().as_mut() {
            Some(f) => {
                self.handle_create_quad(&mut f.create_quad);
                self.handle_create_triangle(&mut f.create_triangle);
            },
            None => {},
        }

        // Shapes
        self.shape_shader.bind();
        self.quad_buffer.draw();
        self.triangle_buffer.draw();

        // Finish
        self.display.swap_buffers();
        self.display.clear();
    }
}
