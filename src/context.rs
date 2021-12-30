use crate::asset::AssetState;
use crate::audio::AudioState;
use crate::event::{Event, EventConverter};
use crate::global::{ctx, Ctx};
use crate::render::{OpenGLState, OpenGLWindow};
use crate::time::{Instant, Timer};
use crate::{DisplayMode, WindowSettings};
use cgmath::Vector2;
use core::time::Duration;
use log::info;
use winit::event::Event as WinitEvent;
use winit::event_loop::ControlFlow;

#[cfg(not(target_arch = "wasm32"))]
fn init_logger() {
    use simplelog::*;

    match TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Stdout, ColorChoice::Auto) {
        Ok(_) => info!("Using the default logger: simplelog::loggers::termlog."),
        Err(_) => info!("Using the provided logger."),
    }
}

#[cfg(target_arch = "wasm32")]
fn init_logger() {
    console_error_panic_hook::set_once();
    match console_log::init_with_level(log::Level::Trace) {
        Ok(_) => info!("Using the default logger: console_log."),
        Err(_) => info!("Using the provided logger."),
    }
}

/// The main entry point into the engine context. All state is initialized by this type.
pub struct Context {
    window: OpenGLWindow,
    stop: bool,
    control_flow: Option<ControlFlow>,
    last_update: Instant,
    wait_next: Instant,
    wait_periodic: Option<Duration>,
}

impl Context {
    // 'static + FnMut(&mut Context) -> FnMut(InputMessage, &mut Context)
    pub fn start<T: 'static + FnMut(Event, &mut Context)>(
        desc: WindowSettings,
        event_handler_creator: fn(&mut Context) -> T,
    ) -> ! {
        init_logger();

        let assets = AssetState::init();
        let audio = AudioState::init();
        let event_loop = winit::event_loop::EventLoop::new();
        let (graphics, window) = OpenGLState::init(&desc, &event_loop);
        Ctx::init(graphics, audio, assets);

        let mut input = EventConverter::new(window.scale_factor(), window.logical_size());
        let mut context = Context {
            window,
            stop: false,
            control_flow: Some(ControlFlow::Poll),
            last_update: Instant::now(),
            wait_next: Instant::now(),
            wait_periodic: None,
        };
        let mut event_handler = event_handler_creator(&mut context);
        let mut update_timer = Timer::new("Event::Update");
        event_loop.run(move |event, _, control_flow| {
            match event {
                WinitEvent::WindowEvent {
                    event,
                    ..
                } => {
                    input.push(event, &mut event_handler, &mut context);
                }
                WinitEvent::MainEventsCleared => {
                    while let Some(read) = ctx().assets().try_pop_read() {
                        event_handler(Event::AssetRead(read), &mut context);
                    }

                    let now = Instant::now();
                    if now >= context.wait_next {
                        if let Some(duration) = context.wait_periodic {
                            context.wait_next += duration;
                            if context.wait_next < now {
                                context.wait_next = now;
                            }
                            context.control_flow = Some(ControlFlow::WaitUntil(context.wait_next));
                        }
                        let delta = now - context.last_update;
                        context.last_update = now;

                        update_timer.start();
                        event_handler(Event::Update(delta.as_secs_f32()), &mut context);
                        context.window.swap_buffers();
                        update_timer.stop();
                    }
                }
                WinitEvent::LoopDestroyed => {
                    context.stop = true;
                }
                _ => {}
            }
            if context.stop {
                *control_flow = ControlFlow::Exit;
            } else if let Some(next_control_flow) = context.control_flow {
                *control_flow = next_control_flow;
                context.control_flow = None;
            }
        });
    }

    pub(crate) fn window_resize(&mut self, physical: Vector2<f32>, logical: Vector2<f32>) {
        let gpu = ctx().graphics();
        gpu.resize(physical, logical);
    }

    /// Sets the title of the window.
    pub fn window_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    /// Gets the logical size of the window.
    pub fn window_logical_size(&self) -> Vector2<f32> {
        self.window.logical_size()
    }

    /// Gets the physical size of the window.
    pub fn window_physical_size(&self) -> Vector2<f32> {
        self.window.physical_size()
    }

    /// Sets the display mode of the window.
    pub fn window_display_mode(&mut self, display_mode: DisplayMode) {
        self.window.set_display_mode(display_mode);
    }

    /// Stops the context after the next update.
    pub fn stop(&mut self) {
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
