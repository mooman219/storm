use bounded_spsc_queue::Producer;
use input::*;
use std::mem;

pub struct InputProducer {
    frame_producer: Producer<InputFrame>,
    frame: InputFrame,
}

impl InputProducer {
    pub fn new(frame_producer: Producer<InputFrame>) -> InputProducer {
        InputProducer {
            frame_producer: frame_producer,
            frame: InputFrame::new(),
        }
    }

    pub fn send(&mut self) {
        let mut frame = InputFrame::new();
        mem::swap(&mut frame, &mut self.frame);
        self.frame_producer.push(frame);
    }
}
