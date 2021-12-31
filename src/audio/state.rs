use crate::audio::{Mixer, SoundInstance};
use crate::sync::{make as spsc_make, Producer};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream,
};

pub(crate) struct AudioState {
    sender: Producer<SoundInstance>,
    stream: Stream,
}

impl AudioState {
    pub(crate) fn init() -> AudioState {
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
            )
            .unwrap();
        stream.play().unwrap();

        AudioState {
            sender,
            stream,
        }
    }

    pub(crate) fn push_sound(&mut self, instance: SoundInstance) {
        self.sender.push(instance);
    }
}

fn as_stereo(xs: &mut [f32]) -> &mut [[f32; 2]] {
    unsafe { core::slice::from_raw_parts_mut(xs.as_mut_ptr() as _, xs.len() / 2) }
}
