// This file is licensed under multiple licenses, and is a modified version of
// https://github.com/Ralith/clatter/blob/main/src/simplex.rs. A copy of the licenses can be found
// here: in the path licenses/Ralith/clatter.

use core::simd::{LaneCount, Simd, SupportedLaneCount};

#[inline(always)]
pub fn pcg<const LANES: usize>(v: Simd<i32, LANES>) -> Simd<i32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    // PCG hash function from "Hash Functions for GPU Rendering"
    let state = v * Simd::splat(747796405) + Simd::splat(2891336453u32 as i32);
    let word = ((state >> ((state >> Simd::splat(28)) + Simd::splat(4))) ^ state) * Simd::splat(277803737);
    (word >> Simd::splat(22)) ^ word
}

// For completeness
#[allow(dead_code)]
#[inline(always)]
pub fn pcg_2d<const LANES: usize>([mut vx, mut vy]: [Simd<i32, LANES>; 2]) -> [Simd<i32, LANES>; 2]
where
    LaneCount<LANES>: SupportedLaneCount,
{
    vx *= Simd::splat(1664525) + Simd::splat(1013904223);
    vy *= Simd::splat(1664525) + Simd::splat(1013904223);

    vx += vy * Simd::splat(1664525);
    vy += vx * Simd::splat(1664525);

    vx ^= vx >> Simd::splat(16);
    vy ^= vy >> Simd::splat(16);

    vx += vy * Simd::splat(1664525);
    vy += vx * Simd::splat(1664525);

    vx ^= vx >> Simd::splat(16);
    vy ^= vy >> Simd::splat(16);

    [vx, vy]
}

#[inline(always)]
pub fn pcg_3d<const LANES: usize>([mut vx, mut vy, mut vz]: [Simd<i32, LANES>; 3]) -> [Simd<i32, LANES>; 3]
where
    LaneCount<LANES>: SupportedLaneCount,
{
    // PCG3D hash function from "Hash Functions for GPU Rendering"
    vx = vx * Simd::splat(1664525) + Simd::splat(1013904223);
    vy = vy * Simd::splat(1664525) + Simd::splat(1013904223);
    vz = vz * Simd::splat(1664525) + Simd::splat(1013904223);

    vx += vy * vz;
    vy += vz * vx;
    vz += vx * vy;

    vx = vx ^ (vx >> Simd::splat(16));
    vy = vy ^ (vy >> Simd::splat(16));
    vz = vz ^ (vz >> Simd::splat(16));

    vx += vy * vz;
    vy += vz * vx;
    vz += vx * vy;

    [vx, vy, vz]
}

#[inline(always)]
pub fn pcg_4d<const LANES: usize>(
    [mut vx, mut vy, mut vz, mut vw]: [Simd<i32, LANES>; 4],
) -> [Simd<i32, LANES>; 4]
where
    LaneCount<LANES>: SupportedLaneCount,
{
    // PCG4D hash function from "Hash Functions for GPU Rendering"
    vx = vx * Simd::splat(1664525) + Simd::splat(1013904223);
    vy = vy * Simd::splat(1664525) + Simd::splat(1013904223);
    vz = vz * Simd::splat(1664525) + Simd::splat(1013904223);
    vw = vw * Simd::splat(1664525) + Simd::splat(1013904223);

    vx += vy * vw;
    vy += vz * vx;
    vz += vx * vy;
    vw += vy * vz;

    vx = vx ^ (vx >> Simd::splat(16));
    vy = vy ^ (vy >> Simd::splat(16));
    vz = vz ^ (vz >> Simd::splat(16));
    vw = vw ^ (vw >> Simd::splat(16));

    vx += vy * vw;
    vy += vz * vx;
    vz += vx * vy;
    vw += vy * vz;

    [vx, vy, vz, vw]
}
