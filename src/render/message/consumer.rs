use bounded_spsc_queue::Consumer;
use render::buffer::geometry::*;
use render::display::*;
use render::geometry::*;
use render::geometry::quad::*;
use render::geometry::triangle::*;
use render::message::*;
use render::message::RenderFrame;
use render::shader::shape::*;
use render::vertex::shape::*;
use time::timer::*;

pub struct RenderConsumer {
    display: Display,
    frame_consumer: Consumer<RenderFrame>,
    shape_shader: ShapeShader,
    triangle_buffer: GeometryBuffer<Triangle<ShapeVertex>>,
    quad_buffer: GeometryBuffer<Quad<ShapeVertex>>,
    timer_render: Timer,
}

impl RenderConsumer {
    pub fn new(display: Display, frame_consumer: Consumer<RenderFrame>) -> RenderConsumer {
        // Get the composed consumer
        let mut consumer = RenderConsumer {
            display: display,
            frame_consumer: frame_consumer,
            shape_shader: ShapeShader::new(),
            triangle_buffer: Triangle::new_geometry_buffer(50),
            quad_buffer: Quad::new_geometry_buffer(100),
            timer_render: Timer::new("Frame"),
        };
        // Initialize it
        consumer.initialize();
        // Return
        consumer
    }

    fn initialize(&mut self) {
        self.display.enable_clear_color();
        self.display.clear_color(0.0, 0.0, 0.0, 1.0);
        self.shape_shader.bind();
        self.shape_shader.set_scale(0.5f32);
        println!(
            "Render: OpenGL version {}",
            self.display.get_version_string()
        );
    }

    pub fn handle_create_quad(&mut self, messages: &mut Vec<CreateQuadMessage>) {
        for message in messages.drain(..) {
            self.quad_buffer.add(message.quad);
        }
    }

    pub fn handle_create_triangle(&mut self, messages: &mut Vec<CreateTriangleMessage>) {
        for message in messages.drain(..) {
            self.triangle_buffer.add(message.triangle);
        }
    }

    pub fn handle_set_translation(&mut self, message: &SetTranslationMessage) {
        if message.set {
            self.shape_shader.set_translation(message.translation);
        }
    }

    pub fn tick(&mut self) {
        // Frame processing
        match self.frame_consumer.try_pop().as_mut() {
            Some(f) => {
                self.timer_render.start();

                // Message Quads
                self.handle_create_quad(&mut f.create_quad);
                self.quad_buffer.sync();
                // Message Triangles
                self.handle_create_triangle(&mut f.create_triangle);
                self.triangle_buffer.sync();
                // Message Shader
                self.shape_shader.bind();
                self.handle_set_translation(&f.translation);
                // Draw Shapes
                self.shape_shader.bind();
                self.quad_buffer.draw();
                self.triangle_buffer.draw();
                // Finish
                self.display.swap_buffers();
                self.display.clear();

                self.timer_render.stop();
            },
            None => {},
        }
    }
}
