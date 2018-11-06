use bounded_spsc_queue;
use input::message::*;
use render::message::*;

pub trait Game {
    /// Function to instantiate a new game object.
    fn new(render: RenderMessenger) -> Self;

    /// Called once per iteration of the game loop.
    fn tick(&mut self) {}

    /// Called when there's input to handle. If there is more then one input event to handle, this
    /// function is called repeatedly until the events are exhausted.
    fn input(&mut self, _: InputFrame) {}
}

pub fn start<G: Game>(
    input_consumer_pipe: bounded_spsc_queue::Consumer<InputFrame>,
    render_producer_pipe: bounded_spsc_queue::Producer<RenderFrame>,
) {
    let mut input_consumer = InputMessenger::new(input_consumer_pipe);
    let render_producer = RenderMessenger::new(render_producer_pipe);
    let mut game = G::new(render_producer);
    loop {
        input_consumer.tick(&mut game);
        game.tick();
    }
}
