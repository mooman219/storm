use bounded_spsc_queue;
use gl;
use input::*;
use render::buffer::geometry::*;
use render::display::*;
use render::geometry::*;
use render::geometry::quad::*;
use render::geometry::triangle::*;
use render::message::*;
use render::shader::shape::*;
use render::vertex::shape::*;
use time::timer::*;
use utility::consume_spsc;

pub struct RenderConsumer {
    display: Display,
    render_consumer: bounded_spsc_queue::Consumer<RenderFrame>,
    resize_consumer: consume_spsc::Consumer<ResizeMessage>,
    shape_shader: ShapeShader,
    triangle_buffer: GeometryBuffer<Triangle<ShapeVertex>>,
    quad_buffer: GeometryBuffer<Quad<ShapeVertex>>,
    timer_render: Timer,
}

impl RenderConsumer {
    pub fn new(
        display: Display,
        render_consumer: bounded_spsc_queue::Consumer<RenderFrame>,
        resize_consumer: consume_spsc::Consumer<ResizeMessage>,
    ) -> RenderConsumer {
        // Get the composed consumer
        let mut consumer = RenderConsumer {
            display: display,
            render_consumer: render_consumer,
            resize_consumer: resize_consumer,
            shape_shader: ShapeShader::new(),
            triangle_buffer: Triangle::new_geometry_buffer(50),
            quad_buffer: Quad::new_geometry_buffer(100),
            timer_render: Timer::new("Frame"),
        };
        // Initialize it
        consumer.display.enable_clear_color();
        consumer.display.clear_color(0.0, 0.0, 0.2, 1.0);
        // Return
        consumer
    }

    fn handle_quads(&mut self, messages: &mut Vec<QuadMessage>) {
        for message in messages.drain(..) {
            match message {
                QuadMessage::Create { pos, size, color } => {
                    let quad = Quad::new_rect(pos, size, color);
                    self.quad_buffer.add(quad);
                },
                QuadMessage::Update { id, pos, size, color } => {
                    let quad = Quad::new_rect(pos, size, color);
                    self.quad_buffer.update(id, quad);
                },
                QuadMessage::Remove { id } => {
                    self.quad_buffer.remove(id);
                },
            }
        }
    }

    fn handle_triangles(&mut self, messages: &mut Vec<TriangleMessage>) {
        for message in messages.drain(..) {
            match message {
                TriangleMessage::Create { pos, height, color } => {
                    let triangle = Triangle::new_iso(pos, height, color);
                    self.triangle_buffer.add(triangle);
                },
                TriangleMessage::Update {
                    id,
                    pos,
                    height,
                    color,
                } => {
                    let triangle = Triangle::new_iso(pos, height, color);
                    self.triangle_buffer.update(id, triangle);
                },
                TriangleMessage::Remove { id } => {
                    self.triangle_buffer.remove(id);
                },
            }
        }
    }

    fn handle_set_translation(&mut self, message: Option<Vector2<f32>>) {
        match message {
            Some(translation) => {
                self.shape_shader.set_translation(translation);
            },
            None => {},
        };
    }

    fn handle_set_scale(&mut self, message: Option<f32>) {
        match message {
            Some(scale) => {
                self.shape_shader.set_scale(scale);
            },
            None => {},
        };
    }

    pub fn resize(&mut self, message: Option<ResizeMessage>) {
        match message {
            Some(msg) => unsafe {
                self.display.resize(msg.width, msg.height);
                gl::Viewport(0, 0, msg.width as i32, msg.height as i32);
                self.shape_shader.bind();
                self.shape_shader.set_bounds(msg.width as f32, msg.height as f32);
            },
            None => {},
        }
    }

    pub fn tick(&mut self) {
        // Resizing
        let message = self.resize_consumer.consume();
        self.resize(message);
        // Frame processing
        match self.render_consumer.try_pop().as_mut() {
            Some(f) => {
                self.timer_render.start();

                // Message Quads
                self.handle_quads(&mut f.quads);
                self.quad_buffer.sync();
                // Message Triangles
                self.handle_triangles(&mut f.triangles);
                self.triangle_buffer.sync();
                // Message Shader
                self.shape_shader.bind();
                self.handle_set_translation(f.translation);
                self.handle_set_scale(f.scale);
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
