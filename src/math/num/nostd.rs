use crate::math::Float;

impl Float for f32 {
    fn sin_deg_fast(self) -> Self {
        super::sin_deg(self)
    }

    fn sin_rad_fast(self) -> Self {
        super::sin_rad(self)
    }

    fn cos_deg_fast(self) -> Self {
        super::cos_deg(self)
    }

    fn cos_rad_fast(self) -> Self {
        super::cos_rad(self)
    }

    fn atan2_fast(self, x: f32) -> Self {
        super::atan2(self, x)
    }

    fn inv_sqrt(self) -> Self {
        let i = self.to_bits();
        let i = 0x5f3759df - (i >> 1);
        let y = f32::from_bits(i);
        y * (1.5 - 0.5 * self * y * y)
    }
}
