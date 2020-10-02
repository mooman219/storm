mod message;

pub use crate::input::message::*;

use crate::{Engine, Program};
use cgmath::prelude::*;
use cgmath::*;
use glutin::event::WindowEvent;

pub struct InputConverter {
    window_size: Vector2<f32>,
    cursor_pos: Vector2<f32>,
}

impl InputConverter {
    pub fn new(window_size: Vector2<f32>) -> InputConverter {
        InputConverter {
            window_size,
            cursor_pos: Vector2::zero(),
        }
    }

    pub fn push(&mut self, event: WindowEvent, program: &mut impl Program, engine: &mut Engine) {
        match event {
            // Window
            WindowEvent::CloseRequested => program.input(InputMessage::CloseRequested, engine),
            WindowEvent::Resized(size) => {
                self.window_size = Vector2::new(size.width as f32, size.height as f32);
                program.input(InputMessage::WindowResized(self.window_size), engine);
            }

            // Keyboard
            WindowEvent::KeyboardInput {
                input,
                ..
            } => {
                if let Some(keycode) = input.virtual_keycode {
                    match input.state {
                        glutin::event::ElementState::Pressed => {
                            program.input(InputMessage::KeyPressed(keycode), engine);
                        }
                        glutin::event::ElementState::Released => {
                            program.input(InputMessage::KeyReleased(keycode), engine);
                        }
                    }
                }
            }

            // Cursor
            WindowEvent::CursorMoved {
                position,
                ..
            } => {
                let cursor_pos = Vector2::new(
                    position.x as f32 - (self.window_size.x / 2.0),
                    -position.y as f32 + (self.window_size.y / 2.0),
                );
                let delta = cursor_pos - self.cursor_pos;
                self.cursor_pos = cursor_pos;
                program.input(
                    InputMessage::CursorMoved {
                        pos: self.cursor_pos,
                        delta,
                    },
                    engine,
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
                    program.input(InputMessage::CursorScroll(ScrollDirection::Left), engine);
                } else if x > 0.0 {
                    program.input(InputMessage::CursorScroll(ScrollDirection::Right), engine);
                }
                if y < 0.0 {
                    program.input(InputMessage::CursorScroll(ScrollDirection::Down), engine);
                } else if y > 0.0 {
                    program.input(InputMessage::CursorScroll(ScrollDirection::Up), engine);
                }
            }
            WindowEvent::MouseInput {
                state,
                button,
                ..
            } => match state {
                glutin::event::ElementState::Pressed => {
                    program.input(
                        InputMessage::CursorPressed {
                            button,
                            pos: self.cursor_pos,
                        },
                        engine,
                    );
                }
                glutin::event::ElementState::Released => {
                    program.input(
                        InputMessage::CursorReleased {
                            button,
                            pos: self.cursor_pos,
                        },
                        engine,
                    );
                }
            },
            WindowEvent::CursorEntered {
                ..
            } => {
                program.input(InputMessage::CursorEntered, engine);
            }
            WindowEvent::CursorLeft {
                ..
            } => {
                program.input(InputMessage::CursorLeft, engine);
            }
            _ => {}
        }
    }
}
