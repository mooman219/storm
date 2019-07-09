use crate::color::RGBA8;
use crate::render::gl::buffer::*;
use crate::render::gl::raw::*;
use crate::render::gl::shader::*;
use crate::render::gl::texture_handle::*;
use crate::render::gl::window::*;
use crate::texture::*;
use crate::types::*;
use beryllium::SDLToken;
use cgmath::*;

struct Batch {
    desc: BatchSettings,
    sprites: Buffer<Sprite>,
    strings: Buffer<Sprite>,
    matrix_translate_scaled: Matrix4<f32>,
    matrix_full: Matrix4<f32>,
}

pub struct OpenGLState {
    window: OpenGLWindow,
    shader: TextureShader,
    texture_atlas: TextureHandle,
    texture_font: TextureHandle,
    batches: Vec<Batch>,
    matrix_bounds: Matrix4<f32>,
    current_logical_size: Vector2<f32>,
}

fn matrix_from_bounds(bounds: &Vector2<f32>) -> Matrix4<f32> {
    let w = bounds.x / 2.0;
    let h = bounds.y / 2.0;
    ortho(-w.floor(), w.ceil(), -h.floor(), h.ceil(), -1.0, 1.0)
}

fn matrix_from_translate_scaled(translation: &Vector2<f32>, scale: f32) -> Matrix4<f32> {
    Matrix4::from_translation(translation.extend(0.0)) * Matrix4::from_scale(scale)
}

impl OpenGLState {
    pub fn new(desc: &WindowSettings, sdl: &SDLToken) -> OpenGLState {
        let window = OpenGLWindow::new(desc, sdl);
        let logical_size = window.logical_size();
        let state = OpenGLState {
            window: window,
            shader: TextureShader::new(),
            texture_atlas: TextureHandle::new(TextureUnit::Atlas),
            texture_font: TextureHandle::new(TextureUnit::Font),
            batches: Vec::new(),
            matrix_bounds: matrix_from_bounds(&logical_size),
            current_logical_size: logical_size,
        };
        // Bind shader once.
        state.shader.bind();
        // Setup cabilities.
        enable(Capability::CullFace);
        enable(Capability::Blend);
        enable(Capability::DepthTest);
        clear_color(0.0, 0.0, 0.0, 1.0);
        depth_func(DepthTest::Less);
        blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        cull_face(CullFace::Back);
        // State is setup.
        state
    }

    pub fn window_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn window_clear_color(&mut self, color: RGBA8) {
        let color: Vector4<f32> = color.into();
        clear_color(color.x, color.y, color.z, color.w);
    }

    pub fn upload_texture_atlas(&mut self, texture: &Image) {
        self.texture_atlas.set_texture(texture);
    }

    pub fn upload_font_atlas(&mut self, texture: &Image) {
        self.texture_font.set_texture(texture);
    }

    pub fn batch_create(&mut self, desc: &BatchSettings) {
        let matrix_translate_scaled = matrix_from_translate_scaled(&desc.translation, desc.scale);
        let matrix_full = self.matrix_bounds * matrix_translate_scaled;
        self.batches.push(Batch {
            desc: *desc,
            sprites: Buffer::new(BufferBindingTarget::ArrayBuffer),
            strings: Buffer::new(BufferBindingTarget::ArrayBuffer),
            matrix_translate_scaled: matrix_translate_scaled,
            matrix_full: matrix_full,
        });
    }

    pub fn batch_update(&mut self, index: usize, desc: &BatchSettings) {
        let batch = &mut self.batches[index];
        let matrix_translate_scaled = matrix_from_translate_scaled(&desc.translation, desc.scale);
        let matrix_full = self.matrix_bounds * matrix_translate_scaled;
        batch.desc = *desc;
        batch.matrix_translate_scaled = matrix_translate_scaled;
        batch.matrix_full = matrix_full;
    }

    pub fn batch_sprite_set(&mut self, index: usize, quads: &Vec<Sprite>) {
        self.batches[index].sprites.set(quads);
    }

    pub fn batch_string_set(&mut self, index: usize, quads: &Vec<Sprite>) {
        self.batches[index].strings.set(quads);
    }

    pub fn batch_remove(&mut self, index: usize) {
        self.batches.swap_remove(index);
    }

    /// Helper function to resize the window.
    fn resize(&mut self) {
        let new_logical_size = self.window.logical_size();
        if self.current_logical_size != new_logical_size {
            self.current_logical_size = new_logical_size;
            let new_physical_size = self.window.physical_size();
            viewport(0, 0, new_physical_size.x as i32, new_physical_size.y as i32);
            self.matrix_bounds = matrix_from_bounds(&new_logical_size);
            for batch in &mut self.batches {
                batch.matrix_full = self.matrix_bounds * batch.matrix_translate_scaled;
            }
        }
    }

    pub fn draw(&mut self) {
        self.resize();
        clear(ClearBit::ColorBuffer | ClearBit::DepthBuffer);
        for batch in &mut self.batches {
            if batch.desc.visible {
                self.shader.ortho(&batch.matrix_full);
                if batch.sprites.len() > 0 {
                    self.shader.texture(TextureUnit::Atlas);
                    batch.sprites.draw();
                }
                if batch.strings.len() > 0 {
                    self.shader.texture(TextureUnit::Font);
                    batch.strings.draw();
                }
            }
        }
        self.window.swap_buffers();
    }
}
