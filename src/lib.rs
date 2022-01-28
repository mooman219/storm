#![no_std]
#![allow(dead_code, non_camel_case_types, non_snake_case)]

pub extern crate log;

extern crate alloc;
// TODO: Resolve when glow is fixed.
// #[cfg(any(test, not(target_arch = "wasm32")))]
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
/// Synchronization utilities.
pub mod sync;
/// Time utilities.
pub mod time;

mod context;

pub use cgmath;
pub use context::{request_stop, start, wait_for, wait_periodic, wait_until};
pub use fontdue;

pub(crate) use context::ctx;
