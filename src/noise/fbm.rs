use super::NoiseFn;
// use core::simd::{LaneCount, Simd, SupportedLaneCount};

/// Noise function that outputs fBm (fractal Brownian motion) noise.
///
/// fBm is a _monofractal_ method. In essence, fBm has a _constant_ fractal dimension. It is as
/// close to statistically _homogeneous_ and _isotropic_ as possible. Homogeneous means "the same
/// everywhere" and isotropic means  "the same in all directions" (note that the two do not mean the
/// same
/// thing).
///
/// The main difference between fractal Brownian motion and regular Brownian motion is that while
/// the increments in Brownian motion are independent, the increments in fractal Brownian motion
/// depend on the previous increment.
///
/// fBm is the result of several noise functions of ever-increasing frequency and ever-decreasing
/// amplitude.
#[derive(Clone, Copy, Debug)]
struct Fbm<const DIM: usize, T: NoiseFn<DIM>> {
    pub source: T,
    /// Total number of frequency octaves to generate the noise with.
    ///
    /// The number of octaves control the _amount of detail_ in the noise
    /// function. Adding more octaves increases the detail, with the drawback
    /// of increasing the calculation time.
    pub octaves: usize,
    pub gain: f32,
    /// A multiplier that determines how quickly the frequency increases for
    /// each successive octave in the noise function.
    ///
    /// The frequency of each successive octave is equal to the product of the
    /// previous octave's frequency and the lacunarity value.
    ///
    /// A lacunarity of 2.0 results in the frequency doubling every octave. For
    /// almost all cases, 2.0 is a good value to use.
    pub lacunarity: f32,
}

impl<const DIM: usize, T: NoiseFn<DIM>> Fbm<DIM, T> {
    pub fn new(source: T, octaves: usize, gain: f32, lacunarity: f32) -> Self {
        Self {
            source,
            octaves,
            gain,
            lacunarity,
        }
    }
}

// impl<const DIM: usize, A, B> NoiseFn<DIM> for Add<DIM, A, B>
// where
//     A: NoiseFn<DIM>,
//     B: NoiseFn<DIM>,
// {
//     #[inline(always)]
//     fn sample<const LANES: usize>(&self, x: [Simd<f32, LANES>; DIM]) -> Simd<f32, LANES>
//     where
//         LaneCount<LANES>: SupportedLaneCount,
//     {
//         self.first.sample(x) + self.second.sample(x)
//     }
// }

// struct Fbm {}

// fn fbm<const LANES: usize>(
//     octaves: usize,
//     gain: f32,
//     lacunarity: f32,
//     point: [Simd<f32, LANES>; 2],
// ) -> Sample<LANES, 2>
// where
//     LaneCount<LANES>: SupportedLaneCount,
// {
//     const NOISE: Simplex2d = Simplex2d::new();

//     let mut result = Sample::default();

//     let mut frequency = 1.0;
//     let mut amplitude = 1.0;
//     let mut scale = 0.0;
//     for _ in 0..octaves {
//         result += NOISE.sample(point.map(|x| x * Simd::splat(frequency))) * Simd::splat(amplitude);
//         scale += amplitude;
//         frequency *= lacunarity;
//         amplitude *= gain;
//     }
//     result / Simd::splat(scale)
// }
