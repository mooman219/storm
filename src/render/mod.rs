mod buffer;
mod client;
mod display;
mod message;
mod raw;
mod server;
mod shader;
mod texture;
mod vertex;

pub use render::client::*;
pub use render::display::*;
pub use render::message::*;

use render::server::*;
use utility::bounded_spsc;
use utility::control;

pub fn start(
    display: Display,
    render_consumer: bounded_spsc::Consumer<Vec<RenderMessage>>,
    render_control: control::Consumer,
) {
    let mut server = RenderServer::new(display, render_consumer, render_control);
    server.run_forever();
}
