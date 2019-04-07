mod client;
mod message;
mod server;

pub(crate) use input::client::*;
pub use input::message::*;

use glutin::EventsLoop;
use input::server::*;
use utility::bounded_spsc;

pub(crate) fn start(event_loop: EventsLoop, input_producer: bounded_spsc::Producer<InputMessage>) {
    let mut server = InputServer::new(event_loop, input_producer);
    server.run_forever();
}
