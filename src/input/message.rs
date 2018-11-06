use bounded_spsc_queue::Consumer;
use cgmath::*;
use game::*;

// Re-exports.
pub use glutin::MouseButton as CursorButton;
pub use glutin::VirtualKeyCode as Key;

// ////////////////////////////////////////////////////////
// Messages
// ////////////////////////////////////////////////////////

/// These are represented as an enumeration to preserve ordering when stored
/// in a vector and read sequentially.
#[derive(Copy, Clone)]
pub enum InputFrame {
    // Represents keyboard events.
    KeyPressed(Key),
    KeyReleased(Key),

    // Represents cursor events.
    CursorPressed(CursorButton, Vector2<f64>),
    CursorReleased(CursorButton, Vector2<f64>),
    CursorLeft,
    CursorEntered,
}

// ////////////////////////////////////////////////////////
// Messenger
// ////////////////////////////////////////////////////////

pub struct InputMessenger {
    input_consumer: Consumer<InputFrame>,
}

impl InputMessenger {
    pub fn new(input_consumer: Consumer<InputFrame>) -> InputMessenger {
        InputMessenger {
            input_consumer: input_consumer,
        }
    }

    pub fn tick<G: Game>(&mut self, game: &mut G) {
        // Frame processing
        loop {
            match self.input_consumer.try_pop() {
                Some(frame) => {
                    game.input(frame);
                },
                None => return,
            }
        }
    }
}
