#[cfg(not(target_arch = "wasm32"))]
#[path = "platform/native.rs"]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native::AssetState;

#[cfg(target_arch = "wasm32")]
#[path = "platform/wasm.rs"]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub(crate) use wasm::AssetState;

mod asset;
mod error;

pub use self::asset::Asset;
pub use self::error::LoaderError;

pub(crate) use self::asset::AssetRequest;

use crate::{App, Context};

pub(crate) trait AssetStateContract<A: App> {
    /// Creates a new asset state.
    fn init() -> Self;

    /// Pushes a read request to the queue. Relative to the current working directory.
    fn read(&mut self, request: AssetRequest<A>);

    /// Processes all available completed read requests.
    fn next(&mut self) -> Option<AssetRequest<A>>;
}

/// Asset related functions.
impl<A: App> Context<A> {
    /// Requests a read of a set of assets. This produces an AssetRead event with the result of the
    /// read once all assets requested have completed.
    ///
    /// ## Platform-specific
    ///
    /// - **Non-web:** The path is relative to the current working directory.
    /// - **Web:** The path is relative to the current url's root.
    pub fn read<C: FnMut(&mut Context<A>, &mut A, alloc::vec::Vec<Asset>) + 'static>(
        &mut self,
        relative_paths: &[impl AsRef<str>],
        callback: C,
    ) {
        let request = AssetRequest::new(relative_paths, callback);
        self.assets().read(request);
    }
}
