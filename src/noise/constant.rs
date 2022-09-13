use super::NoiseFn;
use core::simd::{LaneCount, Simd, SupportedLaneCount};

/// Noise function that outputs a constant value.
///
/// This function takes a input, value, and returns that input for all points,
/// producing a constant-valued field.
///
/// This function is not very useful by itself, but can be used as a source
/// function for other noise functions.
#[derive(Clone, Copy, Debug)]
pub struct Constant {
    pub value: f32,
}

impl Constant {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl Default for Constant {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl<const DIM: usize> NoiseFn<DIM> for Constant {
    fn sample<const LANES: usize>(&self, _: [Simd<f32, LANES>; DIM]) -> Simd<f32, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        Simd::splat(self.value)
    }
}
