use cgmath::*;
use render::gl::buffer::*;
use render::gl::raw::*;
use render::gl::shader::*;
use render::gl::texture_handle::*;
use render::gl::window::*;
use texture::*;
use types::*;

struct Batch {
    desc: BatchDescription,
    sprites: Buffer<SpriteDescription>,
    strings: Buffer<SpriteDescription>,
    matrix_translate_scaled: Matrix4<f32>,
    matrix_full: Matrix4<f32>,
}

pub struct OpenGLState {
    window: Window,
    shader: TextureShader,
    texture_atlas: TextureHandle,
    texture_font: TextureHandle,
    batches: Vec<Batch>,
    matrix_bounds: Matrix4<f32>,
    current_logical_size: Vector2<f64>,
}

fn matrix_from_bounds(bounds: &Vector2<f64>) -> Matrix4<f32> {
    let w = (bounds.x / 2.0) as f32;
    let h = (bounds.y / 2.0) as f32;
    ortho(-w.floor(), w.ceil(), -h.floor(), h.ceil(), -1.0, 1.0)
}

fn matrix_from_translate_scaled(translation: &Vector2<f32>, scale: f32) -> Matrix4<f32> {
    Matrix4::from_translation(translation.extend(0.0)) * Matrix4::from_scale(scale)
}

impl OpenGLState {
    pub fn new(window: Window) -> OpenGLState {
        window.bind();
        let logical_size = window.get_logical_size();
        let mut state = OpenGLState {
            window: window,
            shader: TextureShader::new(),
            texture_atlas: TextureHandle::new(TextureUnit::Atlas),
            texture_font: TextureHandle::new(TextureUnit::Font),
            batches: Vec::new(),
            matrix_bounds: matrix_from_bounds(&logical_size),
            current_logical_size: logical_size,
        };
        state.setup();
        state
    }

    fn setup(&mut self) {
        // Bind shader once.
        self.shader.bind();
        // Setup cabilities.
        enable(Capability::CullFace);
        enable(Capability::Blend);
        enable(Capability::DepthTest);
        clear_color(0.0, 0.5, 0.0, 1.0);
        depth_func(DepthTest::Less);
        blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        cull_face(CullFace::Back);
    }

    pub fn window_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn upload_texture_atlas(&mut self, texture: &Image) {
        self.texture_atlas.set_texture(texture);
    }

    pub fn upload_font_atlas(&mut self, texture: &Image) {
        self.texture_font.set_texture(texture);
    }

    pub fn batch_create(&mut self, desc: &BatchDescription) {
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

    pub fn batch_update(&mut self, index: usize, desc: &BatchDescription) {
        let batch = &mut self.batches[index];
        let matrix_translate_scaled = matrix_from_translate_scaled(&desc.translation, desc.scale);
        let matrix_full = self.matrix_bounds * matrix_translate_scaled;
        batch.desc = *desc;
        batch.matrix_translate_scaled = matrix_translate_scaled;
        batch.matrix_full = matrix_full;
    }

    pub fn batch_sprite_set(&mut self, index: usize, quads: &Vec<SpriteDescription>) {
        self.batches[index].sprites.set(quads);
    }

    pub fn batch_string_set(&mut self, index: usize, quads: &Vec<SpriteDescription>) {
        self.batches[index].strings.set(quads);
    }

    pub fn batch_remove(&mut self, index: usize) {
        self.batches.swap_remove(index);
    }

    fn resize(&mut self) {
        let new_logical_size = self.window.get_logical_size();
        if self.current_logical_size != new_logical_size {
            self.current_logical_size = new_logical_size;
            let new_physical_size = self.window.get_physical_size();
            // viewport(0, 0, new_physical_size.x as i32, new_physical_size.y as i32);
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
