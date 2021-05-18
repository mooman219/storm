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
use winit::event::Event;
use winit::event_loop::ControlFlow;

/// The main entry point into the Storm engine. All interactions with the engine are managed by the
/// API on this type. The engine is send, and can be moved between threads.
pub struct Engine {
    pub render: Renderer,
    control_flow: ControlFlow,
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
            control_flow: ControlFlow::Poll,
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
                    event_handler(InputMessage::MainEventsCleared, &mut engine);
                }
                Event::LoopDestroyed => {
                    engine.control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
            *control_flow = engine.control_flow;
        });
    }

    // TODO: Audio

    /// Stops the engine after the next update.
    pub fn stop(&mut self) {
        info!("Stopping engine...");
        self.control_flow = ControlFlow::Exit;
    }
}
