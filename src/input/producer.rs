use bounded_spsc_queue;
use glutin;
use glutin::ElementState;
use glutin::EventsLoop;
use glutin::MouseButton;
use input::*;
use utility::single_spsc;

pub struct InputProducer {
    input_producer: bounded_spsc_queue::Producer<InputFrame>,
    resize_producer: single_spsc::Producer<ResizeMessage>,
    event_loop: EventsLoop,
    is_active: bool,
}

impl InputProducer {
    pub fn new(
        event_loop: EventsLoop,
        input_producer: bounded_spsc_queue::Producer<InputFrame>,
        resize_producer: single_spsc::Producer<ResizeMessage>,
    ) -> InputProducer {
        InputProducer {
            input_producer: input_producer,
            resize_producer: resize_producer,
            event_loop: event_loop,
            is_active: true,
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn tick(&mut self) {
        // Disjoint References
        let is_active = &mut self.is_active;
        let resize_producer = &mut self.resize_producer;
        let input_producer = &mut self.input_producer;
        // Run the event loop
        self.event_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Resized(w, h) => {
                    resize_producer.push(ResizeMessage { width: w, height: h });
                },
                glutin::WindowEvent::Closed => *is_active = false,
                glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(key) => {
                        let message = match input.state {
                            ElementState::Pressed => InputFrame::KeyPressed(key),
                            ElementState::Released => InputFrame::KeyReleased(key),
                        };
                        input_producer.push(message);
                    },
                    None => {},
                },
                glutin::WindowEvent::CursorMoved { position, .. } => {
                    let (x, y) = position;
                    println!("{}, {}", x, y);
                    // TODO: This fills the buffer.
                    // let message = InputFrame::CursorMoved(Vector2::new(x as f32, y as f32));
                    // input_producer.push(message);
                },
                glutin::WindowEvent::CursorEntered { .. } => {
                    input_producer.push(InputFrame::CursorEntered);
                },
                glutin::WindowEvent::CursorLeft { .. } => {
                    input_producer.push(InputFrame::CursorLeft);
                },
                glutin::WindowEvent::MouseInput { state, button, .. } => {
                    let button = match button {
                        MouseButton::Left => CursorButton::Left,
                        MouseButton::Right => CursorButton::Right,
                        MouseButton::Middle => CursorButton::Middle,
                        MouseButton::Other(value) => CursorButton::Other(value),
                    };
                    let message = match state {
                        ElementState::Pressed => InputFrame::CursorPressed(button),
                        ElementState::Released => InputFrame::CursorReleased(button),
                    };
                    input_producer.push(message);
                },
                // Other events: https://docs.rs/glutin/0.13.1/glutin/enum.WindowEvent.html
                _ => (),
            },
            // Other events: https://docs.rs/glutin/0.13.1/glutin/enum.Event.html
            _ => (),
        });
    }
}
