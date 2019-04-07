use input::message::*;
use utility::bounded_spsc;

pub(crate) struct InputClient {
    input_consumer: bounded_spsc::Consumer<InputMessage>,
}

impl InputClient {
    pub fn new(input_consumer: bounded_spsc::Consumer<InputMessage>) -> InputClient {
        InputClient {
            input_consumer: input_consumer,
        }
    }

    pub fn poll(&mut self) -> Option<InputMessage> {
        self.input_consumer.try_pop()
    }
}
