use cgmath::*;
use glutin::ElementState;
use glutin::Event;
use glutin::EventsLoop;
use glutin::WindowEvent;
use input::message::*;

struct InputState {
    cursor_pos: Vector2<f32>,
    window_size: Vector2<f32>,
    key_state: [bool; 512],
}

pub struct InputManager {
    state: InputState,
    event_loop: EventsLoop,
}

impl InputManager {
    pub fn new(event_loop: EventsLoop, window_size: Vector2<f32>) -> InputManager {
        InputManager {
            state: InputState {
                cursor_pos: Vector2::zero(),
                window_size: window_size,
                key_state: [false; 512],
            },
            event_loop: event_loop,
        }
    }

    pub fn poll(&mut self, mut callback: impl FnMut(InputMessage)) {
        let state = &mut self.state;
        let last_cursor_pos = state.cursor_pos;
        self.event_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event,
                ..
            } => match event {
                // Window
                WindowEvent::CloseRequested => {
                    callback(InputMessage::CloseRequested);
                },
                WindowEvent::Resized(size) => {
                    state.window_size = Vector2::new(size.width as f32, size.height as f32);
                },

                // Keyboard
                WindowEvent::KeyboardInput {
                    input,
                    ..
                } => match input.virtual_keycode {
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
                WindowEvent::CursorEntered {
                    ..
                } => {
                    callback(InputMessage::CursorEntered);
                },
                WindowEvent::CursorLeft {
                    ..
                } => {
                    callback(InputMessage::CursorLeft);
                },
                WindowEvent::CursorMoved {
                    position,
                    ..
                } => {
                    state.cursor_pos = Vector2::new(
                        position.x as f32 - state.window_size.x / 2.0,
                        -position.y as f32 + state.window_size.y / 2.0,
                    );
                },
                WindowEvent::MouseInput {
                    state: button_state,
                    button,
                    ..
                } => {
                    let message = match button_state {
                        ElementState::Pressed => InputMessage::CursorPressed(button, state.cursor_pos),
                        ElementState::Released => InputMessage::CursorReleased(button, state.cursor_pos),
                    };
                    callback(message);
                },
                _ => {},
            },
            _ => {},
        });
        if state.cursor_pos != last_cursor_pos {
            callback(InputMessage::CursorMoved(state.cursor_pos));
        }
    }
}
