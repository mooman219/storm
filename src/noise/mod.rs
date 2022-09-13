// mod fbm;
mod constant;
mod fill;
mod simplex;

#[doc(inline)]
pub use self::constant::Constant;

pub use self::simplex::Simplex;
pub use fill::FillFn;

use core::simd::{LaneCount, Simd, SupportedLaneCount};

/// Base trait for noise functions.
///
/// A noise function is a struct that calculates and outputs a value given a n-dimensional input
/// value.
pub trait NoiseFn<const DIM: usize> {
    fn sample<const LANES: usize>(&self, x: [Simd<f32, LANES>; DIM]) -> Simd<f32, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount;
}
