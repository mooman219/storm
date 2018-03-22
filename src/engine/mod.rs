use bounded_spsc_queue;
use glutin;
use input::*;
use input::consumer::*;
use input::producer::*;
use render::*;
use render::message::consumer::*;
use render::message::producer::*;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use utility::single_spsc;

pub trait Game {
    const TITLE: &'static str = "Untitled";

    fn tick(&mut self, _: &mut RenderProducer) {}

    /// Called when there's input to handle. This is called before the tick is
    /// called.
    fn input(&mut self, _: InputFrame) {}
}

pub fn run<G: Game + Send + 'static>(mut game: G) {
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
    let (render_producer_pipe, render_consumer_pipe) = bounded_spsc_queue::make(3);
    let (input_producer_pipe, input_consumer_pipe) = bounded_spsc_queue::make(100);
    let (resize_producer, resize_consumer) = single_spsc::make();

    // Game thread (daemon)
    thread::spawn(move || {
        let mut render_producer = RenderProducer::new(render_producer_pipe);
        let mut _input_consumer = InputConsumer::new(input_consumer_pipe);
        loop {
            // TODO: Call the input loop: game.input(asdf)
            game.tick(&mut render_producer);
        }
    });

    // Render thread (daemon)
    thread::spawn(move || {
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
