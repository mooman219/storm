use bounded_spsc_queue::Consumer;
use input::*;

pub struct InputConsumer {
    frame_consumer: Consumer<InputFrame>,
}

impl InputConsumer {
    pub fn new(frame_consumer: Consumer<InputFrame>) -> InputConsumer {
        InputConsumer {
            frame_consumer: frame_consumer,
        }
    }
}
