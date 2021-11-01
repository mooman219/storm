/// Configuration settings for sound.
pub struct SoundSettings {
    /// A value between [0, inf).
    pub volume: f32,
    /// The duration over which gain fades from 0 to the target volume in seconds.
    pub smooth_duration: f32,
}

impl Default for SoundSettings {
    fn default() -> Self {
        Self {
            volume: 0.0,
            smooth_duration: 0.1,
        }
    }
}
