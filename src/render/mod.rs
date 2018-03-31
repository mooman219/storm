pub mod buffer;
pub mod color;
pub mod display;
pub mod enums;
pub mod geometry;
pub mod message;
pub mod shader;
pub mod vertex;

use bounded_spsc_queue;
use cgmath::*;
use gl;
use render::buffer::geometry::*;
use render::display::*;
use render::geometry::quad::*;
use render::geometry::triangle::*;
use render::geometry::*;
use render::message::*;
use render::shader::color::*;
use render::vertex::color::*;
use std::thread::sleep;
use std::time::Duration;
use time::timer::*;
use utility::consume_spsc;

struct RenderState {
    display: Display,
    shape_shader: ColorShader,
    triangle_buffer: GeometryBuffer<Triangle<ColorVertex>>,
    quad_buffer: GeometryBuffer<Quad<ColorVertex>>,
}

pub fn start(
    mut display: Display,
    render_consumer: bounded_spsc_queue::Consumer<RenderFrame>,
    resize_consumer: consume_spsc::Consumer<Vector2<u32>>,
) {
    // Initialize the display. The display is bound in the thread we're going to be making opengl
    // calls in. Behavior is undefined is the display is bound outside of the thread and usually
    // segfaults.
    display.bind();
    display.enable_clear_color();
    display.clear_color(0.0, 0.0, 0.2, 1.0);

    // Create the render state.
    let mut state = RenderState {
        display: display,
        shape_shader: ColorShader::new(),
        triangle_buffer: Triangle::new_geometry_buffer(50),
        quad_buffer: Quad::new_geometry_buffer(100),
    };

    // Log render timings.
    let mut timer_render = Timer::new("Render: Frame");
    loop {
        // Resizing.
        state.resize(resize_consumer.consume());
        // Frame processing.
        match render_consumer.try_pop().as_mut() {
            Some(f) => {
                // Start timing.
                timer_render.start();
                // Clear the screen.
                state.display.clear();
                // Message quads.
                state.handle_quads(&mut f.quads);
                state.quad_buffer.sync();
                // Message triangles.
                state.handle_triangles(&mut f.triangles);
                state.triangle_buffer.sync();
                // Message shader.
                state.shape_shader.bind();
                state.handle_set_translation(f.translation);
                state.handle_set_scale(f.scale);
                // Draw shapes.
                state.shape_shader.bind();
                state.quad_buffer.draw();
                state.triangle_buffer.draw();
                // Finish timing.
                timer_render.stop();
                // Finish.
                state.display.swap_buffers();
            },
            None => {},
        }
        // Sleep to avoid pegging a core.
        sleep(Duration::new(0, 100));
    }
}

impl RenderState {
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

    fn resize(&mut self, message: Option<Vector2<u32>>) {
        match message {
            Some(msg) => unsafe {
                self.display.resize(msg.x, msg.y);
                gl::Viewport(0, 0, msg.x as i32, msg.y as i32);
                self.shape_shader.bind();
                self.shape_shader.set_bounds(msg.x as f32, msg.y as f32);
            },
            None => {},
        }
    }
}
