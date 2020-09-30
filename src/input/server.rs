use crate::input::message::*;
use crate::types::WindowSettings;
use crate::utility::bounded_spsc;
use cgmath::prelude::*;
use cgmath::*;
use glutin::event::WindowEvent;

struct InputState {
    cursor_pos: Vector2<f32>,
    window_size: Vector2<f32>,
}

pub struct InputServer {
    input_producer: bounded_spsc::Producer<InputMessage>,
    last_cursor_pos: Vector2<f32>,
    last_window_size: Vector2<f32>,
    cursor_pos: Vector2<f32>,
    window_size: Vector2<f32>,
}

impl InputServer {
    pub fn new(input_producer: bounded_spsc::Producer<InputMessage>, _: WindowSettings) -> InputServer {
        InputServer {
            input_producer,
            last_cursor_pos: Vector2::zero(),
            last_window_size: Vector2::zero(),
            cursor_pos: Vector2::zero(),
            window_size: Vector2::zero(),
        }
    }

    pub fn push(&mut self, event: WindowEvent) {
        match event {
            // Window
            WindowEvent::CloseRequested {} => {
                self.input_producer.push(InputMessage::CloseRequested);
            }
            WindowEvent::Resized(size) => {
                self.window_size = Vector2::new(size.width as f32, size.height as f32);
            }

            // Keyboard
            WindowEvent::KeyboardInput {
                input,
                ..
            } => {
                if let Some(keycode) = input.virtual_keycode {
                    println!("{:?}", input);
                    match input.state {
                        glutin::event::ElementState::Pressed => {
                            self.input_producer.push(InputMessage::KeyPressed(keycode))
                        }
                        glutin::event::ElementState::Released => {
                            self.input_producer.push(InputMessage::KeyReleased(keycode))
                        }
                    }
                }
            }

            // Cursor
            WindowEvent::CursorMoved {
                position,
                ..
            } => {
                self.cursor_pos = Vector2::new(
                    position.x as f32 - (self.window_size.x / 2.0),
                    -position.y as f32 + (self.window_size.y / 2.0),
                );
            }
            WindowEvent::MouseWheel {
                delta,
                ..
            } => {
                let (x, y) = match delta {
                    glutin::event::MouseScrollDelta::LineDelta(x, y) => (x, y),
                    glutin::event::MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                };
                if x < 0.0 {
                    self.input_producer.push(InputMessage::CursorScroll(ScrollDirection::Left));
                } else if x > 0.0 {
                    self.input_producer.push(InputMessage::CursorScroll(ScrollDirection::Right));
                }
                if y < 0.0 {
                    self.input_producer.push(InputMessage::CursorScroll(ScrollDirection::Down));
                } else if y > 0.0 {
                    self.input_producer.push(InputMessage::CursorScroll(ScrollDirection::Up));
                }
            }
            WindowEvent::MouseInput {
                state,
                button,
                ..
            } => match state {
                glutin::event::ElementState::Pressed => {
                    self.input_producer.push(InputMessage::CursorPressed {
                        button,
                        pos: self.cursor_pos,
                    });
                }
                glutin::event::ElementState::Released => {
                    self.input_producer.push(InputMessage::CursorReleased {
                        button,
                        pos: self.cursor_pos,
                    });
                }
            },
            WindowEvent::CursorEntered {
                ..
            } => {
                self.input_producer.push(InputMessage::CursorEntered);
            }
            WindowEvent::CursorLeft {
                ..
            } => {
                self.input_producer.push(InputMessage::CursorLeft);
            }
            _ => {}
        }
    }

    pub fn finalize(&mut self) {
        if self.cursor_pos != self.last_cursor_pos {
            let delta = self.cursor_pos - self.last_cursor_pos;
            self.input_producer.push(InputMessage::CursorMoved {
                pos: self.cursor_pos,
                delta,
            });
            self.last_cursor_pos = self.cursor_pos;
        }
        if self.window_size != self.last_window_size {
            self.input_producer.push(InputMessage::WindowResized(self.window_size));
            self.last_window_size = self.window_size;
        }
    }
}
