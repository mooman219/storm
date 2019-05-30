pub use render::client::*;
pub use render::gl::Window;
pub use render::quad::*;

mod client;
mod gl;
mod message;
mod quad;
mod server;

use utility;

pub fn start(window: Window, render_consumer: utility::bucket_spsc::Consumer<message::RenderState>) {
    let mut server = server::RenderServer::new(window, render_consumer);
    server.run_forever();
}
