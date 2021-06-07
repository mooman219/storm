use crate::render::buffer::Buffer;
use crate::render::raw::BufferBindingTarget;
use crate::render::OpenGLState;
use crate::types::{LayerTransform, Sprite};
use crate::utility::bad::UnsafeShared;
use cgmath::*;

const IDENTITY_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, //
    0.0, 1.0, 0.0, 0.0, //
    0.0, 0.0, 1.0, 0.0, //
    0.0, 0.0, 0.0, 1.0, //
);

/// Data shared between the engine state and game state.
pub struct SharedLayer {
    index: usize,
    transform: Matrix4<f32>,
    ortho: Matrix4<f32>,
    ortho_transform: Matrix4<f32>,
}

impl SharedLayer {
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn set_ortho(&mut self, ortho: &Matrix4<f32>) {
        self.ortho = *ortho;
        self.ortho_transform = self.ortho * self.transform;
    }

    pub fn set_transform_matrix(&mut self, transform: &Matrix4<f32>) {
        self.transform = *transform;
        self.ortho_transform = self.ortho * self.transform;
    }
}

/// Layers represent draw calls and hold configuration associated with drawing to the screen.
pub struct Layer {
    state: UnsafeShared<OpenGLState>,
    shared: UnsafeShared<SharedLayer>,
    is_visible: bool,
    sprites: Buffer<Sprite>,
}

impl Layer {
    pub(crate) fn new(
        state: UnsafeShared<OpenGLState>,
        ortho: &Matrix4<f32>,
    ) -> (UnsafeShared<SharedLayer>, Layer) {
        let shared = UnsafeShared::new(SharedLayer {
            index: 0,
            transform: IDENTITY_MATRIX,
            ortho: *ortho,
            ortho_transform: *ortho,
        });
        let layer = Layer {
            state: state.clone(),
            shared: shared.clone(),
            is_visible: true,
            sprites: Buffer::new(state, BufferBindingTarget::ArrayBuffer),
        };
        (shared, layer)
    }

    pub fn draw(&mut self) {
        if self.is_visible && self.sprites.len() > 0 {
            self.state.shader_ortho(&self.shared.ortho_transform);
            self.sprites.draw();
        }
    }

    /// Sets the sprites that will be drawn.
    pub fn set_sprites(&mut self, sprites: &Vec<Sprite>) {
        self.sprites.set(sprites);
    }

    /// Clears all the sprites, drawing nothing.
    pub fn clear_sprites(&mut self) {
        self.sprites.clear();
    }

    /// Sets the transformation matrix used when drawing this.
    pub fn set_transform(&mut self, transform: &LayerTransform) {
        self.shared.set_transform_matrix(&transform.to_matrix());
    }

    /// Sets the transformation matrix used when drawing this.
    pub fn set_transform_matrix(&mut self, transform: &Matrix4<f32>) {
        self.shared.set_transform_matrix(transform);
    }

    /// If the renderer should render this layer or not when draw is called.
    pub fn set_visible(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
    }
}

impl Drop for Layer {
    fn drop(&mut self) {
        self.state.layer_drop(self.shared.index);
    }
}
