use alloc::{string::String, vec::Vec};

/// Represents a binary blob loaded from an external source.
#[derive(Clone, Debug)]
pub struct Asset {
    /// The path used to query for this asset.
    pub relative_path: String,
    /// The contents of the asset as bytes.
    pub contents: Vec<u8>,
}

impl Asset {
    /// Creates a new asset.
    pub fn new(relative_path: String, contents: Vec<u8>) -> Asset {
        Asset {
            relative_path,
            contents,
        }
    }
}
