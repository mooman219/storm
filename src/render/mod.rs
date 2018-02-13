pub mod buffer;
pub mod enums;
pub mod message;
pub mod shader;
pub mod geometry;
pub mod vertex;
pub mod color;
pub mod display;

use bounded_spsc_queue::Consumer;
use glutin;
use render::message::*;
use render::message::consumer::*;

pub fn render_loop(frame_consumer: Consumer<RenderFrame>) {
    // Event loop creation
    let mut events_loop = glutin::EventsLoop::new();
    // Winow creation
    let display = display::Display::new(
        glutin::WindowBuilder::new()
            .with_title("Hello, World!")
            .with_dimensions(700, 700),
        glutin::ContextBuilder::new(),
        &events_loop,
    );

    let mut render_consumer = RenderConsumer::new(display, frame_consumer);

    let mut running = true;
    while running {
        // Input
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => running = false,
                // glutin::WindowEvent::Resized(w, h) => display.resize(w, h),
                _ => (),
            },
            _ => (),
        });

        // Render
        render_consumer.tick();
    }
}
