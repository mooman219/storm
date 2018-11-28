pub mod buffer;
pub mod color;
pub mod display;
pub mod geometry;
pub mod message;
pub mod raw;
pub mod shader;
pub mod texture;
pub mod vertex;

use cgmath::*;
use channel::bounded_spsc;
use channel::consume_spsc;
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
    texture_packer: TexturePacker,
    texture_atlas: TextureHandle,
    texture_uv: Vec<Vector4<f32>>,
}

pub fn start(
    display: Display,
    render_consumer: bounded_spsc::Consumer<RenderFrame>,
    resize_consumer: consume_spsc::Consumer<Vector2<f64>>,
) {
    // Initialize the display. The display is bound in the thread we're going to be making opengl
    // calls in. Behavior is undefined is the display is bound outside of the thread and usually
    // segfaults.
    display.bind();

    // Create the render state.
    let mut state = RenderState {
        display: display,
        shader_texture: TextureShader::new(),
        quad_texture: Quad::new_geometry_buffer(2500),
        texture_packer: TexturePacker::new(TexturePackerConfig {
            max_width: 2048,
            max_height: 2048,
            texture_padding: 0,
        }),
        texture_atlas: TextureHandle::new(TextureUnit::Atlas),
        texture_uv: Vec::new(),
    };

    // Default texture setup
    {
        let texture = Texture::from_default(color::WHITE, 8, 8);
        let uv = state.texture_packer.pack(&texture);
        state.texture_uv.push(uv);
        let new_atlas = state.texture_packer.export();
        state.texture_atlas.set_texture(&new_atlas);
    }

    // Setup cabilities.
    enable(Capability::CullFace);
    enable(Capability::Multisample);
    enable(Capability::Blend);
    blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
    clear_color(0.0, 0.0, 0.2, 1.0);
    cull_face(CullFace::Back);

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
        for message in messages.drain(..) {
            match message {
                //
                // Quad
                //
                RenderMessage::QuadCreate {
                    pos,
                    size,
                    color,
                    texture,
                } => {
                    let uv = self.texture_uv[texture];
                    let quad = Quad::texture_rect(pos, size, uv, color);
                    self.quad_texture.add(quad);
                },
                RenderMessage::QuadUpdate {
                    id,
                    pos,
                    size,
                    color,
                    texture,
                } => {
                    let uv = self.texture_uv[texture];
                    let quad = Quad::texture_rect(pos, size, uv, color);
                    self.quad_texture.update(id, quad);
                },
                RenderMessage::QuadRemove { id } => {
                    self.quad_texture.remove(id);
                },
                RenderMessage::QuadClear {} => {
                    self.quad_texture.clear();
                },
                //
                // Texture
                //
                RenderMessage::TextureCreate { path } => {
                    let uv = self.texture_packer.pack_path(Path::new(&path));
                    self.texture_uv.push(uv);
                    let new_atlas = self.texture_packer.export();
                    self.texture_atlas.set_texture(&new_atlas);

                    // TODO: Temp debug output
                    let exporter = new_atlas.to_dynamic_image().unwrap();
                    let mut file = std::fs::File::create("examples/testgame/output.png").unwrap();
                    exporter.write_to(&mut file, image::PNG).unwrap();
                },
                //
                // Scene
                //
                RenderMessage::Translate { pos } => {
                    self.shader_texture.set_translation(pos);
                },
                RenderMessage::Scale { factor } => {
                    self.shader_texture.set_scale(factor);
                },
                //
                // Window
                //
                RenderMessage::WindowTitle { title } => {
                    self.display.set_title(title.as_str());
                },
            }
        }
        self.quad_texture.sync();
        self.shader_texture.sync();
    }

    fn resize(&mut self, message: Option<Vector2<f64>>) {
        match message {
            Some(msg) => {
                self.display.resize(msg);
                self.shader_texture.set_bounds(msg.x as f32, msg.y as f32);
            },
            None => {},
        }
    }
}
