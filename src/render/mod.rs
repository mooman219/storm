pub mod buffer;
pub mod color;
pub mod display;
pub mod geometry;
pub mod message;
pub mod packer;
pub mod raw;
pub mod shader;
pub mod texture;
pub mod vertex;

use bounded_spsc_queue;
use cgmath::*;
use channel::consume_spsc;
use image::*;
use render::buffer::geometry::*;
use render::display::*;
use render::geometry::*;
use render::message::*;
use render::raw::*;
use render::shader::*;
use render::texture::*;
use render::vertex::*;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use time::timer::*;

struct RenderState {
    display: Display,
    shader_texture: TextureShader,
    quad_texture: GeometryBuffer<Quad<TextureVertex>>,
    texture_atlas: TextureGl,
}

pub fn start(
    display: Display,
    render_consumer: bounded_spsc_queue::Consumer<RenderFrame>,
    resize_consumer: consume_spsc::Consumer<Vector2<f64>>,
) {
    // Initialize the display. The display is bound in the thread we're going to be making opengl
    // calls in. Behavior is undefined is the display is bound outside of the thread and usually
    // segfaults.
    display.bind();
    enable(Capability::DepthTest);
    enable(Capability::CullFace);
    enable(Capability::Multisample);
    clear_color(0.0, 0.0, 0.2, 1.0);
    depth_func(DepthTest::LessEqual);
    cull_face(CullFace::Back);

    // Create the render state.
    let mut state = RenderState {
        display: display,
        shader_texture: TextureShader::new(),
        quad_texture: Quad::new_geometry_buffer(2500),
        texture_atlas: TextureGl::new(TextureUnit::Atlas),
    };
    // Set the default texture.
    state.shader_texture.set_texture_unit(TextureUnit::Atlas);

    // Log render timings.
    let mut timer_render = Timer::new("[R] Frame");
    loop {
        // Frame processing.
        match render_consumer.try_pop().as_mut() {
            Some(f) => {
                // Start timing.
                timer_render.start();
                // Clear the screen.
                clear(ClearBit::ColorBuffer | ClearBit::DepthBuffer);
                // Resizing.
                state.resize(resize_consumer.consume());
                // Message handling.
                state.handle_messages(&mut f.messages);
                // Draw shapes.
                state.shader_texture.bind();
                state.quad_texture.draw();
                // Finish.
                state.display.swap_buffers();
                // Finish timing.
                timer_render.stop();
            },
            None => {},
        }
        // Sleep to avoid pegging a core.
        sleep(Duration::new(0, 100));
    }
}

impl RenderState {
    fn handle_messages(&mut self, messages: &mut Vec<RenderMessage>) {
        let mut shader_modified = false;
        let mut geometry_modified = false;
        for message in messages.drain(..) {
            match message {
                // Quads
                RenderMessage::QuadCreate { pos, size, color } => {
                    let quad = Quad::texture_rect(pos, size, color);
                    self.quad_texture.add(quad);
                    geometry_modified = true;
                },
                RenderMessage::QuadUpdate { id, pos, size, color } => {
                    let quad = Quad::texture_rect(pos, size, color);
                    self.quad_texture.update(id, quad);
                    geometry_modified = true;
                },
                RenderMessage::QuadRemove { id } => {
                    self.quad_texture.remove(id);
                    geometry_modified = true;
                },
                RenderMessage::CreateTexture { path } => match open(Path::new(&path)) {
                    Ok(image) => {
                        self.texture_atlas.set_image(image);
                    },
                    Err(..) => {
                        panic!("Unable to set image as atlas: {}", &path);
                    },
                },
                RenderMessage::Translate { pos } => {
                    self.shader_texture.set_translation(pos);
                    shader_modified = true;
                },
                RenderMessage::Scale { factor } => {
                    self.shader_texture.set_scale(factor);
                    shader_modified = true;
                },
            }
        }
        if geometry_modified {
            self.quad_texture.sync();
        }
        if shader_modified {
            self.shader_texture.sync();
        }
    }

    fn resize(&mut self, message: Option<Vector2<f64>>) {
        match message {
            Some(msg) => {
                self.display.resize(msg);
                self.shader_texture.bind();
                self.shader_texture.set_bounds(msg.x as f32, msg.y as f32);
            },
            None => {},
        }
    }
}
