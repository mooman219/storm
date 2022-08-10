use crate::asset::{AssetState, AssetStateContract};
use crate::audio::AudioState;
use crate::event::EventConverter;
use crate::graphics::{graphics, OpenGLState, OpenGLWindowContract, WindowSettings};
use crate::time::Instant;
use crate::App;
use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;
use log::info;
use winit::event::Event as WinitEvent;
use winit::event_loop::ControlFlow;

#[no_mangle]
static mut INITIALIZED: AtomicBool = AtomicBool::new(false);

/// The main entry point into the engine context. All state is initialized by this type.
pub struct Context<A: App> {
    // Global states
    assets: AssetState<A>,
    // Context state
    stop: bool,
    control_flow: Option<ControlFlow>,
    last_update: Instant,
    wait_next: Instant,
    wait_periodic: Option<Duration>,
}

/// Initializes the context. Graphics, audio, assets, and you app, are initialized by this function.
pub fn start<A: App>(desc: WindowSettings) -> ! {
    if unsafe { INITIALIZED.swap(true, Ordering::Relaxed) } {
        panic!("Start has already been called.");
    }

    init_logger(A::LOG_LEVEL);

    let event_loop = winit::event_loop::EventLoop::new();
    OpenGLState::init(&desc, &event_loop);
    AudioState::init();
    let assets = AssetState::init();
    let mut ctx = Context {
        assets,
        stop: false,
        control_flow: Some(ControlFlow::Poll),
        last_update: Instant::now(),
        wait_next: Instant::now(),
        wait_periodic: None,
    };
    let mut input = EventConverter::new();
    let mut app = A::new(&mut ctx);
    event_loop.run(move |event, _, control_flow| {
        match event {
            WinitEvent::DeviceEvent {
                ..
            } => {
                input.push(event, &mut ctx, &mut app);
            }
            WinitEvent::WindowEvent {
                ..
            } => {
                input.push(event, &mut ctx, &mut app);
            }
            WinitEvent::MainEventsCleared => {
                while let Some(response) = ctx.assets.next() {
                    response.call(&mut ctx, &mut app);
                }
                let now = Instant::now();
                if now >= ctx.wait_next {
                    {
                        profiling::scope!("storm_update");
                        if let Some(duration) = ctx.wait_periodic {
                            ctx.wait_next += duration;
                            if ctx.wait_next < now {
                                ctx.wait_next = now;
                            }
                            ctx.control_flow = Some(ControlFlow::WaitUntil(ctx.wait_next));
                        }
                        let delta = now - ctx.last_update;
                        ctx.last_update = now;
                        app.on_update(&mut ctx, delta.as_secs_f32());
                    }
                    graphics().window().swap_buffers();
                }
            }
            WinitEvent::LoopDestroyed => {
                ctx.stop = true;
            }
            _ => {}
        }
        if ctx.stop {
            *control_flow = ControlFlow::Exit;
        } else if let Some(next_control_flow) = ctx.control_flow {
            *control_flow = next_control_flow;
            ctx.control_flow = None;
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn init_logger(level: log::Level) {
    use simplelog::*;

    match TermLogger::init(
        level.to_level_filter(),
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    ) {
        Ok(_) => info!("Using the default logger: simplelog::loggers::termlog."),
        Err(_) => info!("Using the provided logger."),
    }
}

#[cfg(target_arch = "wasm32")]
fn init_logger(level: log::Level) {
    console_error_panic_hook::set_once();
    match console_log::init_with_level(level) {
        Ok(_) => info!("Using the default logger: console_log."),
        Err(_) => info!("Using the provided logger."),
    }
}

/// Event loop related functions.
impl<A: App> Context<A> {
    pub(crate) fn assets(&mut self) -> &mut AssetState<A> {
        &mut self.assets
    }

    /// Stops the context after the next update.
    pub fn request_stop(&mut self) {
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
