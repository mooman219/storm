extern crate storm;

use storm::engine;
use storm::game::testgame::*;

fn main() {
    engine::run(TestGame::new());
}
