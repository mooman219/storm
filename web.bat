cargo build --example pong --release --target=wasm32-unknown-unknown
cargo build --example text --release --target=wasm32-unknown-unknown
@REM wasm-bindgen: cargo install -f wasm-bindgen-cli
wasm-bindgen --target web --no-typescript --out-dir ./docs/ --out-name pong ./target/wasm32-unknown-unknown/release/examples/pong.wasm
wasm-bindgen --target web --no-typescript --out-dir ./docs/ --out-name text ./target/wasm32-unknown-unknown/release/examples/text.wasm