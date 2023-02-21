use crate::audio::{audio, SoundControl, SoundInstance};
use crate::math::lerp;
use crate::{App, Context};
use alloc::{sync::Arc, vec::Vec};

#[derive(Copy, Clone, Debug)]
/// An error that prevents successful decoding of an audio stream.
pub enum SoundError {
    /// The channel count is unsupported. Only mono and stero sounds are supported.
    UnsupportedChannelCount,
    /// A feature in the audio file isn't supported by the parser.
    UnsupportedFeature,
    /// The audio file not formatted correctly for the encoding.
    InvalidFormat,
}

/// Basic audio container.
#[derive(Clone)]
pub struct Sound {
    sample_rate: f64,
    duration: f64,
    samples: Arc<[[f32; 2]]>,
}

impl Sound {
    /// Attempts to decode FLAC, Ogg Vorbis, WAV, or ALAC into a sound.
    pub fn from_bytes(bytes: &[u8]) -> Result<Sound, SoundError> {
        let mut audio_stream = {
            let file = std::io::Cursor::new(bytes);
            audrey::Reader::new(file).unwrap()
        };
        let description = audio_stream.description();
        let reader = audio_stream.samples::<f32>().map(std::result::Result::unwrap);
        let mut buffer = Vec::with_capacity(description.sample_rate() as usize);
        match description.channel_count() {
            1 => {
                for x in reader {
                    buffer.push([x, x]);
                }
            }
            2 => {
                let mut iter = reader;
                while let Some(x) = iter.next() {
                    let y = iter.next().unwrap() as f32;
                    buffer.push([x, y]);
                }
            }
            _ => return Err(SoundError::UnsupportedChannelCount),
        }
        Sound::new(description.sample_rate(), buffer)
    }

    /// Creates a new sound from a slice of stereo samples.
    pub fn new(sample_rate: u32, samples: Vec<[f32; 2]>) -> Result<Sound, SoundError> {
        let sample_rate = sample_rate as f64;
        Ok(Sound {
            sample_rate,
            duration: samples.len() as f64 / sample_rate,
            samples: samples.into(),
        })
    }

    /// The duration of the sound in seconds.
    pub fn duration(&self) -> f64 {
        self.duration
    }

    /// The sample rate of the sound.
    pub fn sample_rate(&self) -> f64 {
        self.sample_rate
    }

    /// Plays a sound with a given volume.
    /// # Arguments
    ///
    /// * `volume` - A value between `[0, 1]`, where 0 is muted, and 1 is the sound's original volume.
    /// * `smooth` - The duration in seconds to fade the change in volume from the current value to
    /// the given value. Sounds start at a volume of 0.0 when first played to prevent popping.
    /// # Returns
    ///
    /// * `SoundControl` - A handle to control sound properties during play.
    pub fn play(&self, _ctx: &Context<impl App>, volume: f32, smooth: f32) -> SoundControl {
        let control = SoundControl::new(volume, smooth, false);
        let instance = SoundInstance::new(self, &control);
        audio().push_sound(instance);
        control
    }

    pub(crate) fn mix(&self, sample: f64, amplitude: f32, out: &mut [f32; 2]) {
        if sample < 0.0 {
            return;
        }
        let trunc = sample.trunc();
        let whole = trunc as usize;
        if whole + 1 >= self.samples.len() {
            return;
        }
        let t = (sample - trunc) as f32;
        let a = unsafe { self.samples.get_unchecked(whole) };
        let b = unsafe { self.samples.get_unchecked(whole + 1) };
        out[0] += lerp(a[0], b[0], t) * amplitude;
        out[1] += lerp(a[1], b[1], t) * amplitude;
    }
}
