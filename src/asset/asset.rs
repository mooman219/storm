use super::LoaderError;
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

pub(crate) type AssetCallbackBox = Box<dyn FnMut(alloc::vec::Vec<Asset>) + Send + 'static>;

pub(crate) struct AssetRequest {
    pub assets: Vec<Asset>,
    pub callback: AssetCallbackBox,
}

impl AssetRequest {
    pub(crate) fn new(relative_paths: &[&str], callback: AssetCallbackBox) -> AssetRequest {
        AssetRequest {
            assets: relative_paths
                .iter()
                .map(|path| Asset::new_err(path.to_string(), LoaderError::Pending))
                .collect::<Vec<Asset>>(),
            callback,
        }
    }
}
