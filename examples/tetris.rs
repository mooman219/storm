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

fn game(mut engine: Engine) {
    let device = rodio::default_output_device().unwrap();
    let mut tetris_state = TetrisState::new(engine);
    let mut bruback = Bruback::new();

    bruback.set_music_volume(0.05);
    bruback.play_music(String::from("examples/resources/tetris.ogg"));
    tetris_state.update();
}
