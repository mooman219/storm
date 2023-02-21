use super::NoiseFn;
use core::simd::{LaneCount, Simd, SupportedLaneCount};

/// Noise function that outputs the sum of the two noise function values.
#[derive(Clone, Copy, Debug)]
pub struct Add<const DIM: usize, A, B>
where
    A: NoiseFn<DIM>,
    B: NoiseFn<DIM>,
{
    pub lhs: A,
    pub rhs: B,
}

impl<const DIM: usize, A: NoiseFn<DIM>, B: NoiseFn<DIM>> Add<DIM, A, B> {
    pub fn new(lhs: A, rhs: B) -> Self {
        Self {
            lhs,
            rhs,
        }
    }
}

impl<const DIM: usize, A, B> NoiseFn<DIM> for Add<DIM, A, B>
where
    A: NoiseFn<DIM>,
    B: NoiseFn<DIM>,
{
    #[inline(always)]
    fn sample<const LANES: usize>(&self, x: [Simd<f32, LANES>; DIM]) -> Simd<f32, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        self.lhs.sample(x) + self.rhs.sample(x)
    }
}
