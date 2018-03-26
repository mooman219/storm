use bounded_spsc_queue;
use glutin;
use glutin::ElementState;
use glutin::EventsLoop;
use glutin::MouseButton;
use input::*;
use utility::consume_spsc;

struct InputState {
    input_producer: bounded_spsc_queue::Producer<InputFrame>,
    resize_producer: consume_spsc::Producer<ResizeMessage>,
    is_active: bool,
    cursos_pos: Vector2<f32>,
}

pub struct InputProducer {
    inner: InputState,
    event_loop: EventsLoop,
}

impl InputProducer {
    pub fn new(
        event_loop: EventsLoop,
        input_producer: bounded_spsc_queue::Producer<InputFrame>,
        resize_producer: consume_spsc::Producer<ResizeMessage>,
    ) -> InputProducer {
        InputProducer {
            inner: InputState {
                input_producer: input_producer,
                resize_producer: resize_producer,
                is_active: true,
                cursos_pos: Vector2::new(0f32, 0f32),
            },
            event_loop: event_loop,
        }
    }

    pub fn is_active(&self) -> bool {
        self.inner.is_active
    }

    pub fn tick(&mut self) {
        // TODO: Track input so we ignore the os when it sends a key multiple times.
        // Run the event loop
        let inner = &mut self.inner;
        self.event_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Resized(w, h) => {
                    inner.resize_producer.set(ResizeMessage { width: w, height: h });
                },
                glutin::WindowEvent::Closed => inner.is_active = false,
                glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(key) => {
                        let message = match input.state {
                            ElementState::Pressed => InputFrame::KeyPressed(key),
                            ElementState::Released => InputFrame::KeyReleased(key),
                        };
                        inner.input_producer.push(message);
                    },
                    None => {},
                },
                glutin::WindowEvent::CursorMoved { position, .. } => {
                    let (x, y) = position;
                    inner.cursos_pos = Vector2::new(x as f32, y as f32);
                    // TODO: Replace_spsc for mouse position
                },
                glutin::WindowEvent::CursorEntered { .. } => {
                    inner.input_producer.push(InputFrame::CursorEntered);
                },
                glutin::WindowEvent::CursorLeft { .. } => {
                    inner.input_producer.push(InputFrame::CursorLeft);
                },
                glutin::WindowEvent::MouseInput { state, button, .. } => {
                    let button = match button {
                        MouseButton::Left => CursorButton::Left,
                        MouseButton::Right => CursorButton::Right,
                        MouseButton::Middle => CursorButton::Middle,
                        MouseButton::Other(value) => CursorButton::Other(value),
                    };
                    let message = match state {
                        ElementState::Pressed => InputFrame::CursorPressed(button, inner.cursos_pos),
                        ElementState::Released => InputFrame::CursorReleased(button, inner.cursos_pos),
                    };
                    inner.input_producer.push(message);
                },
                // Other events: https://docs.rs/glutin/0.13.1/glutin/enum.WindowEvent.html
                _ => (),
            },
            // Other events: https://docs.rs/glutin/0.13.1/glutin/enum.Event.html
            _ => (),
        });
    }
}
