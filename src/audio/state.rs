use crate::audio::{Mixer, SoundInstance};
use crate::sync::{make as spsc_make, Producer};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream,
};

#[no_mangle]
static mut _STORM_AUDIO_INITIALIZED: AtomicBool = AtomicBool::new(false);
#[no_mangle]
static mut _STORM_AUDIO: MaybeUninit<AudioState> = MaybeUninit::<AudioState>::uninit();

pub(crate) fn audio() -> &'static mut AudioState {
    unsafe { _STORM_AUDIO.assume_init_mut() }
}

pub(crate) struct AudioState {
    sender: Producer<SoundInstance>,
    _stream: Stream,
}

impl AudioState {
    pub(crate) fn init() {
        if unsafe { _STORM_AUDIO_INITIALIZED.swap(true, Ordering::Relaxed) } {
            panic!("Audio has already initialized.");
        }

        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device available");
        let sample_rate = device.default_output_config().unwrap().sample_rate();
        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate,
            buffer_size: cpal::BufferSize::Default,
        };
        let (sender, receiver) = spsc_make(256);
        let mut mixer = Mixer::new(sample_rate.0, receiver);

        let stream = device
            .build_output_stream(
                &config,
                move |out_flat: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    mixer.sample(as_stereo(out_flat));
                },
                move |err| {
                    log::error!("{}", err);
                },
                None,
            )
            .unwrap();
        stream.play().unwrap();

        unsafe {
            _STORM_AUDIO.write(AudioState {
                sender,
                _stream: stream,
            })
        };
    }

    pub(crate) fn push_sound(&mut self, instance: SoundInstance) {
        self.sender.push(instance);
    }
}

fn as_stereo(xs: &mut [f32]) -> &mut [[f32; 2]] {
    unsafe { core::slice::from_raw_parts_mut(xs.as_mut_ptr() as _, xs.len() / 2) }
}
