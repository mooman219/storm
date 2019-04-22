use cgmath::*;
use glutin::ControlFlow;
use glutin::ElementState;
use glutin::Event;
use glutin::EventsLoop;
use glutin::WindowEvent;
use input::message::*;
use utility::bounded_spsc;

struct InputState {
    cursos_pos: Vector2<f64>,
    key_state: [bool; 512],
    input_producer: bounded_spsc::Producer<InputMessage>,
}

pub struct InputServer {
    state: InputState,
    event_loop: EventsLoop,
}

impl InputServer {
    pub fn new(event_loop: EventsLoop, input_producer: bounded_spsc::Producer<InputMessage>) -> InputServer {
        InputServer {
            state: InputState {
                cursos_pos: Vector2::zero(),
                key_state: [false; 512],
                input_producer: input_producer,
            },
            event_loop: event_loop,
        }
    }

    pub fn run_forever(&mut self) {
        let state = &mut self.state;

        // https://docs.rs/winit/0.19.0/winit/enum.Event.html
        self.event_loop.run_forever(|event| {
            match event {
                Event::WindowEvent {
                    event,
                    ..
                } => match event {
                    // Window
                    WindowEvent::CloseRequested => {
                        state.input_producer.push(InputMessage::CloseRequested);
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
                                        state.input_producer.push(InputMessage::KeyPressed(key));
                                        state.key_state[index] = true;
                                    }
                                },
                                ElementState::Released => {
                                    if state.key_state[index] {
                                        state.input_producer.push(InputMessage::KeyReleased(key));
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
                        state.input_producer.push(InputMessage::CursorEntered);
                    },
                    WindowEvent::CursorLeft {
                        ..
                    } => {
                        state.input_producer.push(InputMessage::CursorLeft);
                    },
                    WindowEvent::CursorMoved {
                        position,
                        ..
                    } => {
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
                        state.input_producer.push(message);
                    },
                    _ => {},
                },
                _ => {},
            };
            ControlFlow::Continue
        });
    }
}
