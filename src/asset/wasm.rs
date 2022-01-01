use crate::asset::{Asset, AssetStateContract, LoaderError};
use alloc::{string::ToString, vec::Vec};
use js_sys::{Array, Function, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};

const PUSH_ARGS: &str = "path";
const PUSH_BODY: &str = r#"
if (!window.storm_asset_payloads) {
    window.storm_asset_payloads = [];
}
fetch(path)
    .then(function (response) {
        if (response.status < 200 || response.status >= 300) {
            window.storm_asset_payloads.push([path, response.status]);
        } else {
            response.arrayBuffer().then(function (buffer) {
                window.storm_asset_payloads.push([path, new Uint8Array(buffer)]);
            }).catch(function (reason) {
                window.storm_asset_payloads.push([path, 500]);
            });;
        }
    }).catch(function (reason) {
        window.storm_asset_payloads.push([path, 500]);
    });
"#;

const PULL_BODY: &str = r#"
if (!window.storm_asset_payloads || window.storm_asset_payloads.length === 0) {
    return [];
}
let temp = window.storm_asset_payloads;
window.storm_asset_payloads = [];
return temp;
"#;

pub(crate) struct AssetState {
    push: Function,
    pull: Function,
    results: Vec<Asset>,
}

impl AssetStateContract for AssetState {
    fn init() -> Self {
        AssetState {
            push: Function::new_with_args(PUSH_ARGS, PUSH_BODY),
            pull: Function::new_no_args(PULL_BODY),
            results: Vec::with_capacity(16),
        }
    }

    fn push_read(&mut self, relative_path: &str) {
        if let Err(_) = self.push.call1(&JsValue::UNDEFINED, &JsValue::from_str(relative_path)) {
            self.results.push(Asset::new_err(relative_path.to_string(), LoaderError::Unsupported));
        }
    }

    fn try_pop_read(&mut self) -> Option<Asset> {
        if self.results.len() == 0 {
            let array: Array = self.pull.call0(&JsValue::UNDEFINED).unwrap().dyn_into().unwrap();
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
