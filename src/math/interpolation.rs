use crate::math::lerp;

pub struct Interpolation {
    /// Initial value.
    start: f32,
    /// Target value.
    end: f32,
    /// Value progress from 0 to 1 between the `start` and `end`.
    progress: f32,
}

impl Interpolation {
    pub fn new(start: f32, end: f32) -> Interpolation {
        Interpolation {
            start,
            end,
            progress: 0.0,
        }
    }

    /// Gets the set start value.
    pub fn start(&self) -> f32 {
        self.start
    }

    /// Gets the set end value.
    pub fn end(&self) -> f32 {
        self.end
    }

    /// Gets the current progress.
    pub fn progress(&self) -> f32 {
        self.progress
    }

    /// Computes the current value of the interpolation.
    pub fn get(&self) -> f32 {
        lerp(self.start, self.end, self.progress)
    }

    /// Sets the start and end of the interpolation, restarting progress.
    pub fn set(&mut self, start: f32, end: f32) {
        self.start = start;
        self.end = end;
        self.progress = 0.0;
    }

    /// Updates the end of the interpolation, continuing from the current value, and restarting
    /// progress.
    pub fn update(&mut self, end: f32) {
        self.start = self.get();
        self.end = end;
        self.progress = 0.0;
    }

    /// Adds the given delta to the progress of the interpolation. Progress is tracked as a value
    /// between [0, 1]
    pub fn advance(&mut self, progress: f32) {
        self.progress += progress;
        if self.progress > 1.0 {
            self.progress = 1.0;
        }
    }
}
