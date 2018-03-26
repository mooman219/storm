use bounded_spsc_queue::Consumer;
use engine::*;
use input::*;

pub struct InputConsumer {
    input_consumer: Consumer<InputFrame>,
}

impl InputConsumer {
    pub fn new(input_consumer: Consumer<InputFrame>) -> InputConsumer {
        InputConsumer {
            input_consumer: input_consumer,
        }
    }

    pub fn tick<G: Game>(&mut self, game: &mut G) {
        // Frame processing
        loop {
            match self.input_consumer.try_pop() {
                Some(frame) => {
                    game.input(frame);
                },
                None => return,
            }
        }
    }
}
