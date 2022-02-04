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

pub(crate) use self::asset::AssetRequest;

/// Requests a read of an asset. This produces an AssetRead event with the result of the read once
/// it has completed.
///
/// ## Platform-specific
///
/// - **Non-web:** The path is relative to the current working directory.
/// - **Web:** The path is relative to the current url's root.
pub fn request_read<T: FnMut(alloc::vec::Vec<Asset>) + Send + 'static>(relative_paths: &[&str], callback: T) {
    let request = AssetRequest::new(relative_paths, alloc::boxed::Box::new(callback));
    crate::ctx().assets().read(request);
}

pub(crate) trait AssetStateContract {
    /// Creates a new asset state.
    fn init() -> Self;

    /// Pushes a read request to the queue. Relative to the current working directory.
    fn read(&mut self, request: AssetRequest);

    /// Processes all available completed read requests.
    fn process(&mut self);
}
