use crate::input::message::*;
use crate::utility::bounded_spsc;

pub struct InputClient {
    input_producer: bounded_spsc::Consumer<InputMessage>,
}

impl InputClient {
    pub fn new(input_producer: bounded_spsc::Consumer<InputMessage>) -> InputClient {
        InputClient {
            input_producer,
        }
    }

    pub fn poll(&self) -> Option<InputMessage> {
        self.input_producer.try_pop()
    }
}
