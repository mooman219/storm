use bounded_spsc_queue;
use glutin;
use input::consumer::*;
use input::producer::*;
use input::*;
use render::message::consumer::*;
use render::message::producer::*;
use render::*;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use utility::consume_spsc;

pub trait Game {
    /// The window name for the game.
    const TITLE: &'static str = "Untitled";

    /// Function to instantiate a new game object.
    fn new(render: RenderProducer) -> Self;

    /// Called once per iteration of the game loop.
    fn tick(&mut self) {}

    /// Called when there's input to handle. If there is more then one input event to handle, this
    /// function is called repeatedly until the events are exhausted.
    fn input(&mut self, _: InputFrame) {}
}

/// Creates and runs a game. Threads for input, rendering, and game logic are created along with
/// communication channels between them. The game is then instantiated. This function blocks until
/// the game window is closed.
pub fn run<G: Game>() {
    // Winow creation
    let event_loop = glutin::EventsLoop::new();
    let display = display::Display::new(
        glutin::WindowBuilder::new()
            .with_title(G::TITLE)
            .with_dimensions(400, 400),
        glutin::ContextBuilder::new(),
        &event_loop,
    );
    // Inter-thread message queues for input and rendering
    let (render_producer_pipe, render_consumer_pipe) = bounded_spsc_queue::make(4);
    let (input_producer_pipe, input_consumer_pipe) = bounded_spsc_queue::make(256);
    let (resize_producer, resize_consumer) = consume_spsc::make();

    // Game thread (daemon)
    thread::spawn(move || {
        let mut input_consumer = InputConsumer::new(input_consumer_pipe);
        let render_producer = RenderProducer::new(render_producer_pipe);
        let mut game = G::new(render_producer);
        loop {
            input_consumer.tick(&mut game);
            game.tick();
        }
    });

    // Render thread (daemon)
    thread::spawn(move || {
        // The display is bound in the thread we're going to be making opengl calls in. Behavior is
        // undefined is the display is bound outside of the thread and usually segfaults.
        display.bind();
        let mut render_consumer = RenderConsumer::new(display, render_consumer_pipe, resize_consumer);
        loop {
            // Render
            render_consumer.tick();
            // Sleep
            sleep(Duration::new(0, 100));
        }
    });

    // Input thread (main)
    let mut input_producer = InputProducer::new(event_loop, input_producer_pipe, resize_producer);
    while input_producer.is_active() {
        // Input
        input_producer.tick();
        // Sleep
        sleep(Duration::new(0, 100));
    }
}
