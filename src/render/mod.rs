pub mod buffer;
pub mod color;
pub mod display;
pub mod geometry;
pub mod raw;
pub mod shader;
pub mod texture;
pub mod vertex;

use cgmath::*;
use channel::bounded_spsc;
use channel::consume_spsc;
use layer::*;
use message::*;
use render::buffer::geometry::*;
use render::display::*;
use render::geometry::*;
use render::raw::*;
use render::shader::*;
use render::texture::*;
use render::vertex::*;
use sprite::*;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use time::timer::*;

struct Layer {
    desc: LayerDescription,
    sprites: GeometryBuffer<Quad<TextureVertex>>,
}

struct RenderState {
    display: Display,
    shader_texture: TextureShader,
    layers: Vec<Layer>,
    texture_packer: TexturePacker,
    texture_atlas: TextureHandle,
    texture_uv: Vec<Vector4<f32>>,
}

pub fn start(
    display: Display,
    render_consumer: bounded_spsc::Consumer<Vec<RenderMessage>>,
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
        layers: Vec::new(),
        texture_packer: TexturePacker::new(TexturePackerConfig {
            max_width: 2048,
            max_height: 2048,
            texture_padding: 0,
        }),
        texture_atlas: TextureHandle::new(TextureUnit::Atlas),
        texture_uv: Vec::new(),
    };

    // Initial setup of the render state.
    state.setup();

    // Log render timings.
    let mut timer_render = Timer::new("[R] Frame");
    loop {
        // Frame processing.
        match render_consumer.try_pop().as_mut() {
            Some(mut messages) => {
                // Start timing.
                timer_render.start();
                // Clear the screen.
                clear(ClearBit::ColorBuffer | ClearBit::DepthBuffer);
                // Resizing.
                state.resize(resize_consumer.consume());
                // Message handling.
                state.handle_messages(&mut messages);
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
    fn setup(&mut self) {
        // Default texture setup
        {
            let texture = Texture::from_default(color::WHITE, 8, 8);
            let uv = self.texture_packer.pack(&texture);
            self.texture_uv.push(uv);
            let new_atlas = self.texture_packer.export();
            self.texture_atlas.set_texture(&new_atlas);
        }

        // Setup cabilities.
        {
            enable(Capability::CullFace);
            enable(Capability::Multisample);
            enable(Capability::Blend);
            blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
            clear_color(0.0, 0.0, 0.2, 1.0);
            cull_face(CullFace::Back);
        }

        // Setup the default texture.
        self.shader_texture.set_texture_unit(TextureUnit::Atlas);
    }

    fn handle_messages(&mut self, messages: &mut Vec<RenderMessage>) {
        for message in messages.drain(..) {
            match message {
                // Layer
                RenderMessage::LayerCreate { layer, desc } => self.layers.insert(
                    layer,
                    Layer {
                        desc: desc,
                        sprites: Quad::new_geometry_buffer(1024),
                    },
                ),
                RenderMessage::LayerUpdate { layer, desc } => {
                    self.layers[layer].desc = desc;
                },
                RenderMessage::LayerRemove { layer } => {
                    self.layers.remove(layer);
                },
                RenderMessage::LayerClear { layer } => {
                    self.layers[layer].sprites.clear();
                },

                // Sprite
                RenderMessage::SpriteCreate { layer, desc } => {
                    let uv = self.texture_uv[desc.texture.key()];
                    let quad = Quad::texture_rect(desc.pos, desc.size, uv, desc.color);
                    self.layers[layer].sprites.add(quad);
                },
                RenderMessage::SpriteUpdate { layer, sprite, desc } => {
                    let uv = self.texture_uv[desc.texture.key()];
                    let quad = Quad::texture_rect(desc.pos, desc.size, uv, desc.color);
                    self.layers[layer].sprites.update(sprite, quad);
                },
                RenderMessage::SpriteRemove { layer, sprite } => {
                    self.layers[layer].sprites.remove(sprite);
                },

                // Texture
                RenderMessage::TextureLoad { path } => {
                    let uv = self.texture_packer.pack_path(Path::new(&path));
                    self.texture_uv.push(uv);
                    let new_atlas = self.texture_packer.export();
                    self.texture_atlas.set_texture(&new_atlas);
                    // todo: Performance improvement by marking textures as dirty and setting it last.
                },

                // Window
                RenderMessage::WindowTitle { title } => {
                    self.display.set_title(title.as_str());
                },
            }
        }
        for layer in &self.layers {
            if layer.desc.visible {
                layer.sprites.sync();
            }
        }
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
