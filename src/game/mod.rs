pub mod entity;
pub mod state;
pub mod universe;
pub mod world;

use bounded_spsc_queue::Producer;
use render::comm::frame::Frame;

pub fn game_loop(frame_producer: Producer<Frame>) {}
