use std::cell::RefCell;

use crate::asset::{AssetRequest, AssetStateContract, LoaderError};
use crate::App;
use alloc::vec::Vec;
use hashbrown::HashMap;
use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

#[wasm_bindgen(raw_module = "./storm.js")]
extern "C" {
    fn fs_load_files(index: usize, paths: Array);
    fn fs_init_callback(callback: &Closure<dyn Fn(usize, Array)>);
}

thread_local! {
    static STORM_ASSET_FINISHED: RefCell<Vec<(usize, Array)>> = RefCell::new(Vec::new());
}

pub(crate) struct AssetState<A: App> {
    count: usize,
    pending: HashMap<usize, AssetRequest<A>>,
}

impl<A: App> AssetStateContract<A> for AssetState<A> {
    fn init() -> Self {
        let closure = Closure::new(|slot_key: usize, responses: Array| {
            STORM_ASSET_FINISHED.with_borrow_mut(|finished| {
                finished.push((slot_key, responses));
            });
        });
        fs_init_callback(&closure);
        closure.forget();

        AssetState {
            count: 0,
            pending: HashMap::with_capacity(16),
        }
    }

    fn read(&mut self, request: AssetRequest<A>) {
        let paths = Array::new();
        for asset in &request.assets {
            paths.push(&JsValue::from_str(&asset.relative_path));
        }
        self.pending.insert(self.count, request);
        fs_load_files(self.count, paths);
        self.count += 1;
    }

    fn next(&mut self) -> Option<AssetRequest<A>> {
        STORM_ASSET_FINISHED.with_borrow_mut(|pending| match pending.pop() {
            Some((slot_key, responses)) => {
                let slot_request = self.pending.remove(&slot_key);
                match slot_request {
                    Some(mut request) => {
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
                        return Some(request);
                    }
                    None => panic!(
                        "Got asset results from Javascript, but no pending Rust request. Key [{}]",
                        slot_key
                    ),
                }
            }
            _ => None,
        })
    }
}
