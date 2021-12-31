#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native::AssetState;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub(crate) use wasm::AssetState;

mod asset;
mod error;

pub use self::asset::Asset;
pub use self::error::LoaderError;

use crate::ctx;

/// Requests a read of an asset. This produces an AssetRead event with the result of the read once
/// it has completed.
///
/// ## Platform-specific
///
/// - **Non-web:** The path is relative to the current working directory.
/// - **Web:** The path is relative to the current url's root.
pub fn request_read(relative_path: &str) {
    ctx().assets().push_read(relative_path);
}

pub(crate) trait AssetStateContract {
    /// Creates a new asset state.
    fn init() -> Self;

    /// Pushes a read request to the queue. Relative to the current working directory.
    fn push_read(&mut self, relative_path: &str);

    /// Pops the next available read off the queue, returning None if there are no finished reads
    /// available.
    fn try_pop_read(&mut self) -> Option<Asset>;
}
