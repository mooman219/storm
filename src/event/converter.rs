use crate::event::{Event, ScrollDirection};
use crate::Context;
use cgmath::prelude::*;
use cgmath::*;
use winit::event::WindowEvent;

pub struct EventConverter {
    window_size: Vector2<f32>,
    cursor_pos: Vector2<f32>,
}

impl EventConverter {
    pub fn new(window_size: Vector2<f32>) -> EventConverter {
        EventConverter {
            window_size,
            cursor_pos: Vector2::zero(),
        }
    }

    pub fn push<T: 'static + FnMut(Event, &mut Context)>(
        &mut self,
        event: WindowEvent,
        event_handler: &mut T,
        context: &mut Context,
    ) {
        match event {
            // Window
            WindowEvent::CloseRequested => event_handler(Event::CloseRequested, context),
            WindowEvent::Resized(size) => {
                context.window_check_resize();
                self.window_size = Vector2::new(size.width as f32, size.height as f32);
                event_handler(Event::WindowResized(self.window_size), context);
            }
            WindowEvent::ScaleFactorChanged {
                ..
            } => {
                context.window_check_resize();
            }

            // Keyboard
            WindowEvent::ReceivedCharacter(char) => {
                event_handler(Event::ReceivedCharacter(char), context);
            }
            WindowEvent::KeyboardInput {
                input,
                ..
            } => {
                if let Some(keycode) = input.virtual_keycode {
                    match input.state {
                        winit::event::ElementState::Pressed => {
                            event_handler(Event::KeyPressed(keycode), context);
                        }
                        winit::event::ElementState::Released => {
                            event_handler(Event::KeyReleased(keycode), context);
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
                    Event::CursorMoved {
                        pos: self.cursor_pos,
                        delta,
                    },
                    context,
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
                    event_handler(Event::CursorScroll(ScrollDirection::Left), context);
                } else if x > 0.0 {
                    event_handler(Event::CursorScroll(ScrollDirection::Right), context);
                }
                if y < 0.0 {
                    event_handler(Event::CursorScroll(ScrollDirection::Down), context);
                } else if y > 0.0 {
                    event_handler(Event::CursorScroll(ScrollDirection::Up), context);
                }
            }
            WindowEvent::MouseInput {
                state,
                button,
                ..
            } => match state {
                winit::event::ElementState::Pressed => {
                    event_handler(
                        Event::CursorPressed {
                            button,
                            pos: self.cursor_pos,
                        },
                        context,
                    );
                }
                winit::event::ElementState::Released => {
                    event_handler(
                        Event::CursorReleased {
                            button,
                            pos: self.cursor_pos,
                        },
                        context,
                    );
                }
            },
            WindowEvent::CursorEntered {
                ..
            } => {
                event_handler(Event::CursorEntered, context);
            }
            WindowEvent::CursorLeft {
                ..
            } => {
                event_handler(Event::CursorLeft, context);
            }
            _ => {}
        }
    }
}
