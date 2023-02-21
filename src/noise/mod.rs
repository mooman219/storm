mod add;
mod constant;
mod fbm;
mod fill;
mod multiply;
mod simplex;

pub use self::add::Add;
pub use self::constant::Constant;
pub use self::multiply::Multiply;
pub use self::simplex::Simplex;
pub use fill::FillFn;

use core::simd::{LaneCount, Simd, SupportedLaneCount};

/// Base trait for noise functions.
///
/// A noise function is a struct that calculates and outputs a value given a n-dimensional input
/// value.
pub trait NoiseFn<const DIM: usize>: Sized {
    fn sample<const LANES: usize>(&self, x: [Simd<f32, LANES>; DIM]) -> Simd<f32, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount;

    fn add<T>(self, other: T) -> Add<DIM, Self, T>
    where
        T: NoiseFn<DIM>,
    {
        Add::new(self, other)
    }

    fn multiply<T>(self, other: T) -> Multiply<DIM, Self, T>
    where
        T: NoiseFn<DIM>,
    {
        Multiply::new(self, other)
    }
}
