mod signal;
mod spsc;

pub use signal::Signal;
pub use spsc::{make, Consumer, Producer};
