use crate::asset::{Asset, AssetStateContract, LoaderError};
use alloc::vec::Vec;
use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;

// Literally exploiting an injection vulnerability to append arbitrary js into the bundle
#[wasm_bindgen(raw_module = r#"./web.js';
// uses `var` instead of `let` so there's no TDZ in case functions using it are called before this module body executes
var asset_payloads;

function _push_asset(path) {
    fetch(path)
        .then(function (response) {
            if (response.status < 200 || response.status >= 300) {
                (asset_payloads ||= []).push([path, response.status]);
            } else {
                response.arrayBuffer().then(function (buffer) {
                    (asset_payloads ||= []).push([path, new Uint8Array(buffer)]);
                }).catch(function (reason) {
                    (asset_payloads ||= []).push([path, 500]);
                });;
            }
        }).catch(function (reason) {
            (asset_payloads ||= []).push([path, 500]);
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
    fn push_asset(path: &str);
    fn pull_assets() -> Array;
}

pub(crate) struct AssetState {
    results: Vec<Asset>,
}

impl AssetStateContract for AssetState {
    fn init() -> Self {
        AssetState {
            results: Vec::with_capacity(16),
        }
    }

    fn push_read(&mut self, relative_path: &str) {
        push_asset(relative_path);
    }

    fn try_pop_read(&mut self) -> Option<Asset> {
        if self.results.len() == 0 {
            let array: Array = pull_assets();
            for value in array.iter() {
                let tuple: Array = value.dyn_into().unwrap();
                let path = tuple.get(0).as_string().unwrap();
                let second = tuple.get(1);
                if second.is_object() {
                    let second: Uint8Array = second.dyn_into().unwrap();
                    let second = second.to_vec();
                    self.results.push(Asset::new_ok(path, second))
                } else {
                    let second = second.as_f64().unwrap() as u32;
                    let second = match second {
                        400 => LoaderError::InvalidInput,
                        401 | 402 | 403 => LoaderError::PermissionDenied,
                        404 => LoaderError::NotFound,
                        _ => LoaderError::Other,
                    };
                    self.results.push(Asset::new_err(path, second))
                }
            }
        }
        self.results.pop()
    }
}
