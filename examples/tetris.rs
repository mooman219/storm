extern crate rand;
use storm::time::*;
use storm::*;
mod tetris_game;

use tetris_game::*;

/// Run with: cargo run --example tetris --release
fn main() {
    Engine::start(
        WindowSettings {
            title: String::from("Storm: Tetris"),
            display_mode: DisplayMode::Windowed {
                width: 1280,
                height: 1024,
                resizable: true,
            },
            vsync: Vsync::Disabled,
        },
        game,
    );
}

fn game(engine: Engine) {
    let mut tetris_state = TetrisState::new(engine);
    tetris_state.update();
}


//todos:
//1. Pause music, with current track being paused
//2. Correct Tetris Scoring
//3. volume up and down
//4. Effect when you clear a row
//5. hover move