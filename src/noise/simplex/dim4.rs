// This file is licensed under multiple licenses, and is a modified version of
// https://github.com/Ralith/clatter/blob/main/src/simplex.rs. A copy of the licenses can be found
// here: in the path licenses/Ralith/clatter.

use super::{hash, NoiseFn, Simplex};
use core::simd::{LaneCount, Mask, Simd, SimdFloat, SimdPartialEq, SimdPartialOrd, SupportedLaneCount};
use std::simd::StdFloat;

impl NoiseFn<4> for Simplex {
    #[inline(always)]
    fn sample<const LANES: usize>(&self, [x, y, z, w]: [Simd<f32, LANES>; 4]) -> Simd<f32, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        /// (((dimension + 1) as f32).sqrt() - 1.0) / dimension as f32
        const SKEW: f32 = 0.3090169943749474241023;
        /// ((1.0 / ((dimension + 1) as f32).sqrt()) - 1.0) / dimension as f32
        const UNSKEW: f32 = 0.1381966011250105151795;

        // Determine which simplex these points lie in, and compute the distance along each axis to each
        // vertex of the simplex

        let s = Simd::splat(SKEW) * (x + y + z + w);

        let ips = (x + s).floor();
        let jps = (y + s).floor();
        let kps = (z + s).floor();
        let lps = (w + s).floor();

        let i = ips.cast::<i32>();
        let j = jps.cast::<i32>();
        let k = kps.cast::<i32>();
        let l = lps.cast::<i32>();

        let t = Simd::splat(UNSKEW) * (i + j + k + l).cast();
        let x0 = x - (ips - t);
        let y0 = y - (jps - t);
        let z0 = z - (kps - t);
        let w0 = w - (lps - t);

        let mut rank_x = Simd::splat(0);
        let mut rank_y = Simd::splat(0);
        let mut rank_z = Simd::splat(0);
        let mut rank_w = Simd::splat(0);

        let cond = x0.simd_gt(y0);
        rank_x += cond.select(Simd::splat(1), Simd::splat(0));
        rank_y += cond.select(Simd::splat(0), Simd::splat(1));
        let cond = x0.simd_gt(z0);
        rank_x += cond.select(Simd::splat(1), Simd::splat(0));
        rank_z += cond.select(Simd::splat(0), Simd::splat(1));
        let cond = x0.simd_gt(w0);
        rank_x += cond.select(Simd::splat(1), Simd::splat(0));
        rank_w += cond.select(Simd::splat(0), Simd::splat(1));
        let cond = y0.simd_gt(z0);
        rank_y += cond.select(Simd::splat(1), Simd::splat(0));
        rank_z += cond.select(Simd::splat(0), Simd::splat(1));
        let cond = y0.simd_gt(w0);
        rank_y += cond.select(Simd::splat(1), Simd::splat(0));
        rank_w += cond.select(Simd::splat(0), Simd::splat(1));
        let cond = z0.simd_gt(w0);
        rank_z += cond.select(Simd::splat(1), Simd::splat(0));
        rank_w += cond.select(Simd::splat(0), Simd::splat(1));

        let i1 = Mask::<i32, LANES>::to_int(rank_x.simd_gt(Simd::splat(2)));
        let j1 = Mask::<i32, LANES>::to_int(rank_y.simd_gt(Simd::splat(2)));
        let k1 = Mask::<i32, LANES>::to_int(rank_z.simd_gt(Simd::splat(2)));
        let l1 = Mask::<i32, LANES>::to_int(rank_w.simd_gt(Simd::splat(2)));

        let i2 = Mask::<i32, LANES>::to_int(rank_x.simd_gt(Simd::splat(1)));
        let j2 = Mask::<i32, LANES>::to_int(rank_y.simd_gt(Simd::splat(1)));
        let k2 = Mask::<i32, LANES>::to_int(rank_z.simd_gt(Simd::splat(1)));
        let l2 = Mask::<i32, LANES>::to_int(rank_w.simd_gt(Simd::splat(1)));

        let i3 = Mask::<i32, LANES>::to_int(rank_x.simd_gt(Simd::splat(0)));
        let j3 = Mask::<i32, LANES>::to_int(rank_y.simd_gt(Simd::splat(0)));
        let k3 = Mask::<i32, LANES>::to_int(rank_z.simd_gt(Simd::splat(0)));
        let l3 = Mask::<i32, LANES>::to_int(rank_w.simd_gt(Simd::splat(0)));

        let x1 = x0 + i1.cast() + Simd::splat(UNSKEW);
        let y1 = y0 + j1.cast() + Simd::splat(UNSKEW);
        let z1 = z0 + k1.cast() + Simd::splat(UNSKEW);
        let w1 = w0 + l1.cast() + Simd::splat(UNSKEW);
        let x2 = x0 + i2.cast() + Simd::splat(2.0 * UNSKEW);
        let y2 = y0 + j2.cast() + Simd::splat(2.0 * UNSKEW);
        let z2 = z0 + k2.cast() + Simd::splat(2.0 * UNSKEW);
        let w2 = w0 + l2.cast() + Simd::splat(2.0 * UNSKEW);
        let x3 = x0 + i3.cast() + Simd::splat(3.0 * UNSKEW);
        let y3 = y0 + j3.cast() + Simd::splat(3.0 * UNSKEW);
        let z3 = z0 + k3.cast() + Simd::splat(3.0 * UNSKEW);
        let w3 = w0 + l3.cast() + Simd::splat(3.0 * UNSKEW);
        let x4 = (x0 - Simd::splat(1.0)) + Simd::splat(4.0 * UNSKEW);
        let y4 = (y0 - Simd::splat(1.0)) + Simd::splat(4.0 * UNSKEW);
        let z4 = (z0 - Simd::splat(1.0)) + Simd::splat(4.0 * UNSKEW);
        let w4 = (w0 - Simd::splat(1.0)) + Simd::splat(4.0 * UNSKEW);

        //
        // Hash the integer coordinates
        //

        let gi0 = Gradient4d::new(self.seed, [i, j, k, l]);
        let gi1 = Gradient4d::new(self.seed, [i - i1, j - j1, k - k1, l - l1]);
        let gi2 = Gradient4d::new(self.seed, [i - i2, j - j2, k - k2, l - l2]);
        let gi3 = Gradient4d::new(self.seed, [i - i3, j - j3, k - k3, l - l3]);
        let gi4 = Gradient4d::new(self.seed, [i, j, k, l].map(|x| x + Simd::splat(1)));

        //
        // Compute base weight factors associated with each vertex
        //

        // These FMA operations are equivalent to: let t = max(0, 0.5 - x*x - y*y - z*z - w*w)
        let t0 = w0
            .mul_add(-w0, z0.mul_add(-z0, y0.mul_add(-y0, x0.mul_add(-x0, Simd::splat(0.5)))))
            .simd_max(Simd::splat(0.0));
        let t1 = w1
            .mul_add(-w1, z1.mul_add(-z1, y1.mul_add(-y1, x1.mul_add(-x1, Simd::splat(0.5)))))
            .simd_max(Simd::splat(0.0));
        let t2 = w2
            .mul_add(-w2, z2.mul_add(-z2, y2.mul_add(-y2, x2.mul_add(-x2, Simd::splat(0.5)))))
            .simd_max(Simd::splat(0.0));
        let t3 = w3
            .mul_add(-w3, z3.mul_add(-z3, y3.mul_add(-y3, x3.mul_add(-x3, Simd::splat(0.5)))))
            .simd_max(Simd::splat(0.0));
        let t4 = w4
            .mul_add(-w4, z4.mul_add(-z4, y4.mul_add(-y4, x4.mul_add(-x4, Simd::splat(0.5)))))
            .simd_max(Simd::splat(0.0));

        // Cube each weight
        let t02 = t0 * t0;
        let t04 = t02 * t02;
        let t12 = t1 * t1;
        let t14 = t12 * t12;
        let t22 = t2 * t2;
        let t24 = t22 * t22;
        let t32 = t3 * t3;
        let t34 = t32 * t32;
        let t42 = t4 * t4;
        let t44 = t42 * t42;

        // Compute contributions from each gradient
        let g0d = gi0.dot([x0, y0, z0, w0]);
        let g1d = gi1.dot([x1, y1, z1, w1]);
        let g2d = gi2.dot([x2, y2, z2, w2]);
        let g3d = gi3.dot([x3, y3, z3, w3]);
        let g4d = gi4.dot([x4, y4, z4, w4]);

        let n0 = t04 * g0d;
        let n1 = t14 * g1d;
        let n2 = t24 * g2d;
        let n3 = t34 * g3d;
        let n4 = t44 * g4d;

        // Scaling factor found by numerical optimization
        const SCALE: f32 = 62.77772078955791;
        (n0 + n1 + n2 + n3 + n4) * Simd::splat(SCALE)
    }
}

/// Uniformly maps i32s to vectors from the origin towards the midpoint of an edge of a hypercube
struct Gradient4d<const LANES: usize>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    // Masks guiding dimension selection
    l24: Mask<i32, LANES>,
    l16: Mask<i32, LANES>,
    l8: Mask<i32, LANES>,

    // Signs for the selected dimensions
    sign1: Mask<i32, LANES>,
    sign2: Mask<i32, LANES>,
    sign3: Mask<i32, LANES>,
}

impl<const LANES: usize> Gradient4d<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline(always)]
    fn new(seed: i32, [i, j, k, l]: [Simd<i32, LANES>; 4]) -> Self {
        let hash = hash::pcg_4d([
            i ^ Simd::splat(seed),
            j ^ Simd::splat(seed),
            k ^ Simd::splat(seed),
            l ^ Simd::splat(seed),
        ])[0];

        let h = hash & Simd::splat(31);
        Self {
            l24: Simd::splat(24).simd_gt(h),
            l16: Simd::splat(16).simd_gt(h),
            l8: Simd::splat(8).simd_gt(h),

            sign1: Simd::splat(0).simd_eq(h & Simd::splat(1)),
            sign2: Simd::splat(0).simd_eq(h & Simd::splat(2)),
            sign3: Simd::splat(0).simd_eq(h & Simd::splat(4)),
        }
    }

    /// Directly compute the dot product of the gradient vector with a vector
    #[inline(always)]
    fn dot(&self, [x, y, z, t]: [Simd<f32, LANES>; 4]) -> Simd<f32, LANES> {
        let u = self.l24.select(x, y);
        let v = self.l16.select(y, z);
        let w = self.l8.select(z, t);
        self.sign1.select(u, -u) + self.sign2.select(v, -v) + self.sign3.select(w, -w)
    }

    /// Compute the actual gradient vector
    ///
    /// Slower than `dot` and only needed to compute derivatives
    #[inline(always)]
    fn vector(&self) -> [Simd<f32, LANES>; 4] {
        // Select axes
        //       h: u  v  w
        // 24..=31: y, z, t
        // 17..=23: x, z, t
        //  8..=16: x, y, t
        //  0..=7 : x, y, z
        let gx = self.l24.select(Simd::splat(1), Simd::splat(0));
        let gy = (self.l16 | !self.l24).select(Simd::splat(1), Simd::splat(0));
        let gz = (self.l8 | !self.l16).select(Simd::splat(1), Simd::splat(0));
        let gt = (!self.l8).select(Simd::splat(1), Simd::splat(0));

        // Select signs
        let gx = self.sign1.select(gx, -gx);
        let gy = self.l24.select_mask(self.sign2, self.sign1).select(gy, -gy);
        let gz = self.l16.select_mask(self.sign3, self.sign2).select(gz, -gz);
        let gt = self.sign3.select(gt, -gt);

        [gx.cast(), gy.cast(), gz.cast(), gt.cast()]
    }
}
