cargo build --example particles --release --target=wasm32-unknown-unknown
@REM wasm-bindgen: cargo install -f wasm-bindgen-cli
wasm-bindgen --target web --no-typescript --out-dir ./docs/ --out-name particles ./target/wasm32-unknown-unknown/release/examples/particles.wasm