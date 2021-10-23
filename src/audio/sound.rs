use alloc::sync::Arc;
use oddio::Frames;

pub(crate) enum Channels {
    Mono(Arc<Frames<[f32; 1]>>),
    Stero(Arc<Frames<[f32; 2]>>),
}

/// Basic audio container.
pub struct Sound {
    frames: Channels,
}

impl Sound {
    pub(crate) fn new(channels: u32, sample_rate: u32, samples: Vec<f32>) -> Result<Sound, SoundError> {
        match channels {
            1 => {
                let samples = unsafe { core::mem::transmute::<Vec<f32>, Vec<[f32; 1]>>(samples) };
                Ok(Sound {
                    frames: Channels::Mono(Frames::from_slice(sample_rate, &samples)),
                })
            }
            2 => {
                if samples.len() % 2 != 0 {
                    return Err(SoundError::InvalidSampleSize);
                }
                let samples = unsafe {
                    let mut samples = core::mem::transmute::<Vec<f32>, Vec<[f32; 2]>>(samples);
                    samples.set_len(samples.len() >> 1);
                    samples
                };
                Ok(Sound {
                    frames: Channels::Stero(Frames::from_slice(sample_rate, &samples)),
                })
            }
            _ => Err(SoundError::UnsupportedChannelCount),
        }
    }

    pub fn play(&self) {}
}

#[derive(Debug)]
/// An error that prevents successful decoding of an audio stream.
pub enum SoundError {
    /// The channel count is unsupported. Only mono and stero sounds are supported.
    UnsupportedChannelCount,
    /// A feature in the audio file isn't supported by the parser.
    UnsupportedFeature,
    /// The audio file not formatted correctly for the encoding.
    InvalidFormat,
    /// The number of samples in the audio file do not divide by the number of channels in the
    /// stream.
    InvalidSampleSize,
}
