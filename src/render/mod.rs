pub use crate::render::client::*;
pub use crate::render::gl::Window;
mod client;
mod gl;
mod message;
mod server;

use crate::utility;

pub fn start(window: Window, render_consumer: utility::bucket_spsc::Consumer<message::RenderState>) {
    let mut server = server::RenderServer::new(window, render_consumer);
    server.run_forever();
}
