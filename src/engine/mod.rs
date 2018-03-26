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
use utility::consume_spsc;

pub trait Game {
    const TITLE: &'static str = "Untitled";

    fn new(render: RenderProducer) -> Self;

    fn tick(&mut self) {}

    /// Called when there's input to handle. This is called before the tick is
    /// called.
    fn input(&mut self, _: InputFrame) {}
}

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
