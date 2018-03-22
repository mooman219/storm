use bounded_spsc_queue::Consumer;
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

    pub fn consume_keyboard() {}

    pub fn consume_cursor() {}

    pub fn tick(&mut self) {
        // Frame processing
        match self.input_consumer.try_pop().as_mut() {
            Some(_) => {
                // TODO: Actually process input.
            },
            None => {},
        }
    }
}
