mod buffer;
mod display;
mod manager;
mod message;
mod raw;
mod shader;
mod texture;
mod vertex;

pub use render::display::*;
pub use render::manager::*;
pub use render::message::*;

use cgmath::*;
use channel::bounded_spsc;
use color;
use layer::*;
use render::buffer::geometry::*;
use render::raw::*;
use render::shader::*;
use render::texture::*;
use render::vertex::*;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use time::*;

struct Layer {
    desc: LayerDescription,
    sprites: GeometryBuffer<TextureVertex>,
}

struct RenderState {
    display: Display,
    shader_texture: TextureShader,
    layers: Vec<Layer>,
    texture_packer: TexturePacker,
    texture_atlas: TextureHandle,
    texture_uv: Vec<Vector4<f32>>,
    current_size: Vector2<f64>,
}

pub fn start(display: Display, render_consumer: bounded_spsc::Consumer<Vec<RenderMessage>>) {
    // Initialize the display. The display is bound in the thread we're going to be making opengl
    // calls in. Behavior is undefined is the display is bound outside of the thread and usually
    // segfaults.
    display.bind();

    // Create the render state.
    let current_size = display.get_size();
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
        current_size: current_size,
    };

    // Initial setup of the render state.
    state.setup();

    // Log render timings.
    let mut timer_render = Timer::new("[R] Frame");
    loop {
        // Frame processing.
        match render_consumer.try_pop().as_mut() {
            Some(mut messages) => {
                timer_render.start();
                clear(ClearBit::ColorBuffer | ClearBit::DepthBuffer);
                state.resize();
                state.handle_messages(&mut messages);

                for layer in &mut state.layers {
                    if layer.desc.visible {
                        state.shader_texture.set_scale(layer.desc.scale);
                        state.shader_texture.set_translation(layer.desc.translation);
                        state.shader_texture.sync_ortho();
                        layer.sprites.draw();
                    }
                }

                state.display.swap_buffers();
                timer_render.stop();
            },
            None => {},
        }
        // todo: Use Condvar and wait until alerterd to start rendering.
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
            // enable(Capability::Blend);
            enable(Capability::DepthTest);
            // blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
            clear_color(0.0, 0.0, 0.2, 1.0);
            depth_func(DepthTest::Less);
            cull_face(CullFace::Back);
        }

        // Setup the default texture.
        self.shader_texture.bind();
        self.shader_texture.sync_ortho();
        self.shader_texture.sync_atlas();
    }

    fn handle_messages(&mut self, messages: &mut Vec<RenderMessage>) {
        let mut texture_dirty = false;
        for message in messages.drain(..) {
            match message {
                // Layer
                RenderMessage::LayerCreate { layer, desc } => {
                    let slot = Layer {
                        desc: desc,
                        sprites: GeometryBuffer::new(1024),
                    };
                    self.layers.insert(layer, slot)
                },
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
                    let quad = TextureVertex::new(desc.pos, desc.size, uv, desc.color);
                    trace!("{:?}", quad);
                    self.layers[layer].sprites.add(quad);
                },
                RenderMessage::SpriteUpdate { layer, sprite, desc } => {
                    let uv = self.texture_uv[desc.texture.key()];
                    let quad = TextureVertex::new(desc.pos, desc.size, uv, desc.color);
                    self.layers[layer].sprites.update(sprite, quad);
                },
                RenderMessage::SpriteRemove { layer, sprite } => {
                    self.layers[layer].sprites.remove(sprite);
                },

                // Texture
                RenderMessage::TextureLoad { path } => {
                    let uv = self.texture_packer.pack_path(Path::new(&path));
                    self.texture_uv.push(uv);
                    texture_dirty = true;
                },

                // Window
                RenderMessage::WindowTitle { title } => {
                    self.display.set_title(title.as_str());
                },
            }
        }

        // Sync textures if updated.
        if texture_dirty {
            let new_atlas = self.texture_packer.export();
            self.texture_atlas.set_texture(&new_atlas);
        }
        // Sync visible layers.
        for layer in &mut self.layers {
            if layer.desc.visible {
                layer.sprites.sync();
            }
        }
    }

    fn resize(&mut self) {
        let new_size = self.display.get_size();
        if self.current_size != new_size {
            self.current_size = new_size;
            self.display.resize(new_size);
            self.shader_texture.set_bounds(new_size.x as f32, new_size.y as f32);
        }
    }
}
