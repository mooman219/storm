use super::sound::{Sound, SoundError};
use claxon::{Error as ClaxonError, FlacReader};

/// Interpret a slice of bytes as a FLAC file and decodes it into a sound.
pub fn read_flac(bytes: &[u8]) -> Result<Sound, SoundError> {
    let mut reader = FlacReader::new(bytes).map_err(map)?;
    let mut buffer = if let Some(samples) = reader.streaminfo().samples {
        Vec::with_capacity(samples as usize)
    } else {
        Vec::new()
    };
    let scale = (1 << reader.streaminfo().bits_per_sample) / 2;
    let scale = 1.0 / scale as f32;
    for sample in reader.samples() {
        buffer.push(sample.unwrap() as f32 * scale);
    }
    Sound::new(reader.streaminfo().channels, reader.streaminfo().sample_rate, buffer)
}

fn map(error: ClaxonError) -> SoundError {
    match error {
        ClaxonError::IoError(_) => SoundError::InvalidFormat,
        ClaxonError::FormatError(_) => SoundError::InvalidFormat,
        ClaxonError::Unsupported(_) => SoundError::UnsupportedFeature,
    }
}
