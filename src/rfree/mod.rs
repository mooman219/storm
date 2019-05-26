mod client;
mod gl;
mod manager;
mod message;
mod server;
mod texture;

pub use render::client::*;
pub use render::message::*;
pub use render::server::Window;

use render::server::*;
use utility;

pub fn start(
    window: Window,
    render_consumer: utility::bucket_spsc::Consumer<RenderState>,
    render_control: utility::control::Consumer,
) {
    let mut server = RenderServer::new(window, render_consumer, render_control);
    server.run_forever();
}
