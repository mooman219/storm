use crate::render::gl::buffer::*;
use crate::render::gl::raw::{
    BlendFactor, BufferBindingTarget, Capability, ClearBit, CullFace, DepthTest, OpenGL, TextureUnit,
};
use crate::render::gl::shader::*;
use crate::render::gl::texture_handle::*;
use crate::render::gl::window::*;
use crate::texture::*;
use crate::types::*;
use cgmath::*;

struct Batch {
    desc: BatchSettings,
    sprites: Buffer<Sprite>,
    strings: Buffer<Sprite>,
    matrix_transform: Matrix4<f32>,
    matrix_full: Matrix4<f32>,
}

pub struct OpenGLState {
    window: OpenGLWindow,
    gl: OpenGL,
    shader: TextureShader,
    texture_atlas: TextureHandle,
    batches: Vec<Batch>,
    matrix_bounds: Matrix4<f32>,
    current_logical_size: Vector2<f32>,
}

fn matrix_from_bounds(bounds: &Vector2<f32>) -> Matrix4<f32> {
    let w = bounds.x / 2.0;
    let h = bounds.y / 2.0;
    ortho(-w.floor(), w.ceil(), -h.floor(), h.ceil(), -1.0, 1.0)
}

impl OpenGLState {
    pub fn new(desc: &WindowSettings, event_loop: &glutin::event_loop::EventLoop<()>) -> OpenGLState {
        let (window, gl) = OpenGLWindow::new(desc, event_loop);
        let current_logical_size = window.logical_size();
        let gl = OpenGL::new(gl);
        let logical_size = window.logical_size();
        let shader = TextureShader::new(gl.clone());
        let texture_atlas = TextureHandle::new(gl.clone(), TextureUnit::Atlas);
        let state = OpenGLState {
            window,
            gl,
            shader,
            texture_atlas,
            batches: Vec::new(),
            matrix_bounds: matrix_from_bounds(&logical_size),
            current_logical_size,
        };
        // Bind shader once.
        state.shader.bind();
        state.shader.texture(TextureUnit::Atlas);
        // Setup cabilities.
        state.gl.enable(Capability::CullFace);
        state.gl.enable(Capability::Blend);
        state.gl.enable(Capability::DepthTest);
        state.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        state.gl.depth_func(DepthTest::Less);
        state.gl.blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        state.gl.cull_face(CullFace::Back);
        // State is setup.
        state
    }

    pub fn current_logical_size(&self) -> Vector2<f32> {
        self.current_logical_size
    }

    pub fn clear_color(&mut self, color: RGBA8) {
        let color: Vector4<f32> = color.into();
        self.gl.clear_color(color.x, color.y, color.z, color.w);
    }

    pub fn window_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn window_display_mode(&mut self, display_mode: DisplayMode) {
        self.window.set_display_mode(display_mode);
    }

    pub fn window_vsync(&mut self, vsync: Vsync) {
        self.window.set_vsync(vsync);
    }

    pub fn upload_texture_atlas(&mut self, texture: &Image) {
        self.texture_atlas.set_texture(texture);
    }

    pub fn batch_create(&mut self, desc: &BatchSettings) {
        let matrix_transform = desc.transform_matrix();
        let matrix_full = self.matrix_bounds * matrix_transform;
        self.batches.push(Batch {
            desc: *desc,
            sprites: Buffer::new(self.gl.clone(), BufferBindingTarget::ArrayBuffer),
            strings: Buffer::new(self.gl.clone(), BufferBindingTarget::ArrayBuffer),
            matrix_transform,
            matrix_full,
        });
    }

    pub fn batch_update(&mut self, index: usize, desc: &BatchSettings) {
        let batch = &mut self.batches[index];
        let matrix_transform = desc.transform_matrix();
        let matrix_full = self.matrix_bounds * matrix_transform;
        batch.desc = *desc;
        batch.matrix_transform = matrix_transform;
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
            let new_physical_size = self.window.physical_size();
            trace!("Window resized {:?}", new_physical_size);
            self.current_logical_size = new_logical_size;
            self.gl.viewport(0, 0, new_physical_size.x as i32, new_physical_size.y as i32);
            self.matrix_bounds = matrix_from_bounds(&new_logical_size);
            for batch in &mut self.batches {
                batch.matrix_full = self.matrix_bounds * batch.matrix_transform;
            }
        }
    }

    pub fn draw(&mut self) {
        self.resize();
        self.gl.clear(ClearBit::ColorBuffer | ClearBit::DepthBuffer);
        for batch in &mut self.batches {
            if batch.desc.visible {
                self.shader.ortho(&batch.matrix_full);
                if batch.sprites.len() > 0 {
                    batch.sprites.draw();
                }
                self.gl.clear(ClearBit::DepthBuffer);
                if batch.strings.len() > 0 {
                    batch.strings.draw();
                }
            }
        }
        self.window.swap_buffers();
    }
}
