use crate::input::message::*;
use crate::utility::bounded_spsc;
use beryllium::*;
use cgmath::prelude::*;
use cgmath::*;
use std::collections::HashMap;
use std::time::Instant;

struct ControllerState {
    joystick_id: JoystickID,
}

struct InputState {
    cursor_pos: Vector2<f32>,
    window_size: Vector2<f32>,
}

pub struct InputServer<'a> {
    input_producer: bounded_spsc::Producer<InputMessage>,
    controllers: HashMap<JoystickID, Controller<'a>>,
    cursor_pos: Vector2<f32>,
    window_size: Vector2<f32>,
}

impl<'a> InputServer<'a> {
    pub fn new(input_producer: bounded_spsc::Producer<InputMessage>) -> InputServer<'a> {
        InputServer {
            input_producer,
            cursor_pos: Vector2::zero(),
            controllers: HashMap::new(),
            window_size: Vector2::zero(),
        }
    }

    pub fn tick(&mut self, sdl: &'a SDLToken) {
        let last_cursor_pos = self.cursor_pos;
        let last_window_size = self.window_size;
        while let Some(event) = sdl.poll_event() {
            match event {
                // Window
                Event::WindowClosed {
                    ..
                } => {
                    self.input_producer.push(InputMessage::CloseRequested);
                }
                Event::WindowSizeChanged {
                    width,
                    height,
                    ..
                } => {
                    self.window_size = Vector2::new(width as f32, height as f32);
                }
                
                Event::ControllerDeviceAdded { joystick_id, .. } => {
                    self.controllers.insert(joystick_id, sdl.open_controller(joystick_id).unwrap());
                }
                Event::ControllerButton { pressed, button, joystick_id, .. } => {
                    if pressed {
                        self.input_producer.push(InputMessage::ButtonPressed(joystick_id, button));
                    } else {
                        self.input_producer.push(InputMessage::ButtonReleased(joystick_id, button));
                    }
                }

                // Keyboard
                Event::Keyboard {
                    is_key_down,
                    repeat_count,
                    key_info,
                    ..
                } => {
                    if repeat_count == 0 {
                        if let Some(keycode) = key_info.keycode {
                            if is_key_down {
                                self.input_producer.push(InputMessage::KeyPressed(keycode));
                            } else {
                                self.input_producer.push(InputMessage::KeyReleased(keycode));
                            }
                        }
                    }
                }

                // Cursor
                Event::MouseMotion {
                    x,
                    y,
                    ..
                } => {
                    self.cursor_pos = Vector2::new(
                        x as f32 - self.window_size.x / 2.0,
                        -y as f32 + self.window_size.y / 2.0,
                    );
                }
                Event::MouseWheel {
                    mut x,
                    mut y,
                    is_flipped,
                    ..
                } => {
                    if is_flipped {
                        x *= -1;
                        y *= -1;
                    }
                    if x < 0 {
                        self.input_producer.push(InputMessage::CursorScroll(ScrollDirection::Left));
                    } else if x > 0 {
                        self.input_producer.push(InputMessage::CursorScroll(ScrollDirection::Right));
                    }
                    if y < 0 {
                        self.input_producer.push(InputMessage::CursorScroll(ScrollDirection::Down));
                    } else if y > 0 {
                        self.input_producer.push(InputMessage::CursorScroll(ScrollDirection::Up));
                    }
                }
                Event::MouseButtonEvent {
                    is_pressed,
                    button,
                    ..
                } => {
                    if is_pressed {
                        self.input_producer.push(InputMessage::CursorPressed {
                            button,
                            pos: self.cursor_pos,
                        });
                    } else {
                        self.input_producer.push(InputMessage::CursorReleased {
                            button,
                            pos: self.cursor_pos,
                        });
                    }
                }
                Event::MouseEnteredWindow {
                    ..
                } => {
                    self.input_producer.push(InputMessage::CursorEntered);
                }
                Event::MouseLeftWindow {
                    ..
                } => {
                    self.input_producer.push(InputMessage::CursorLeft);
                }
                _ => {}
            }
        }
        if self.cursor_pos != last_cursor_pos {
            let delta = self.cursor_pos - last_cursor_pos;
            self.input_producer.push(InputMessage::CursorMoved {
                pos: self.cursor_pos,
                delta,
            });
        }
        if self.window_size != last_window_size {
            self.input_producer.push(InputMessage::WindowResized(self.window_size));
        }
    }
}
