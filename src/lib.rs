#![allow(dead_code, non_camel_case_types, non_snake_case)]

#[macro_use]
pub extern crate log;
extern crate alloc;

pub mod math;
pub mod time;

pub use crate::input::*;
pub use crate::types::*;
pub use cgmath;

mod input;
mod render;
mod text;
mod texture;
mod types;
mod utility;

use crate::input::InputConverter;
use crate::render::Renderer;
use crate::time::Instant;
use core::time::Duration;
use winit::event::Event;
use winit::event_loop::ControlFlow;

/// The main entry point into the Storm engine. All interactions with the engine are managed by the
/// API on this type. The engine is send, and can be moved between threads.
pub struct Engine {
    pub render: Renderer,
    stop: bool,
    control_flow: Option<ControlFlow>,
    last_update: Instant,
    wait_next: Instant,
    wait_periodic: Option<Duration>,
}

impl Engine {
    // 'static + FnMut(&mut Engine) -> FnMut(InputMessage, &mut Engine)
    pub fn start<T: 'static + FnMut(InputMessage, &mut Engine)>(
        desc: WindowSettings,
        event_handler_creator: fn(&mut Engine) -> T,
    ) {
        info!("Starting engine...");
        let event_loop = winit::event_loop::EventLoop::new();
        let render = Renderer::new(&desc, &event_loop);
        let mut input = InputConverter::new(render.current_logical_size());
        let mut engine = Engine {
            render,
            stop: false,
            control_flow: Some(ControlFlow::Poll),
            last_update: Instant::now(),
            wait_next: Instant::now(),
            wait_periodic: None,
        };
        info!("Starting handler...");
        let mut event_handler = event_handler_creator(&mut engine);
        info!("Starting loop...");
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event,
                    ..
                } => {
                    input.push(event, &mut event_handler, &mut engine);
                }
                Event::MainEventsCleared => {
                    let now = Instant::now();
                    if now >= engine.wait_next {
                        if let Some(duration) = engine.wait_periodic {
                            engine.wait_next = now + duration;
                            engine.control_flow = Some(ControlFlow::WaitUntil(engine.wait_next));
                        }
                        let delta = (now - engine.last_update).as_secs_f32();
                        event_handler(InputMessage::Update(delta), &mut engine);
                        engine.last_update = now;
                    }
                }
                Event::LoopDestroyed => {
                    engine.stop = true;
                }
                _ => {}
            }
            if engine.stop {
                *control_flow = ControlFlow::Exit;
            } else if let Some(next_control_flow) = engine.control_flow {
                *control_flow = next_control_flow;
                engine.control_flow = None;
            }
        });
    }

    /// Stops the engine after the next update.
    pub fn stop(&mut self) {
        info!("Stopping engine...");
        self.stop = true;
    }

    /// Prevents the update event from being sent for at least the duration. If a periodic wait is
    /// active, this wait will temporarily override only if it causes the next update event to
    /// happen later than the periodic wait would have.
    pub fn wait_for(&mut self, duration: Duration) {
        self.wait_until(Instant::now() + duration);
    }

    /// Prevents the update event from being sent until at least the given instant. If a periodic
    /// wait is active, this wait will temporarily override only if it causes the next update event
    /// to happen later than the periodic wait would have.
    pub fn wait_until(&mut self, instant: Instant) {
        if instant > self.wait_next {
            self.wait_next = instant;
            self.control_flow = Some(ControlFlow::WaitUntil(self.wait_next));
        }
    }

    /// Prevents the update event from being sent more frequently than the given duration. Set this
    /// to None to disable the periodic wait.
    pub fn wait_periodic(&mut self, duration: Option<Duration>) {
        self.wait_periodic = duration;
    }
}
