// This file is licensed under multiple licenses, and is a modified version of
// https://github.com/Ralith/clatter/blob/main/src/simplex.rs. A copy of the licenses can be found
// here: in the path licenses/Ralith/clatter.

use super::{hash, NoiseFn, Simplex};
use core::simd::{LaneCount, Simd, SimdFloat, SimdPartialEq, SimdPartialOrd, SupportedLaneCount};
use std::simd::StdFloat;

impl NoiseFn<2> for Simplex {
    #[inline(always)]
    fn sample<const LANES: usize>(&self, [x, y]: [Simd<f32, LANES>; 2]) -> Simd<f32, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        /// (((dimension + 1) as f32).sqrt() - 1.0) / dimension as f32
        const SKEW: f32 = 0.3660254037844386467637;
        /// -((1.0 / ((dimension + 1) as f32).sqrt()) - 1.0) / dimension as f32
        const UNSKEW: f32 = 0.2113248654051871177454;

        // Skew to distort simplexes with side length sqrt(2)/sqrt(3) until they make up
        // squares
        let s = (x + y) * Simd::splat(SKEW);
        let ips = (x + s).floor();
        let jps = (y + s).floor();

        // Integer coordinates for the base vertex of the triangle
        let i = ips.cast::<i32>();
        let j = jps.cast::<i32>();

        let t = (i + j).cast::<f32>() * Simd::splat(UNSKEW);

        // Unskewed distances to the first point of the enclosing simplex
        let x0 = x - (ips - t);
        let y0 = y - (jps - t);

        let i1 = x0.simd_ge(y0).to_int();
        let j1 = y0.simd_gt(x0).to_int();

        // Distances to the second and third points of the enclosing simplex
        let x1 = x0 + i1.cast() + Simd::splat(UNSKEW);
        let y1 = y0 + j1.cast() + Simd::splat(UNSKEW);
        let x2 = x0 + Simd::splat(-1.0) + Simd::splat(2.0 * UNSKEW);
        let y2 = y0 + Simd::splat(-1.0) + Simd::splat(2.0 * UNSKEW);

        let gi0 = hash::pcg_3d([i, j, Simd::splat(self.seed)])[0];
        let gi1 = hash::pcg_3d([i - i1, j - j1, Simd::splat(self.seed)])[0];
        let gi2 = hash::pcg_3d([i + Simd::splat(1), j + Simd::splat(1), Simd::splat(self.seed)])[0];

        // Weights associated with the gradients at each corner
        // These FMA operations are equivalent to: let t = max(0, 0.5 - x*x - y*y)
        let t0 = y0.mul_add(-y0, x0.mul_add(-x0, Simd::splat(0.5))).simd_max(Simd::splat(0.0));
        let t1 = y1.mul_add(-y1, x1.mul_add(-x1, Simd::splat(0.5))).simd_max(Simd::splat(0.0));
        let t2 = y2.mul_add(-y2, x2.mul_add(-x2, Simd::splat(0.5))).simd_max(Simd::splat(0.0));

        let t20 = t0 * t0;
        let t40 = t20 * t20;
        let t21 = t1 * t1;
        let t41 = t21 * t21;
        let t22 = t2 * t2;
        let t42 = t22 * t22;

        let [gx0, gy0] = gradient_2d(gi0);
        let g0 = gx0 * x0 + gy0 * y0;
        let n0 = t40 * g0;
        let [gx1, gy1] = gradient_2d(gi1);
        let g1 = gx1 * x1 + gy1 * y1;
        let n1 = t41 * g1;
        let [gx2, gy2] = gradient_2d(gi2);
        let g2 = gx2 * x2 + gy2 * y2;
        let n2 = t42 * g2;

        // Scaling factor found by numerical optimization
        const SCALE: f32 = 45.26450774985561631259;
        (n0 + n1 + n2) * Simd::splat(SCALE)
    }
}

#[inline(always)]
fn gradient_2d<const LANES: usize>(hash: Simd<i32, LANES>) -> [Simd<f32, LANES>; 2]
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let h = hash & Simd::splat(7);

    let mask = h.simd_lt(Simd::splat(4));
    let x_magnitude = mask.select(Simd::splat(1.0), Simd::splat(2.0));
    let y_magnitude = mask.select(Simd::splat(2.0), Simd::splat(1.0));

    let h_and_1 = (h & Simd::splat(1)).simd_eq(Simd::splat(0));
    let h_and_2 = (h & Simd::splat(2)).simd_eq(Simd::splat(0));

    let gx = mask.select_mask(h_and_1, h_and_2).select(x_magnitude, -x_magnitude);
    let gy = mask.select_mask(h_and_2, h_and_1).select(y_magnitude, -y_magnitude);
    [gx, gy]
}
