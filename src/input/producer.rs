use bounded_spsc_queue::Producer;
use glutin;
use glutin::ElementState;
use glutin::EventsLoop;
use glutin::MouseButton;
use glutin::VirtualKeyCode;
use input::*;
use std::mem;

pub struct InputProducer {
    input_producer: Producer<InputFrame>,
    frame: InputFrame,
    event_loop: EventsLoop,
    is_active: bool,
    resize_message: Option<ResizeMessage>,
}

impl InputProducer {
    pub fn new(event_loop: EventsLoop, input_producer: Producer<InputFrame>) -> InputProducer {
        InputProducer {
            input_producer: input_producer,
            frame: InputFrame::new(),
            event_loop: event_loop,
            is_active: true,
            resize_message: None,
        }
    }

    fn key_input(&mut self, key: VirtualKeyCode, state: ElementState) {
        let message = match state {
            ElementState::Pressed => KeyMessage::Pressed(key),
            ElementState::Released => KeyMessage::Released(key),
        };
        self.frame.key.push(message);
    }

    fn cursor_moved(&mut self, x: f64, y: f64) {
        let message = CursorMessage::Moved(Vector2::new(x as f32, y as f32));
        self.frame.cursor.push(message);
    }

    fn cursor_input(&mut self, button: MouseButton, state: ElementState) {
        let button = match button {
            MouseButton::Left => CursorButton::Left,
            MouseButton::Right => CursorButton::Right,
            MouseButton::Middle => CursorButton::Middle,
            MouseButton::Other(value) => CursorButton::Other(value),
        };
        let message = match state {
            ElementState::Pressed => CursorMessage::Pressed(button),
            ElementState::Released => CursorMessage::Released(button),
        };
        self.frame.cursor.push(message);
    }

    fn cursor_left(&mut self) {
        self.frame.cursor.push(CursorMessage::Left);
    }

    fn cursor_entered(&mut self) {
        self.frame.cursor.push(CursorMessage::Entered);
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn next_resize(&mut self) -> Option<ResizeMessage> {
        let message = self.resize_message;
        self.resize_message = None;
        message
    }

    pub fn tick(&mut self) {
        // Disjoint References
        let is_active = &mut self.is_active;
        let resize_message = &mut self.resize_message;
        // Run the event loop
        self.event_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Resized(w, h) => {
                    *resize_message = Some(ResizeMessage {
                        width: w,
                        height: h,
                    });
                },
                glutin::WindowEvent::Closed => *is_active = false,
                glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(key) => {
                        // TODO: key_input
                    },
                    None => {},
                },
                glutin::WindowEvent::CursorMoved { position, .. } => {
                    // TODO: cursor_moved
                },
                glutin::WindowEvent::CursorEntered { .. } => {
                    // TODO: cursor_entered
                },
                glutin::WindowEvent::CursorLeft { .. } => {
                    // TODO: cursor_left
                },
                glutin::WindowEvent::MouseInput { state, button, .. } => {
                    // TODO: cursor_input
                },
                _ => (),
            },
            _ => (),
        });
    }

    pub fn send(&mut self) {
        let mut frame = InputFrame::new();
        mem::swap(&mut frame, &mut self.frame);
        self.input_producer.push(frame);
    }
}
