use cgmath::*;

/// Utility type to create simple transformation matrices.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LayerTransform {
    /// The translation of the layer.
    pub translation: Vector2<f32>,
    /// The zoom level of the layer. This is 1.0 by default, meaning 1 pixel takes up 1x1 pixels on
    /// screen.
    pub scale: f32,
    /// Rotation is measured in turns from [0, 1). Values outside of the range are wrapped into the
    /// range. For example, 1.75 is wrapped into 0.75, -0.4 is wrapped into 0.6.
    pub rotation: f32,
}

impl LayerTransform {
    pub fn new() -> LayerTransform {
        LayerTransform {
            translation: Vector2::new(0.0, 0.0),
            scale: 1.0,
            rotation: 0.0,
        }
    }

    /// Creates a new transform matix based on the parameters of the LayerTransform. The transform
    /// matrix is built in this order: Scale * Translation * Rotation.
    pub fn matrix(&self) -> Matrix4<f32> {
        let mut translation = self.translation;
        translation.x = (translation.x * self.scale).floor() / self.scale;
        translation.y = (translation.y * self.scale).floor() / self.scale;
        Matrix4::from_scale(self.scale)
            * Matrix4::from_translation(translation.extend(0.0))
            * Matrix4::from_angle_z(Rad(core::f32::consts::PI * 2.0 * self.rotation))
    }
}
