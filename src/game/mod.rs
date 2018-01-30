pub mod entity;
pub mod state;
pub mod universe;
pub mod world;

use bounded_spsc_queue::Producer;
use render::message::frame::RenderFrame;

pub fn game_loop(frame_producer: Producer<RenderFrame>) {}
