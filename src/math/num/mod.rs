mod nostd;
mod trigonometry;

use self::trigonometry::{atan2, cos_deg, cos_rad, sin_deg, sin_rad};
use crate::graphics::IndiceType;

/// Extra functions on floating point values.
pub trait Float {
    /// Computes the sine of a number. Input in degrees, output in radians.
    ///
    /// * Average error of 0.00060 radians.
    /// * Largest error of 0.00229 radians.
    /// * Speedup of 30x over f32.sin();
    fn sin_deg_fast(self) -> Self;

    /// Computes the sine of a number. Input in radians, output in radians.
    ///
    /// * Average error of 0.00060 radians.
    /// * Largest error of 0.00229 radians.
    /// * Speedup of 30x over f32.sin();
    fn sin_rad_fast(self) -> Self;

    /// Computes the cosine of a number. Input in degrees, output in radians.
    ///
    /// * Average error of 0.00060 radians.
    /// * Largest error of 0.00229 radians.
    /// * Speedup of 30x over f32.cos();
    fn cos_deg_fast(self) -> Self;

    /// Computes the cosine of a number. Input in radians, output in radians.
    ///
    /// * Average error of 0.00060 radians.
    /// * Largest error of 0.00229 radians.
    /// * Speedup of 30x over f32.cos();
    fn cos_rad_fast(self) -> Self;

    /// Computes the four quadrant arctangent of self (y) and other (x) in radians.
    ///
    /// * Average error of 0.00231 radians.
    /// * Largest error of 0.00488 radians.
    /// * Speedup of 20.67x over f32.atan2(y);
    fn atan2_fast(self, x: f32) -> Self;

    /// Quake fast inverse square root.
    fn inv_sqrt(self) -> Self;

    /// Converts perceptual (db) into linear ([0, 1]).
    fn perceptual(self) -> Self;
}

/// Extra functions on unsigned integers.
pub trait UnsignedInteger: Copy + Clone {
    const INDICE_TYPE: IndiceType;
}

impl UnsignedInteger for u8 {
    const INDICE_TYPE: IndiceType = IndiceType::UnsignedByte;
}

impl UnsignedInteger for u16 {
    const INDICE_TYPE: IndiceType = IndiceType::UnsignedShort;
}

impl UnsignedInteger for u32 {
    const INDICE_TYPE: IndiceType = IndiceType::UnsignedInt;
}
