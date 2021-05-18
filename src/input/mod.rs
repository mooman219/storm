mod message;

pub use self::message::*;

use crate::Engine;
use cgmath::prelude::*;
use cgmath::*;
use winit::event::WindowEvent;

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

    pub fn push<T: 'static + FnMut(InputMessage, &mut Engine)>(
        &mut self,
        event: WindowEvent,
        event_handler: &mut T,
        engine: &mut Engine,
    ) {
        match event {
            // Window
            WindowEvent::CloseRequested => event_handler(InputMessage::CloseRequested, engine),
            WindowEvent::Resized(size) => {
                self.window_size = Vector2::new(size.width as f32, size.height as f32);
                event_handler(InputMessage::WindowResized(self.window_size), engine);
            }

            // Keyboard
            WindowEvent::ReceivedCharacter(char) => {
                event_handler(InputMessage::ReceivedCharacter(char), engine);
            }
            WindowEvent::KeyboardInput {
                input,
                ..
            } => {
                if let Some(keycode) = input.virtual_keycode {
                    match input.state {
                        winit::event::ElementState::Pressed => {
                            event_handler(InputMessage::KeyPressed(keycode), engine);
                        }
                        winit::event::ElementState::Released => {
                            event_handler(InputMessage::KeyReleased(keycode), engine);
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
                event_handler(
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
                    winit::event::MouseScrollDelta::LineDelta(x, y) => (x, y),
                    winit::event::MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                };
                if x < 0.0 {
                    event_handler(InputMessage::CursorScroll(ScrollDirection::Left), engine);
                } else if x > 0.0 {
                    event_handler(InputMessage::CursorScroll(ScrollDirection::Right), engine);
                }
                if y < 0.0 {
                    event_handler(InputMessage::CursorScroll(ScrollDirection::Down), engine);
                } else if y > 0.0 {
                    event_handler(InputMessage::CursorScroll(ScrollDirection::Up), engine);
                }
            }
            WindowEvent::MouseInput {
                state,
                button,
                ..
            } => match state {
                winit::event::ElementState::Pressed => {
                    event_handler(
                        InputMessage::CursorPressed {
                            button,
                            pos: self.cursor_pos,
                        },
                        engine,
                    );
                }
                winit::event::ElementState::Released => {
                    event_handler(
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
                event_handler(InputMessage::CursorEntered, engine);
            }
            WindowEvent::CursorLeft {
                ..
            } => {
                event_handler(InputMessage::CursorLeft, engine);
            }
            _ => {}
        }
    }
}
