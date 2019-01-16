extern crate storm;

use storm::game::*;
use storm::input::message::*;
use storm::log::LevelFilter;
use storm::render::message::*;
use storm::time::clock::*;

mod smb_lib;
use smb_lib::World;
 
/// Run with: cargo run --example smb --release
fn main() {
    storm::log::set_max_level(LevelFilter::Off);
    storm::run::<TestGame>();
}

pub struct TestGame {
    render: RenderMessenger,
    clock: Clock,
    world: World,
}

impl TestGame {
}
 
impl Game for TestGame {
    fn new(mut render: RenderMessenger) -> Self {
        
        let mut game = TestGame {
            world: World::new(&mut render),

            render: render,
            clock: Clock::new(144),
        };

        game.render.window_title("Game of Testing");
        game.render.send();
        game
    }

    fn input(&mut self, event: InputFrame) {
        self.world.input(event);
    }

    fn tick(&mut self) {
        let delta = self.clock.get_delta();
        self.world.tick(delta);
        self.world.render(&mut self.render);
        self.render.send();
        self.clock.tick();
    }
}