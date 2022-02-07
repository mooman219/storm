use crate::event::ScrollDirection;
use crate::graphics::{graphics, OpenGLWindowContract};
use crate::{App, Context};
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
    focused: bool,
}

impl EventConverter {
    pub(crate) fn new() -> EventConverter {
        let window = graphics().window();
        EventConverter {
            scale_factor: window.scale_factor(),
            physical_size: window.physical_size(),
            logical_size: window.logical_size(),
            physical_cursor_pos: Vector2::zero(),
            normalized_cursor_pos: Vector2::zero(),
            pressed_keys: [false; 256],
            focused: false,
        }
    }

    pub(crate) fn push<A: App>(&mut self, event: WinitEvent<()>, ctx: &mut Context<A>, app: &mut A) {
        match event {
            WinitEvent::DeviceEvent {
                event,
                ..
            } => match event {
                DeviceEvent::MouseMotion {
                    delta,
                } => {
                    let delta = Vector2::new(delta.0 as f32, -delta.1 as f32);
                    app.on_cursor_delta(ctx, delta, self.focused);
                }
                _ => {}
            },
            WinitEvent::WindowEvent {
                event,
                ..
            } => {
                match event {
                    WindowEvent::Focused(focused) => {
                        self.focused = focused;
                        app.on_window_focused(ctx, self.focused);
                    }
                    WindowEvent::CloseRequested => app.on_close_requested(ctx),
                    WindowEvent::Resized(physical_size) => {
                        self.physical_size =
                            Vector2::new(physical_size.width as f32, physical_size.height as f32);
                        self.logical_size = self.physical_size / self.scale_factor;

                        graphics().resize_viewport(self.physical_size, self.logical_size);
                        app.on_window_resized(ctx, self.physical_size, self.logical_size, self.scale_factor);
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        new_inner_size,
                    } => {
                        self.scale_factor = scale_factor as f32;
                        self.physical_size =
                            Vector2::new(new_inner_size.width as f32, new_inner_size.height as f32);
                        self.logical_size = self.physical_size / self.scale_factor;

                        graphics().resize_viewport(self.physical_size, self.logical_size);
                        app.on_window_resized(ctx, self.physical_size, self.logical_size, self.scale_factor);
                    }

                    // Keyboard
                    WindowEvent::ReceivedCharacter(char) => {
                        app.on_received_character(ctx, char);
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
                                        app.on_key_pressed(ctx, keycode, *is_pressed);
                                        *is_pressed = true;
                                    } else {
                                        app.on_key_pressed(ctx, keycode, false);
                                    }
                                }
                                winit::event::ElementState::Released => {
                                    if val < 256 {
                                        self.pressed_keys[val as usize] = false;
                                    }
                                    app.on_key_released(ctx, keycode);
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
                        app.on_cursor_moved(ctx, self.physical_cursor_pos, self.normalized_cursor_pos);
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
                            app.on_cursor_scroll(ctx, ScrollDirection::Left);
                        } else if x > 0.0 {
                            app.on_cursor_scroll(ctx, ScrollDirection::Right);
                        }
                        if y < 0.0 {
                            app.on_cursor_scroll(ctx, ScrollDirection::Down);
                        } else if y > 0.0 {
                            app.on_cursor_scroll(ctx, ScrollDirection::Up);
                        }
                    }
                    WindowEvent::MouseInput {
                        state,
                        button,
                        ..
                    } => match state {
                        winit::event::ElementState::Pressed => {
                            app.on_cursor_pressed(
                                ctx,
                                button,
                                self.physical_cursor_pos,
                                self.normalized_cursor_pos,
                            );
                        }
                        winit::event::ElementState::Released => {
                            app.on_cursor_released(
                                ctx,
                                button,
                                self.physical_cursor_pos,
                                self.normalized_cursor_pos,
                            );
                        }
                    },
                    WindowEvent::CursorEntered {
                        ..
                    } => app.on_cursor_entered(ctx),
                    WindowEvent::CursorLeft {
                        ..
                    } => app.on_cursor_left(ctx),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
