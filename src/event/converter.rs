use crate::event::{Event, ScrollDirection};
use crate::Context;
use cgmath::prelude::*;
use cgmath::*;
use winit::event::WindowEvent;

pub struct EventConverter {
    scale_factor: f32,
    physical_size: Vector2<f32>,
    logical_size: Vector2<f32>,
    cursor_pos: Vector2<f32>,
}

impl EventConverter {
    pub fn new(scale_factor: f32, physical_size: Vector2<f32>) -> EventConverter {
        EventConverter {
            scale_factor,
            physical_size,
            logical_size: physical_size / scale_factor,
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
            WindowEvent::Resized(physical_size) => {
                self.physical_size = Vector2::new(physical_size.width as f32, physical_size.height as f32);
                self.logical_size = self.physical_size / self.scale_factor;

                context.window_resize(self.physical_size, self.logical_size);
                event_handler(
                    Event::WindowResized {
                        physical_size: self.physical_size,
                        logical_size: self.logical_size,
                    },
                    context,
                );
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                new_inner_size,
            } => {
                self.scale_factor = scale_factor as f32;
                self.physical_size = Vector2::new(new_inner_size.width as f32, new_inner_size.height as f32);
                self.logical_size = self.physical_size / self.scale_factor;

                context.window_resize(self.physical_size, self.logical_size);
                event_handler(
                    Event::WindowResized {
                        physical_size: self.physical_size,
                        logical_size: self.logical_size,
                    },
                    context,
                );
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
                let cursor_pos = Vector2::new(position.x as f32, self.physical_size.y - position.y as f32);
                let delta = (cursor_pos - self.cursor_pos) / self.scale_factor;
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
