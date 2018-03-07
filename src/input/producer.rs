use bounded_spsc_queue::Producer;
use glutin;
use glutin::EventsLoop;
use glutin::VirtualKeyCode;
use input::*;
use std::mem;

pub struct InputProducer {
    input_producer: Producer<InputFrame>,
    frame: InputFrame,
    event_loop: EventsLoop,
    is_active: bool,
    resize_message: Option<ResizeMessage>,
}

impl InputProducer {
    pub fn new(event_loop: EventsLoop, input_producer: Producer<InputFrame>) -> InputProducer {
        InputProducer {
            input_producer: input_producer,
            frame: InputFrame::new(),
            event_loop: event_loop,
            is_active: true,
            resize_message: None,
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn next_resize(&mut self) -> Option<ResizeMessage> {
        let message = self.resize_message;
        self.resize_message = None;
        message
    }

    pub fn tick(&mut self) {
        // Disjoint References
        let is_active = &mut self.is_active;
        let resize_message = &mut self.resize_message;
        // Run the event loop
        self.event_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Resized(w, h) => {
                    *resize_message = Some(ResizeMessage {
                        width: w,
                        height: h,
                    });
                },
                glutin::WindowEvent::Closed => *is_active = false,
                glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(key) => {
                        if key == VirtualKeyCode::Escape {
                            *is_active = false;
                        }
                    },
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        });
    }

    pub fn send(&mut self) {
        let mut frame = InputFrame::new();
        mem::swap(&mut frame, &mut self.frame);
        self.input_producer.push(frame);
    }
}
