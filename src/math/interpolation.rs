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

    pub fn start(&self) -> f32 {
        self.start
    }

    pub fn end(&self) -> f32 {
        self.end
    }

    pub fn progress(&self) -> f32 {
        self.progress
    }

    pub fn get(&self) -> f32 {
        lerp(self.start, self.end, self.progress)
    }

    pub fn set(&mut self, start: f32, end: f32) {
        self.start = start;
        self.end = end;
        self.progress = 0.0;
    }

    pub fn update(&mut self, end: f32) {
        self.start = self.get();
        self.end = end;
        self.progress = 0.0;
    }

    pub fn advance(&mut self, progress: f32) {
        self.progress += progress;
        if self.progress > 1.0 {
            self.progress = 1.0;
        }
    }
}
