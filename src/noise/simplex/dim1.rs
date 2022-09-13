// This file is licensed under multiple licenses, and is a modified version of
// https://github.com/Ralith/clatter/blob/main/src/simplex.rs. A copy of the licenses can be found
// here: in the path licenses/Ralith/clatter.

use super::{hash, NoiseFn, Simplex};
use core::simd::{LaneCount, Simd, SimdPartialEq, SupportedLaneCount};
use std::simd::StdFloat;

impl NoiseFn<1> for Simplex {
    #[inline(always)]
    fn sample<const LANES: usize>(&self, [x]: [Simd<f32, LANES>; 1]) -> Simd<f32, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        // Gradients are selected deterministically based on the whole part of `x`
        let i = x.floor();
        let i0 = i.cast::<i32>();
        let i1 = i0 + Simd::splat(1);

        // the fractional part of x, i.e. the distance to the left gradient node. 0 ≤ x0 < 1.
        let x0 = x - i;
        // signed distance to the right gradient node
        let x1 = x0 - Simd::splat(1.0);

        // Select gradients
        let gi0 = hash::pcg(Simd::splat(self.seed) ^ i0.cast());
        let gi1 = hash::pcg(Simd::splat(self.seed) ^ i1.cast());

        // Compute the contribution from the first gradient
        // n0 = grad0 * (1 - x0^2)^4 * x0
        let x20 = x0 * x0;
        let t0 = Simd::<f32, LANES>::splat(1.0) - x20;
        let t20 = t0 * t0;
        let t40 = t20 * t20;
        let gx0 = gradient_1d::<LANES>(gi0);
        let n0 = t40 * gx0 * x0;

        // Compute the contribution from the second gradient
        // n1 = grad1 * (x0 - 1) * (1 - (x0 - 1)^2)^4
        let x21 = x1 * x1;
        let t1 = Simd::<f32, LANES>::splat(1.0) - x21;
        let t21 = t1 * t1;
        let t41 = t21 * t21;
        let gx1 = gradient_1d::<LANES>(gi1);
        let n1 = t41 * gx1 * x1;

        // n0 + n1 =
        //    grad0 * x0 * (1 - x0^2)^4
        //  + grad1 * (x0 - 1) * (1 - (x0 - 1)^2)^4
        //
        // Assuming worst-case values for grad0 and grad1, we therefore need only determine the maximum of
        //
        // |x0 * (1 - x0^2)^4| + |(x0 - 1) * (1 - (x0 - 1)^2)^4|
        //
        // for 0 ≤ x0 < 1. This can be done by root-finding on the derivative, obtaining 81 / 256 when
        // x0 = 0.5, which we finally multiply by the maximum gradient to get the maximum value,
        // allowing us to scale into [-1, 1]
        const SCALE: f32 = 256.0 / (81.0 * 8.0);
        (n0 + n1) * Simd::splat(SCALE)
    }
}

/// Generates a random integer gradient in ±7 inclusive
///
/// This differs from Gustavson's well-known implementation in that gradients can be zero, and the
/// maximum gradient is 7 rather than 8.
#[inline(always)]
fn gradient_1d<const LANES: usize>(hash: Simd<i32, LANES>) -> Simd<f32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let h = hash & Simd::splat(0xF);
    let v = (Simd::splat(1) + (h & Simd::splat(7))).cast::<f32>();

    let h_and_8 = (h & Simd::splat(8)).simd_eq(Simd::splat(0));
    h_and_8.select(v, Simd::splat(0.0) - v)
}
