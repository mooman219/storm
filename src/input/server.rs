use crate::input::message::*;
use crate::utility::bounded_spsc;
use beryllium::*;
use cgmath::*;

struct InputState {
    cursor_pos: Vector2<f32>,
    window_size: Vector2<f32>,
}

pub struct InputServer {
    input_producer: bounded_spsc::Producer<InputMessage>,
    cursor_pos: Vector2<f32>,
    window_size: Vector2<f32>,
}

impl InputServer {
    pub fn new(
        input_producer: bounded_spsc::Producer<InputMessage>,
        window_size: Vector2<f32>,
    ) -> InputServer {
        InputServer {
            input_producer: input_producer,
            cursor_pos: Vector2::zero(),
            window_size: window_size,
        }
    }

    pub fn tick(&mut self, sdl: &SDLToken) {
        let last_cursor_pos = self.cursor_pos;
        // let last_window_size = self.window_size;
        while let Some(event) = sdl.poll_event() {
            match event {
                // Window
                Event::WindowClosed {
                    ..
                } => {
                    self.input_producer.push(InputMessage::CloseRequested);
                },
                Event::WindowSizeChanged {
                    width,
                    height,
                    ..
                } => {
                    self.window_size = Vector2::new(width as f32, height as f32);
                },

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
                },

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
                },
                Event::MouseButtonEvent {
                    is_pressed,
                    button,
                    ..
                } => {
                    if is_pressed {
                        self.input_producer.push(InputMessage::CursorPressed(button, self.cursor_pos));
                    } else {
                        self.input_producer.push(InputMessage::CursorReleased(button, self.cursor_pos));
                    }
                },
                Event::MouseEnteredWindow {
                    ..
                } => {
                    self.input_producer.push(InputMessage::CursorEntered);
                },
                Event::MouseLeftWindow {
                    ..
                } => {
                    self.input_producer.push(InputMessage::CursorLeft);
                },
                _ => {},
            }
        }
        if self.cursor_pos != last_cursor_pos {
            self.input_producer.push(InputMessage::CursorMoved(self.cursor_pos));
        }
    }
}
