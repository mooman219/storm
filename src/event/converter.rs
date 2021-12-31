use crate::ctx;
use crate::event::{Event, ScrollDirection};
use crate::graphics::OpenGLWindowContract;
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
    pub fn new() -> EventConverter {
        let window = ctx().graphics().window();
        EventConverter {
            scale_factor: window.scale_factor(),
            physical_size: window.physical_size(),
            logical_size: window.logical_size(),
            cursor_pos: Vector2::zero(),
        }
    }

    pub fn push<T: 'static + FnMut(Event)>(&mut self, event: WindowEvent, event_handler: &mut T) {
        match event {
            // Window
            WindowEvent::CloseRequested => event_handler(Event::CloseRequested),
            WindowEvent::Resized(physical_size) => {
                self.physical_size = Vector2::new(physical_size.width as f32, physical_size.height as f32);
                self.logical_size = self.physical_size / self.scale_factor;

                ctx().graphics().resize_viewport(self.physical_size, self.logical_size);
                event_handler(Event::WindowResized {
                    physical_size: self.physical_size,
                    logical_size: self.logical_size,
                });
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                new_inner_size,
            } => {
                self.scale_factor = scale_factor as f32;
                self.physical_size = Vector2::new(new_inner_size.width as f32, new_inner_size.height as f32);
                self.logical_size = self.physical_size / self.scale_factor;

                ctx().graphics().resize_viewport(self.physical_size, self.logical_size);
                event_handler(Event::WindowResized {
                    physical_size: self.physical_size,
                    logical_size: self.logical_size,
                });
            }

            // Keyboard
            WindowEvent::ReceivedCharacter(char) => {
                event_handler(Event::ReceivedCharacter(char));
            }
            WindowEvent::KeyboardInput {
                input,
                ..
            } => {
                if let Some(keycode) = input.virtual_keycode {
                    match input.state {
                        winit::event::ElementState::Pressed => {
                            event_handler(Event::KeyPressed(keycode));
                        }
                        winit::event::ElementState::Released => {
                            event_handler(Event::KeyReleased(keycode));
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
                event_handler(Event::CursorMoved {
                    pos: self.cursor_pos,
                    delta,
                });
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
                    event_handler(Event::CursorScroll(ScrollDirection::Left));
                } else if x > 0.0 {
                    event_handler(Event::CursorScroll(ScrollDirection::Right));
                }
                if y < 0.0 {
                    event_handler(Event::CursorScroll(ScrollDirection::Down));
                } else if y > 0.0 {
                    event_handler(Event::CursorScroll(ScrollDirection::Up));
                }
            }
            WindowEvent::MouseInput {
                state,
                button,
                ..
            } => match state {
                winit::event::ElementState::Pressed => {
                    event_handler(Event::CursorPressed {
                        button,
                        pos: self.cursor_pos,
                    });
                }
                winit::event::ElementState::Released => {
                    event_handler(Event::CursorReleased {
                        button,
                        pos: self.cursor_pos,
                    });
                }
            },
            WindowEvent::CursorEntered {
                ..
            } => {
                event_handler(Event::CursorEntered);
            }
            WindowEvent::CursorLeft {
                ..
            } => {
                event_handler(Event::CursorLeft);
            }
            _ => {}
        }
    }
}
