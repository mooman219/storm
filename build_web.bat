cargo build --example particles --release --target=wasm32-unknown-unknown
@REM wasm-bindgen: cargo install -f wasm-bindgen-cli
wasm-bindgen --target no-modules --no-typescript --out-dir ./docs/ --out-name web ./target/wasm32-unknown-unknown/release/examples/web.wasm