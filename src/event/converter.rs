use crate::ctx;
use crate::event::{Event, ScrollDirection};
use crate::graphics::OpenGLWindowContract;
use cgmath::prelude::*;
use cgmath::*;
use winit::event::{DeviceEvent, Event as WinitEvent, WindowEvent};

pub struct EventConverter {
    scale_factor: f32,
    physical_size: Vector2<f32>,
    logical_size: Vector2<f32>,
    physical_cursor_pos: Vector2<f32>,
    normalized_cursor_pos: Vector2<f32>,
    pressed_keys: [bool; 256],
}

impl EventConverter {
    pub fn new() -> EventConverter {
        let window = ctx().graphics().window();
        EventConverter {
            scale_factor: window.scale_factor(),
            physical_size: window.physical_size(),
            logical_size: window.logical_size(),
            physical_cursor_pos: Vector2::zero(),
            normalized_cursor_pos: Vector2::zero(),
            pressed_keys: [false; 256],
        }
    }

    pub fn push<T: 'static + FnMut(Event)>(&mut self, event: WinitEvent<()>, event_handler: &mut T) {
        match event {
            WinitEvent::DeviceEvent {
                event,
                ..
            } => match event {
                DeviceEvent::MouseMotion {
                    delta,
                } => {
                    let delta = Vector2::new(delta.0 as f32, -delta.1 as f32);
                    event_handler(Event::CursorDelta {
                        delta,
                    });
                }
                _ => {}
            },
            WinitEvent::WindowEvent {
                event,
                ..
            } => {
                match event {
                    // Window
                    WindowEvent::CloseRequested => event_handler(Event::CloseRequested),
                    WindowEvent::Resized(physical_size) => {
                        self.physical_size =
                            Vector2::new(physical_size.width as f32, physical_size.height as f32);
                        self.logical_size = self.physical_size / self.scale_factor;

                        ctx().graphics().resize_viewport(self.physical_size, self.logical_size);
                        event_handler(Event::WindowResized {
                            physical_size: self.physical_size,
                            logical_size: self.logical_size,
                            scale_factor: self.scale_factor,
                        });
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        new_inner_size,
                    } => {
                        self.scale_factor = scale_factor as f32;
                        self.physical_size =
                            Vector2::new(new_inner_size.width as f32, new_inner_size.height as f32);
                        self.logical_size = self.physical_size / self.scale_factor;

                        ctx().graphics().resize_viewport(self.physical_size, self.logical_size);
                        event_handler(Event::WindowResized {
                            physical_size: self.physical_size,
                            logical_size: self.logical_size,
                            scale_factor: self.scale_factor,
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
                            let val = keycode as u32;
                            match input.state {
                                winit::event::ElementState::Pressed => {
                                    if val < 256 {
                                        let is_pressed = &mut self.pressed_keys[val as usize];
                                        event_handler(Event::KeyPressed {
                                            keycode,
                                            is_repeat: *is_pressed,
                                        });
                                        *is_pressed = true;
                                    } else {
                                        event_handler(Event::KeyPressed {
                                            keycode,
                                            is_repeat: false,
                                        });
                                    }
                                }
                                winit::event::ElementState::Released => {
                                    if val < 256 {
                                        self.pressed_keys[val as usize] = false;
                                    }
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
                        let cursor_pos =
                            Vector2::new(position.x as f32, self.physical_size.y - position.y as f32);
                        let normalized_pos =
                            (cursor_pos.div_element_wise(self.physical_size) * 2.0) - Vector2::new(1.0, 1.0);

                        self.physical_cursor_pos = cursor_pos;
                        self.normalized_cursor_pos = normalized_pos;
                        event_handler(Event::CursorMoved {
                            physical_pos: self.physical_cursor_pos,
                            normalized_pos: self.normalized_cursor_pos,
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
                                physical_pos: self.physical_cursor_pos,
                                normalized_pos: self.normalized_cursor_pos,
                            });
                        }
                        winit::event::ElementState::Released => {
                            event_handler(Event::CursorReleased {
                                button,
                                physical_pos: self.physical_cursor_pos,
                                normalized_pos: self.normalized_cursor_pos,
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
            _ => {}
        }
    }
}
