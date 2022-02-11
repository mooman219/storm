use super::LoaderError;
use crate::{App, Context};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

/// Represents a binary blob loaded from an external source.
#[derive(Clone, Debug)]
pub struct Asset {
    /// The path used to query for this asset.
    pub relative_path: String,
    /// Either the contents of the asset as bytes, or an error.
    pub result: Result<Vec<u8>, LoaderError>,
}

impl Asset {
    /// Creates a new asset that successfully loaded.
    pub fn new_ok(relative_path: String, contents: Vec<u8>) -> Asset {
        Asset {
            relative_path,
            result: Ok(contents),
        }
    }

    /// Creates a new asset that failed to load.
    pub fn new_err(relative_path: String, error: LoaderError) -> Asset {
        Asset {
            relative_path,
            result: Err(error),
        }
    }
}

pub(crate) struct AssetRequest<A: App> {
    pub assets: Vec<Asset>,
    pub callback: Box<dyn FnMut(&mut Context<A>, &mut A, Vec<Asset>) + Send + 'static>,
}

impl<A: App> AssetRequest<A> {
    pub(crate) fn new<C: FnMut(&mut Context<A>, &mut A, Vec<Asset>) + Send + 'static>(
        relative_paths: &[impl AsRef<str>],
        callback: C,
    ) -> AssetRequest<A> {
        AssetRequest {
            assets: relative_paths
                .into_iter()
                .map(|path| Asset::new_err(path.as_ref().to_string(), LoaderError::Pending))
                .collect::<Vec<Asset>>(),
            callback: Box::new(callback),
        }
    }

    pub(crate) fn call(mut self, ctx: &mut Context<A>, app: &mut A) {
        (self.callback)(ctx, app, self.assets)
    }
}
