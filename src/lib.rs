#![no_std]
#![allow(dead_code, non_camel_case_types, non_snake_case)]

pub extern crate log;

extern crate alloc;
#[cfg(any(test, not(target_arch = "wasm32")))]
extern crate std;

/// Audio primitives. Creating and controlling sounds are included in here.
pub mod audio;
/// Color primitives. These are used in the graphics and image modules for managing images and
/// textures.
pub mod color;
/// Event utilities.
pub mod event;
/// Graphics primitives.
pub mod graphics;
/// Image utilities. Images are used for creating textures.
pub mod image;
/// Math utilities.
pub mod math;
/// Time utilities.
pub mod time;

pub use crate::prelude::*;
pub use cgmath;
pub use context::Context;
pub use crevice;
pub use fontdue;

pub(crate) use global::ctx;

mod context;
mod global;
mod loader;
mod prelude;
mod render;
mod sync;

/// Returns a simple 1x1 white texture. This texture is reused globally.
pub fn default_texture() -> crate::graphics::Texture {
    let gpu = ctx().graphics();
    gpu.default_texture()
}

/// Gets the max texture size supported on the GPU.
pub fn max_texture_size() -> usize {
    ctx().graphics().max_texture_size() as usize
}

/// Clears the screen buffers according to the clear mode.
pub fn clear(clear_mode: ClearMode) {
    let gl = ctx().graphics().gl();
    if let Some(clear_color) = clear_mode.color {
        gl.clear_color(clear_color);
    }
    gl.clear(clear_mode.mode);
}
