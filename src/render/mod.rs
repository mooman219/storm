pub mod buffer;
pub mod enums;
pub mod message;
pub mod shader;
pub mod geometry;
pub mod vertex;
pub mod color;
pub mod display;

use bounded_spsc_queue::*;
use glutin;
use input::*;
use input::producer::*;
use render::enums::*;
use render::message::*;
use render::message::consumer::*;

pub fn create_target(
    input_producer: Producer<InputFrame>,
    render_consumer: Consumer<RenderFrame>,
) -> (InputProducer, RenderConsumer) {
    println!("Render: Creating new target");
    // Winow creation
    let event_loop = glutin::EventsLoop::new();
    let display = display::Display::new(
        glutin::WindowBuilder::new()
            .with_title("Storms and Swords")
            .with_dimensions(400, 400),
        glutin::ContextBuilder::new(),
        &event_loop,
    );
    println!("Render: OpenGL version {}", GlString::Version.get_string());
    // Setup communication
    let input = InputProducer::new(event_loop, input_producer);
    let render = RenderConsumer::new(display, render_consumer);
    (input, render)
}
