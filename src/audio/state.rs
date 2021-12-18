use crate::audio::{
    spsc::{self, Producer},
    Mixer, SoundInstance,
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream,
};

#[no_mangle]
static mut AUDIO: Option<AudioState> = None;

pub struct AudioState {
    sender: Producer<SoundInstance>,
    stream: Stream,
}

impl AudioState {
    pub fn init() {
        if unsafe { AUDIO.is_some() } {
            panic!("State already initialized");
        }

        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device available");
        let sample_rate = device.default_output_config().unwrap().sample_rate();
        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate,
            buffer_size: cpal::BufferSize::Default,
        };
        let (sender, receiver) = spsc::make(256);
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
            )
            .unwrap();
        stream.play().unwrap();

        let state = AudioState {
            sender,
            stream,
        };
        unsafe { AUDIO = Some(state) };
    }

    pub fn send(&mut self, instance: SoundInstance) {
        self.sender.push(instance);
    }

    pub fn ctx() -> &'static mut AudioState {
        unsafe {
            if let Some(ctx) = AUDIO.as_mut() {
                return ctx;
            }
            panic!("State not initialized")
        }
    }
}

fn as_stereo(xs: &mut [f32]) -> &mut [[f32; 2]] {
    unsafe { std::slice::from_raw_parts_mut(xs.as_mut_ptr() as _, xs.len() / 2) }
}
