use cgmath::*;
use color;
use layer::*;
use render::buffer::geometry::*;
use render::raw::*;
use render::shader::*;
use render::texture::*;
use render::vertex::*;
use render::*;
use std::path::Path;
use time::*;
use utility::bounded_spsc;
use utility::control;

struct Layer {
    desc: LayerDescription,
    sprites: GeometryBuffer<TextureVertex>,
}

pub struct RenderServer {
    display: Display,
    render_consumer: bounded_spsc::Consumer<Vec<RenderMessage>>,
    render_control: control::Consumer,

    shader_texture: TextureShader,
    layers: Vec<Layer>,
    texture_packer: TexturePacker,
    texture_atlas: TextureHandle,
    texture_uv: Vec<Vector4<f32>>,
    current_size: Vector2<f64>,
}

impl RenderServer {
    pub fn new(
        display: Display,
        render_consumer: bounded_spsc::Consumer<Vec<RenderMessage>>,
        render_control: control::Consumer,
    ) -> RenderServer {
        // Initialize the display. The display is bound in the thread we're going to be making opengl
        // calls in. Behavior is undefined is the display is bound outside of the thread and usually
        // segfaults.
        display.bind();

        let current_size = display.get_size();
        let mut state = RenderServer {
            display: display,
            render_consumer: render_consumer,
            render_control: render_control,
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

        // Setup cabilities.
        enable(Capability::CullFace);
        enable(Capability::Multisample);
        enable(Capability::DepthTest);
        clear_color(0.0, 0.0, 0.2, 1.0);
        depth_func(DepthTest::Less);
        cull_face(CullFace::Back);

        // Default texture setup
        state.texture_add(Texture::from_default(color::WHITE, 8, 8));
        state.texture_sync();

        // Setup the default texture.
        state.shader_texture.bind();
        state.shader_texture.sync_ortho();
        state.shader_texture.sync_atlas();

        state
    }

    pub fn texture_add(&mut self, texture: Texture) {
        let uv = self.texture_packer.pack(&texture);
        self.texture_uv.push(uv);
    }

    pub fn texture_add_path(&mut self, path: String) {
        let uv = self.texture_packer.pack_path(Path::new(&path));
        self.texture_uv.push(uv);
    }

    pub fn texture_sync(&mut self) {
        let new_atlas = self.texture_packer.export();
        self.texture_atlas.set_texture(&new_atlas);
    }

    pub fn draw(&mut self) {
        clear(ClearBit::ColorBuffer);
        for layer in &mut self.layers {
            if layer.desc.visible {
                clear(ClearBit::DepthBuffer);
                self.shader_texture.set_scale(layer.desc.scale);
                self.shader_texture.set_translation(layer.desc.translation);
                self.shader_texture.sync_ortho();
                layer.sprites.draw();
            }
        }
        self.display.swap_buffers();
    }

    pub fn resize(&mut self) {
        let new_size = self.display.get_size();
        if self.current_size != new_size {
            self.current_size = new_size;
            self.display.resize(new_size);
            self.shader_texture.set_bounds(new_size.x as f32, new_size.y as f32);
        }
    }

    pub fn run_forever(&mut self) {
        let mut timer_render = Timer::new("[R] Frame");
        loop {
            match self.render_consumer.try_pop().as_mut() {
                Some(mut messages) => {
                    timer_render.start();
                    self.resize();
                    self.handle_messages(&mut messages);
                    self.draw();
                    timer_render.stop();
                },
                None => {
                    self.render_control.wait();
                },
            }
        }
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
                    self.texture_add_path(path);
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
            self.texture_sync();
        }
        // Sync visible layers.
        for layer in &mut self.layers {
            if layer.desc.visible {
                layer.sprites.sync();
            }
        }
    }
}
