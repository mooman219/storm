pub mod frame;

use bounded_spsc_queue::Consumer;
use bounded_spsc_queue::Producer;
use input::InputFrame;

pub struct InputProducer {
    frame_producer: Producer<InputFrame>,
}

impl InputProducer {
    pub fn new(frame_producer: Producer<InputFrame>) -> InputProducer {
        InputProducer {
            frame_producer: frame_producer,
        }
    }
}

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
