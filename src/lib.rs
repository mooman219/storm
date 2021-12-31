#![no_std]
#![allow(dead_code, non_camel_case_types, non_snake_case)]

pub extern crate log;

extern crate alloc;
#[cfg(any(test, not(target_arch = "wasm32")))]
extern crate std;

/// Asset utilities.
pub mod asset;
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
pub use context::{request_stop, start, wait_for, wait_periodic, wait_until};
pub use crevice;
pub use fontdue;
pub use render::{
    clear, default_texture, max_texture_size, set_window_display_mode, set_window_title,
    viewport_logical_size, viewport_physical_size, window_logical_size, window_physical_size,
};

pub(crate) use context::ctx;

mod context;
mod prelude;
mod render;
mod sync;

// ====================================
// Assets
// ====================================

/// Requests a read of an asset. This produces an AssetRead event with the result of the read once
/// it has completed.
///
/// ## Platform-specific
///
/// - **Non-web:** The path is relative to the current working directory.
/// - **Web:** The path is relative to the current url's root.
pub fn request_read(relative_path: &str) {
    use asset::AssetStateContract;
    ctx().assets().push_read(relative_path);
}
