use crate::asset::{AssetRequest, AssetStateContract, LoaderError};
use alloc::vec::Vec;
use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

// Literally exploiting an injection vulnerability to append arbitrary js into the bundle
#[wasm_bindgen(raw_module = r#"./web.js';
// uses `var` instead of `let` so there's no TDZ in case functions using it are called before this module body executes
var asset_payloads;

function _push_asset(index, paths) {
    let promises = paths.map(function (path) {
        return fetch(path).then(function (response) {
            if (response.status < 200 || response.status >= 300) {
                return response.status;
            } else {
                return response.arrayBuffer().then(function (buffer) {
                    return new Uint8Array(buffer);
                }).catch(function (reason) {
                    return 500;
                });
            }
        }).catch(function (reason) {
            return 500;
        });
    });
    Promise.all(promises).then(function (array) {
        (asset_payloads ||= []).push([index, array]);
    });
}

function _pull_assets() {
    let temp = asset_payloads;
    asset_payloads = [];
    return temp || [];
}

export {
    _push_asset as push_asset,
    _pull_assets as pull_assets
};
//"#)] // comment out the unclosed string for valid syntax
extern "C" {
    fn push_asset(index: usize, paths: Array);
    fn pull_assets() -> Array;
}

pub(crate) struct AssetState {
    populated: usize,
    pending: Vec<Option<AssetRequest>>,
}

impl AssetStateContract for AssetState {
    fn init() -> Self {
        AssetState {
            populated: 0,
            pending: Vec::with_capacity(16),
        }
    }

    fn read(&mut self, request: AssetRequest) {
        let paths = Array::new();
        for asset in &request.assets {
            paths.push(&JsValue::from_str(&asset.relative_path));
        }
        if self.populated == self.pending.len() {
            self.pending.push(Some(request));
            push_asset(self.populated, paths);
        } else {
            for (index, slot) in &mut self.pending.iter_mut().enumerate() {
                if slot.is_none() {
                    *slot = Some(request);
                    push_asset(index, paths);
                    break;
                }
            }
        }
        self.populated += 1;
    }

    fn process(&mut self) {
        for value in pull_assets().iter() {
            let value: Array = value.dyn_into().unwrap();
            let slot_index = value.get(0).as_f64().unwrap() as usize;
            let slot_request = self.pending[slot_index].take();
            self.populated -= 1;
            match slot_request {
                Some(mut request) => {
                    let responses: Array = value.get(1).dyn_into().unwrap();
                    for (index, response) in responses.iter().enumerate() {
                        if response.is_object() {
                            let contents: Uint8Array = response.dyn_into().unwrap();
                            let contents = contents.to_vec();
                            request.assets[index].result = Ok(contents);
                        } else {
                            let status = response.as_f64().unwrap() as u32;
                            let status = match status {
                                400 => LoaderError::InvalidInput,
                                401 | 402 | 403 => LoaderError::PermissionDenied,
                                404 => LoaderError::NotFound,
                                _ => LoaderError::Other,
                            };
                            request.assets[index].result = Err(status);
                        }
                    }
                    (request.callback)(request.assets);
                }
                None => panic!(
                    "Got asset results from Javascript, but no pending Rust request. Slot [{}]",
                    slot_index
                ),
            }
        }
    }
}
