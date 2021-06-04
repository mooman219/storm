use crate::render::buffer::Buffer;
use crate::render::raw::{BufferBindingTarget, ClearBit, OpenGL};
use crate::render::shader::TextureShader;
use crate::types::{BatchTransform, Sprite};
use crate::utility::bad::UnsafeShared;
use cgmath::*;

const IDENTITY_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, //
    0.0, 1.0, 0.0, 0.0, //
    0.0, 0.0, 1.0, 0.0, //
    0.0, 0.0, 0.0, 1.0, //
);

struct Inner {
    gl: OpenGL,
    is_visible: bool,
    sprites: Buffer<Sprite>,
    // Matrices
    transform: Matrix4<f32>,
    ortho: Matrix4<f32>,
    ortho_transform: Matrix4<f32>,
}

pub struct Batch {
    inner: UnsafeShared<Inner>,
}

impl Batch {
    pub(crate) fn new(gl: OpenGL, ortho: &Matrix4<f32>) -> (Batch, Batch) {
        let inner = UnsafeShared::new(Inner {
            gl: gl.clone(),
            is_visible: true,
            sprites: Buffer::new(gl, BufferBindingTarget::ArrayBuffer),
            // Matrices
            transform: IDENTITY_MATRIX,
            ortho: *ortho,
            ortho_transform: ortho * IDENTITY_MATRIX,
        });
        let a = Batch {
            inner: inner.clone(),
        };
        let b = Batch {
            inner,
        };
        (a, b)
    }

    pub(crate) fn count(&self) -> usize {
        self.inner.count()
    }

    pub(crate) fn set_ortho(&mut self, ortho: &Matrix4<f32>) {
        let batch = &mut self.inner;
        batch.ortho = *ortho;
        batch.ortho_transform = batch.ortho * batch.transform;
    }

    pub(crate) fn draw(&mut self, shader: &TextureShader) {
        let batch = &self.inner;
        if batch.is_visible && batch.sprites.len() > 0 {
            shader.ortho(&batch.ortho_transform);
            batch.sprites.draw();
            batch.gl.clear(ClearBit::DepthBuffer);
        }
    }

    /// Sets the sprites that will be drawn.
    pub fn set_sprites(&mut self, sprites: &Vec<Sprite>) {
        self.inner.sprites.set(sprites);
    }

    /// Clears all the sprites, drawing nothing.
    pub fn clear_sprites(&mut self) {
        self.inner.sprites.clear();
    }

    /// Sets the transformation matrix used when drawing this.
    pub fn set_transform(&mut self, transform: &BatchTransform) {
        let batch = &mut self.inner;
        batch.transform = transform.to_matrix();
        batch.ortho_transform = batch.ortho * batch.transform;
    }

    /// Sets the transformation matrix used when drawing this.
    pub fn set_transform_matrix(&mut self, transform: &Matrix4<f32>) {
        let batch = &mut self.inner;
        batch.transform = *transform;
        batch.ortho_transform = batch.ortho * batch.transform;
    }

    /// If the renderer should render this batch or not.
    pub fn set_visible(&mut self, is_visible: bool) {
        self.inner.is_visible = is_visible;
    }
}
