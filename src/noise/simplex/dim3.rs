// This file is licensed under multiple licenses, and is a modified version of
// https://github.com/Ralith/clatter/blob/main/src/simplex.rs. A copy of the licenses can be found
// here: in the path licenses/Ralith/clatter.

use super::{hash, NoiseFn, Simplex};
use core::simd::{LaneCount, Mask, Simd, SimdFloat, SimdPartialEq, SimdPartialOrd, SupportedLaneCount};
use std::simd::StdFloat;

impl NoiseFn<3> for Simplex {
    #[inline(always)]
    fn sample<const LANES: usize>(&self, [x, y, z]: [Simd<f32, LANES>; 3]) -> Simd<f32, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        /// (((dimension + 1) as f32).sqrt() - 1.0) / dimension as f32
        const SKEW: f32 = 0.3333333333333333333333;
        /// ((1.0 / ((dimension + 1) as f32).sqrt()) - 1.0) / dimension as f32
        const UNSKEW: f32 = 0.1666666666666666666667;

        // Find skewed simplex grid coordinates associated with the input coordinates
        let f = (x + y + z) * Simd::splat(SKEW);
        let x0 = (x + f).floor();
        let y0 = (y + f).floor();
        let z0 = (z + f).floor();

        // Integer grid coordinates
        let i = x0.cast::<i32>();
        let j = y0.cast::<i32>();
        let k = z0.cast::<i32>();

        let g = Simd::splat(UNSKEW) * (x0 + y0 + z0);
        let x0 = x - (x0 - g);
        let y0 = y - (y0 - g);
        let z0 = z - (z0 - g);

        let x0_ge_y0 = x0.simd_ge(y0);
        let y0_ge_z0 = y0.simd_ge(z0);
        let x0_ge_z0 = x0.simd_ge(z0);

        let i1 = x0_ge_y0 & x0_ge_z0;
        let j1 = !x0_ge_y0 & y0_ge_z0;
        let k1 = !x0_ge_z0 & !y0_ge_z0;

        let i2 = x0_ge_y0 | x0_ge_z0;
        let j2 = !x0_ge_y0 | y0_ge_z0;
        let k2 = !(x0_ge_z0 & y0_ge_z0);

        let v1x = i + i1.select(Simd::splat(1), Simd::splat(0));
        let v1y = j + j1.select(Simd::splat(1), Simd::splat(0));
        let v1z = k + k1.select(Simd::splat(1), Simd::splat(0));

        let v2x = i + i2.select(Simd::splat(1), Simd::splat(0));
        let v2y = j + j2.select(Simd::splat(1), Simd::splat(0));
        let v2z = k + k2.select(Simd::splat(1), Simd::splat(0));

        let v3x = i + Simd::splat(1);
        let v3y = j + Simd::splat(1);
        let v3z = k + Simd::splat(1);

        let x1 = x0 - i1.select(Simd::splat(1.0), Simd::splat(0.0)) + Simd::splat(UNSKEW);
        let y1 = y0 - j1.select(Simd::splat(1.0), Simd::splat(0.0)) + Simd::splat(UNSKEW);
        let z1 = z0 - k1.select(Simd::splat(1.0), Simd::splat(0.0)) + Simd::splat(UNSKEW);

        let x2 = x0 - i2.select(Simd::splat(1.0), Simd::splat(0.0)) + Simd::splat(SKEW);
        let y2 = y0 - j2.select(Simd::splat(1.0), Simd::splat(0.0)) + Simd::splat(SKEW);
        let z2 = z0 - k2.select(Simd::splat(1.0), Simd::splat(0.0)) + Simd::splat(SKEW);

        let x3 = x0 + Simd::splat(-0.5);
        let y3 = y0 + Simd::splat(-0.5);
        let z3 = z0 + Simd::splat(-0.5);

        // Compute base weight factors associated with each vertex, `0.5 - v . v` where v is the
        // difference between the sample point and the vertex.
        let t0 = (Simd::splat(0.5) - x0 * x0 - y0 * y0 - z0 * z0).simd_max(Simd::splat(0.0));
        let t1 = (Simd::splat(0.5) - x1 * x1 - y1 * y1 - z1 * z1).simd_max(Simd::splat(0.0));
        let t2 = (Simd::splat(0.5) - x2 * x2 - y2 * y2 - z2 * z2).simd_max(Simd::splat(0.0));
        let t3 = (Simd::splat(0.5) - x3 * x3 - y3 * y3 - z3 * z3).simd_max(Simd::splat(0.0));

        // Square weights
        let t20 = t0 * t0;
        let t21 = t1 * t1;
        let t22 = t2 * t2;
        let t23 = t3 * t3;

        // ...twice!
        let t40 = t20 * t20;
        let t41 = t21 * t21;
        let t42 = t22 * t22;
        let t43 = t23 * t23;

        // Compute contribution from each vertex
        let g0 = Gradient3d::new(self.seed, [i, j, k]);
        let g0d = g0.dot([x0, y0, z0]);
        let v0 = t40 * g0d;

        let g1 = Gradient3d::new(self.seed, [v1x, v1y, v1z]);
        let g1d = g1.dot([x1, y1, z1]);
        let v1 = t41 * g1d;

        let g2 = Gradient3d::new(self.seed, [v2x, v2y, v2z]);
        let g2d = g2.dot([x2, y2, z2]);
        let v2 = t42 * g2d;

        let g3 = Gradient3d::new(self.seed, [v3x, v3y, v3z]);
        let g3d = g3.dot([x3, y3, z3]);
        let v3 = t43 * g3d;

        // Scaling factor found by numerical optimization
        const SCALE: f32 = 67.79816627147162;
        (v3 + v2 + v1 + v0) * Simd::splat(SCALE)
    }
}

/// Generates a random gradient vector from the origin towards the midpoint of an edge of a
/// double-unit cube
struct Gradient3d<const LANES: usize>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    // Masks guiding dimension selection
    l8: Mask<i32, LANES>,
    l4: Mask<i32, LANES>,
    h12_or_14: Mask<i32, LANES>,

    // Signs for the selected dimensions
    h1: Simd<i32, LANES>,
    h2: Simd<i32, LANES>,
}

impl<const LANES: usize> Gradient3d<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Compute hash values used by `grad3d` and `grad3d_dot`
    #[inline(always)]
    fn new(seed: i32, [i, j, k]: [Simd<i32, LANES>; 3]) -> Self {
        let hash = hash::pcg_4d([i, j, k, Simd::splat(seed)])[0];
        let hasha13 = hash & Simd::splat(13);
        Self {
            l8: hasha13.simd_lt(Simd::splat(8)),
            l4: hasha13.simd_lt(Simd::splat(2)),
            h12_or_14: hasha13.simd_eq(Simd::splat(12)),

            h1: hash << Simd::splat(31),
            h2: (hash & Simd::splat(2)) << Simd::splat(30),
        }
    }

    /// Computes the dot product of a vector with the gradient vector
    #[inline(always)]
    fn dot(&self, [x, y, z]: [Simd<f32, LANES>; 3]) -> Simd<f32, LANES> {
        let u = self.l8.select(x, y);
        let v = self.l4.select(y, self.h12_or_14.select(x, z));
        // Maybe flip sign bits, then sum
        Simd::<f32, LANES>::from_bits(u.to_bits() ^ self.h1.cast())
            + Simd::<f32, LANES>::from_bits(v.to_bits() ^ self.h2.cast())
    }

    /// The gradient vector generated by `dot`
    ///
    /// This is a separate function because it's slower than `grad3d_dot` and only needed when computing
    /// derivatives.
    #[inline(always)]
    fn vector(&self) -> [Simd<f32, LANES>; 3] {
        let first = Simd::<f32, LANES>::from_bits(self.h1.cast() | Simd::<f32, LANES>::splat(1.0).to_bits());
        let gx = self.l8.select(first, Simd::splat(0.0));
        let gy = (!self.l8).select(first, Simd::splat(0.0));

        let second = Simd::<f32, LANES>::from_bits(self.h2.cast() | Simd::<f32, LANES>::splat(1.0).to_bits());
        let gy = self.l4.select(second, gy);
        let gx = (!self.l4 & self.h12_or_14).select(second, gx);
        let gz = (!(self.h12_or_14 | self.l4)).select(second, Simd::splat(0.0));

        [gx, gy, gz]
    }
}
