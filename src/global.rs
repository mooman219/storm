use crate::asset::AssetState;
use crate::audio::AudioState;
use crate::render::OpenGLState;

#[no_mangle]
static mut CTX: Option<Ctx> = None;

/// The global state context.
pub(crate) struct Ctx {
    graphics: OpenGLState,
    audio: AudioState,
    assets: AssetState,
}

impl Ctx {
    pub fn init(graphics: OpenGLState, audio: AudioState, assets: AssetState) {
        if unsafe { CTX.is_some() } {
            panic!("State already initialized");
        }

        unsafe {
            CTX = Some(Ctx {
                graphics,
                audio,
                assets,
            })
        };
    }

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

/// Retrieves the global state context.
#[inline(always)]
pub(crate) fn ctx() -> &'static mut Ctx {
    unsafe {
        if let Some(ctx) = CTX.as_mut() {
            return ctx;
        }
        panic!("State not initialized")
    }
}
