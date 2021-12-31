use crate::asset::{AssetState, AssetStateContract};
use crate::audio::AudioState;
use crate::event::{Event, EventConverter};
use crate::render::{OpenGLState, OpenGLWindowContract};
use crate::time::{Instant, Timer};
use crate::WindowSettings;
use core::time::Duration;
use log::info;
use winit::event::Event as WinitEvent;
use winit::event_loop::ControlFlow;

#[no_mangle]
static mut CTX: Option<Context> = None;

/// Retrieves the global state context.
#[inline(always)]
pub(crate) fn ctx() -> &'static mut Context {
    unsafe {
        if let Some(ctx) = CTX.as_mut() {
            return ctx;
        }
        panic!("State not initialized")
    }
}

/// The main entry point into the engine context. All state is initialized by this type.
pub(crate) struct Context {
    // Global states
    graphics: OpenGLState,
    audio: AudioState,
    assets: AssetState,
    // Context state
    stop: bool,
    control_flow: Option<ControlFlow>,
    last_update: Instant,
    wait_next: Instant,
    wait_periodic: Option<Duration>,
}

impl Context {
    #[inline(always)]
    pub(crate) fn graphics(&mut self) -> &mut OpenGLState {
        &mut self.graphics
    }

    #[inline(always)]
    pub(crate) fn audio(&mut self) -> &mut AudioState {
        &mut self.audio
    }

    #[inline(always)]
    pub(crate) fn assets(&mut self) -> &mut AssetState {
        &mut self.assets
    }
}

/// Initializes the context. Graphics, audio, assets, are initialized by this function.
pub fn start<T: 'static + FnMut(Event)>(desc: WindowSettings, event_handler_creator: fn() -> T) -> ! {
    if unsafe { CTX.is_some() } {
        panic!("Start has already been called.");
    }

    init_logger();

    let assets = AssetState::init();
    let audio = AudioState::init();
    let event_loop = winit::event_loop::EventLoop::new();
    let graphics = OpenGLState::init(&desc, &event_loop);
    unsafe {
        CTX = Some(Context {
            graphics,
            audio,
            assets,
            stop: false,
            control_flow: Some(ControlFlow::Poll),
            last_update: Instant::now(),
            wait_next: Instant::now(),
            wait_periodic: None,
        })
    };

    let mut input = EventConverter::new();
    let mut event_handler = event_handler_creator();
    let mut update_timer = Timer::new("Event::Update");
    event_loop.run(move |event, _, control_flow| {
        let ctx = ctx();
        match event {
            WinitEvent::WindowEvent {
                event,
                ..
            } => {
                input.push(event, &mut event_handler);
            }
            WinitEvent::MainEventsCleared => {
                while let Some(read) = ctx.assets().try_pop_read() {
                    event_handler(Event::AssetRead(read));
                }

                let now = Instant::now();
                if now >= ctx.wait_next {
                    if let Some(duration) = ctx.wait_periodic {
                        ctx.wait_next += duration;
                        if ctx.wait_next < now {
                            ctx.wait_next = now;
                        }
                        ctx.control_flow = Some(ControlFlow::WaitUntil(ctx.wait_next));
                    }
                    let delta = now - ctx.last_update;
                    ctx.last_update = now;

                    update_timer.start();
                    event_handler(Event::Update(delta.as_secs_f32()));
                    ctx.graphics().window().swap_buffers();
                    update_timer.stop();
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

/// Stops the context after the next update.
pub fn request_stop() {
    let ctx = ctx();
    ctx.stop = true;
}

/// Prevents the update event from being sent for at least the duration. If a periodic wait is
/// active, this wait will temporarily override only if it causes the next update event to
/// happen later than the periodic wait would have.
pub fn wait_for(duration: Duration) {
    wait_until(Instant::now() + duration);
}

/// Prevents the update event from being sent until at least the given instant. If a periodic
/// wait is active, this wait will temporarily override only if it causes the next update event
/// to happen later than the periodic wait would have.
pub fn wait_until(instant: Instant) {
    let ctx = ctx();
    if instant > ctx.wait_next {
        ctx.wait_next = instant;
        ctx.control_flow = Some(ControlFlow::WaitUntil(ctx.wait_next));
    }
}

/// Prevents the update event from being sent more frequently than the given duration. Set this
/// to None to disable the periodic wait.
pub fn wait_periodic(duration: Option<Duration>) {
    let ctx = ctx();
    ctx.wait_periodic = duration;
}

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
