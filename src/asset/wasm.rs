use crate::asset::{Asset, AssetStateContract, LoaderError};

pub(crate) struct AssetState {}

impl AssetStateContract for AssetState {
    fn init() -> Self {
        AssetState {}
    }

    fn push_read(&mut self, _relative_path: &str) {}

    fn try_pop_read(&mut self) -> Option<Asset> {
        None
    }
}
