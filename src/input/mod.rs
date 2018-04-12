pub mod message;

use bounded_spsc_queue;
use cgmath::*;
use channel::consume_spsc;
use channel::replace_spsc;
use glutin;
use glutin::ElementState;
use glutin::EventsLoop;
use glutin::MouseButton;
use input::message::*;
use std::thread::sleep;
use std::time::Duration;
use time::timer::*;

struct InputState {
    is_active: bool,
    cursos_pos: Vector2<f32>,
    key_states: [bool; 512],
}

pub fn start(
    mut event_loop: EventsLoop,
    input_producer: bounded_spsc_queue::Producer<InputFrame>,
    resize_producer: consume_spsc::Producer<Vector2<u32>>,
    cursor_producer: replace_spsc::Producer<Vector2<f32>>,
) {
    // Create the input state.
    let mut state = InputState {
        is_active: true,
        cursos_pos: Vector2::new(0f32, 0f32),
        key_states: [false; 512],
    };

    // Log input timings.
    let mut timer_input = Timer::new("[I] Frame");
    while state.is_active {
        // Start timing.
        timer_input.start();
        // Run the event loop to record input events.
        event_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Resized(w, h) => {
                    resize_producer.set(Vector2::new(w, h));
                },
                glutin::WindowEvent::Closed => state.is_active = false,
                glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(key) => {
                        // pub scancode: u32,
                        let index = input.scancode as usize;
                        match input.state {
                            ElementState::Pressed => {
                                if !state.key_states[index] {
                                    input_producer.push(InputFrame::KeyPressed(key));
                                    state.key_states[index] = true;
                                }
                            },
                            ElementState::Released => {
                                if state.key_states[index] {
                                    input_producer.push(InputFrame::KeyReleased(key));
                                    state.key_states[index] = false;
                                }
                            },
                        }
                    },
                    None => {},
                },
                glutin::WindowEvent::CursorMoved { position, .. } => {
                    let (x, y) = position;
                    state.cursos_pos = Vector2::new(x as f32, y as f32);
                    cursor_producer.set(state.cursos_pos);
                },
                glutin::WindowEvent::CursorEntered { .. } => {
                    input_producer.push(InputFrame::CursorEntered);
                },
                glutin::WindowEvent::CursorLeft { .. } => {
                    input_producer.push(InputFrame::CursorLeft);
                },
                glutin::WindowEvent::MouseInput {
                    state: button_state,
                    button,
                    ..
                } => {
                    let button = match button {
                        MouseButton::Left => CursorButton::Left,
                        MouseButton::Right => CursorButton::Right,
                        MouseButton::Middle => CursorButton::Middle,
                        MouseButton::Other(value) => CursorButton::Other(value),
                    };
                    let message = match button_state {
                        ElementState::Pressed => InputFrame::CursorPressed(button, state.cursos_pos),
                        ElementState::Released => InputFrame::CursorReleased(button, state.cursos_pos),
                    };
                    input_producer.push(message);
                },
                // Other events: https://docs.rs/glutin/0.13.1/glutin/enum.WindowEvent.html
                _ => (),
            },
            // Other events: https://docs.rs/glutin/0.13.1/glutin/enum.Event.html
            _ => (),
        });
        // Finish timing.
        timer_input.stop();
        // Sleep to avoid pegging a core.
        sleep(Duration::new(0, 100));
    }
}
