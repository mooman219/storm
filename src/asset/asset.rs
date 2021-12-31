use alloc::{string::String, vec::Vec};

use super::LoaderError;

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
