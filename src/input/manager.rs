use cgmath::*;
use glutin::ElementState;
use glutin::Event;
use glutin::EventsLoop;
use glutin::WindowEvent;
use input::message::*;

struct InputState {
    cursos_pos: Vector2<f64>,
    key_state: [bool; 512],
}

pub struct InputManager {
    state: InputState,
    event_loop: EventsLoop,
}

impl InputManager {
    pub fn new(event_loop: EventsLoop) -> InputManager {
        InputManager {
            state: InputState {
                cursos_pos: Vector2::zero(),
                key_state: [false; 512],
            },
            event_loop: event_loop,
        }
    }

    pub fn poll(&mut self, mut callback: impl FnMut(InputMessage)) {
        let state = &mut self.state;

        // https://docs.rs/winit/0.19.0/winit/enum.Event.html
        self.event_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                // Window
                WindowEvent::CloseRequested => {
                    callback(InputMessage::CloseRequested);
                },

                // Keyboard
                WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(key) => {
                        let index = input.scancode as usize;
                        match input.state {
                            ElementState::Pressed => {
                                if !state.key_state[index] {
                                    callback(InputMessage::KeyPressed(key));
                                    state.key_state[index] = true;
                                }
                            },
                            ElementState::Released => {
                                if state.key_state[index] {
                                    callback(InputMessage::KeyReleased(key));
                                    state.key_state[index] = false;
                                }
                            },
                        }
                    },
                    None => {},
                },

                // Cursor
                WindowEvent::CursorEntered { .. } => {
                    callback(InputMessage::CursorEntered);
                },
                WindowEvent::CursorLeft { .. } => {
                    callback(InputMessage::CursorLeft);
                },
                WindowEvent::CursorMoved { position, .. } => {
                    let (x, y) = position.into();
                    state.cursos_pos = Vector2::new(x, y);
                },

                // Mouse
                WindowEvent::MouseInput {
                    state: button_state,
                    button,
                    ..
                } => {
                    let message = match button_state {
                        ElementState::Pressed => InputMessage::CursorPressed(button, state.cursos_pos),
                        ElementState::Released => InputMessage::CursorReleased(button, state.cursos_pos),
                    };
                    callback(message);
                },
                _ => {},
            },
            _ => {},
        });
    }
}
