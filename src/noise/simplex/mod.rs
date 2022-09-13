mod dim1;
mod dim2;
mod dim3;
mod dim4;

mod hash;

use super::NoiseFn;

/// Noise function that outputs 1/2/3/4-dimensional Simplex noise.
#[derive(Clone, Copy, Debug)]
pub struct Simplex {
    seed: i32,
}

impl Simplex {
    pub fn seed(seed: i32) -> Self {
        Self { seed }
    }
}

impl Default for Simplex {
    fn default() -> Self {
        Self { seed: 0 }
    }
}
