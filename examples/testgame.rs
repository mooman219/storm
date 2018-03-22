extern crate storm;

use storm::engine;
use storm::game::testgame::*;

/// Run with: cargo run --example testgame --release
/// Ideally the game code would exist in examples/, but it is difficult to develop on because
/// files in the examples folder are not referenced by the RLS. Game code will exist in the main
/// source folder for the time being while this project is being prototyped.
///
/// See https://github.com/rust-lang-nursery/rls/issues/269
fn main() {
    engine::run(TestGame::new());
}
