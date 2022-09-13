struct Fbm {}

fn fbm<const LANES: usize>(
    octaves: usize,
    gain: f32,
    lacunarity: f32,
    point: [Simd<f32, LANES>; 2],
) -> Sample<LANES, 2>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    const NOISE: Simplex2d = Simplex2d::new();

    let mut result = Sample::default();

    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut scale = 0.0;
    for _ in 0..octaves {
        result += NOISE.sample(point.map(|x| x * Simd::splat(frequency))) * Simd::splat(amplitude);
        scale += amplitude;
        frequency *= lacunarity;
        amplitude *= gain;
    }
    result / Simd::splat(scale)
}
