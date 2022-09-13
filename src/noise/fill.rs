use super::NoiseFn;
use alloc::vec::Vec;
use core::simd::{LaneCount, Simd, SupportedLaneCount};

/// Base trait for aggregating noise.
///
/// A fill function is a struct that calculates and outputs a value given a n-dimensional input
/// value and aggregates it into a 1 dimensional vector.
pub trait FillFn<const DIM: usize> {
    fn fill<const LANES: usize>(
        &self,
        offset: [f32; DIM],
        size: [usize; DIM],
        step: f32,
        output: &mut Vec<f32>,
    ) where
        LaneCount<LANES>: SupportedLaneCount;
}

fn increment<const LANES: usize>(step: f32) -> Simd<f32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let mut inc = Simd::splat(0.0);
    let mut next = step;
    for i in 1..LANES {
        inc[i] += next;
        next += step;
    }
    inc
}

impl<Noise> FillFn<1> for Noise
where
    Noise: NoiseFn<1>,
{
    fn fill<const LANES: usize>(
        &self,
        [offset_x]: [f32; 1],
        [size_x]: [usize; 1],
        step: f32,
        output: &mut Vec<f32>,
    ) where
        LaneCount<LANES>: SupportedLaneCount,
    {
        let inc_x = increment(step) + Simd::splat(offset_x);
        let mut x = 0;
        'loop_x: loop {
            let px = Simd::splat(x as f32 * step) + inc_x;
            let sample = self.sample([px]);
            for i in 0..LANES {
                if x + i >= size_x {
                    break 'loop_x;
                }
                output.push(sample[i]);
            }
            x += LANES;
        }
    }
}

impl<Noise> FillFn<2> for Noise
where
    Noise: NoiseFn<2>,
{
    fn fill<const LANES: usize>(
        &self,
        [offset_x, offset_y]: [f32; 2],
        [size_x, size_y]: [usize; 2],
        step: f32,
        output: &mut Vec<f32>,
    ) where
        LaneCount<LANES>: SupportedLaneCount,
    {
        let inc_x = increment(step) + Simd::splat(offset_x);
        let offset_y = Simd::splat(offset_y);
        for y in 0..size_y {
            let py = Simd::splat(y as f32 * step) + offset_y;
            let mut x = 0;
            'loop_x: loop {
                let px = Simd::splat(x as f32 * step) + inc_x;
                let sample = self.sample([px, py]);
                for i in 0..LANES {
                    if x + i >= size_x {
                        break 'loop_x;
                    }
                    output.push(sample[i]);
                }
                x += LANES;
            }
        }
    }
}

impl<Noise> FillFn<3> for Noise
where
    Noise: NoiseFn<3>,
{
    fn fill<const LANES: usize>(
        &self,
        [offset_x, offset_y, offset_z]: [f32; 3],
        [size_x, size_y, size_z]: [usize; 3],
        step: f32,
        output: &mut Vec<f32>,
    ) where
        LaneCount<LANES>: SupportedLaneCount,
    {
        let inc_x = increment(step) + Simd::splat(offset_x);
        let offset_y = Simd::splat(offset_y);
        let offset_z = Simd::splat(offset_z);
        for z in 0..size_z {
            let pz = Simd::splat(z as f32 * step) + offset_z;
            for y in 0..size_y {
                let py = Simd::splat(y as f32 * step) + offset_y;
                let mut x = 0;
                'loop_x: loop {
                    let px = Simd::splat(x as f32 * step) + inc_x;
                    let sample = self.sample([px, py, pz]);
                    for i in 0..LANES {
                        if x + i >= size_x {
                            break 'loop_x;
                        }
                        output.push(sample[i]);
                    }
                    x += LANES;
                }
            }
        }
    }
}

impl<Noise> FillFn<4> for Noise
where
    Noise: NoiseFn<4>,
{
    fn fill<const LANES: usize>(
        &self,
        [offset_x, offset_y, offset_z, offset_w]: [f32; 4],
        [size_x, size_y, size_z, size_w]: [usize; 4],
        step: f32,
        output: &mut Vec<f32>,
    ) where
        LaneCount<LANES>: SupportedLaneCount,
    {
        let inc_x = increment(step) + Simd::splat(offset_x);
        let offset_y = Simd::splat(offset_y);
        let offset_z = Simd::splat(offset_z);
        let offset_w = Simd::splat(offset_w);
        for w in 0..size_w {
            let pw = Simd::splat(w as f32 * step) + offset_w;
            for z in 0..size_z {
                let pz = Simd::splat(z as f32 * step) + offset_z;
                for y in 0..size_y {
                    let py = Simd::splat(y as f32 * step) + offset_y;
                    let mut x = 0;
                    'loop_x: loop {
                        let px = Simd::splat(x as f32 * step) + inc_x;
                        let sample = self.sample([px, py, pz, pw]);
                        for i in 0..LANES {
                            if x + i >= size_x {
                                break 'loop_x;
                            }
                            output.push(sample[i]);
                        }
                        x += LANES;
                    }
                }
            }
        }
    }
}
