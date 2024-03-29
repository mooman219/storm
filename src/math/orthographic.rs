use crate::math::IDENTITY_MATRIX;
use cgmath::*;

/// Creates an orthographic matrix from screen bounds with a fixed aspect ratio and with 0,0 in the
/// center.
pub fn ortho_from_bounds(bounds: &Vector2<f32>) -> Matrix4<f32> {
    let w = bounds.x / 2.0;
    let h = bounds.y / 2.0;
    ortho(-w.floor(), w.ceil(), -h.floor(), h.ceil(), -1.0, 1.0)
}

pub struct OrthographicParams {
    /// The translation of the layer.
    pub translation: Vector3<f32>,
    /// The zoom level of the layer. This is 1.0 by default, meaning 1 pixel takes up 1x1 pixels on
    /// screen.
    pub scale: f32,
    /// Rotation is measured in turns from [0, 1). Values outside of the range are wrapped into the
    /// range. For example, 1.75 is wrapped into 0.75, -0.4 is wrapped into 0.6.
    pub rotation: f32,
}

/// Simple camera for orthographic projections.
pub struct OrthographicCamera {
    params: OrthographicParams,
    pixel_perfect: bool,
    logical_size: Vector2<f32>,

    transform: Matrix4<f32>,
    transform_dirty: bool,
    ortho: Matrix4<f32>,
    ortho_dirty: bool,
    ortho_transform: Matrix4<f32>,
    ortho_transform_dirty: bool,
}

impl OrthographicCamera {
    pub fn new(logical_size: Vector2<f32>) -> OrthographicCamera {
        OrthographicCamera {
            params: OrthographicParams {
                translation: Vector3::zero(),
                scale: 1.0,
                rotation: 0.0,
            },
            pixel_perfect: true,
            logical_size,

            transform: IDENTITY_MATRIX,
            transform_dirty: false,
            ortho: IDENTITY_MATRIX,
            ortho_dirty: true,
            ortho_transform: IDENTITY_MATRIX,
            ortho_transform_dirty: true,
        }
    }

    /// Gets an immutable reference to the transform parameters.
    pub fn get(&self) -> &OrthographicParams {
        &self.params
    }

    /// Gets an mutable reference to the transform parameters.
    pub fn set(&mut self) -> &mut OrthographicParams {
        self.transform_dirty = true;
        self.ortho_transform_dirty = true;
        &mut self.params
    }

    /// Flags pixel perfect alignment.
    pub fn set_pixel_perfect(&mut self, pixel_perfect: bool) {
        if self.pixel_perfect != pixel_perfect {
            self.pixel_perfect = pixel_perfect;
            self.transform_dirty = true;
            self.ortho_transform_dirty = true;
        }
    }

    /// Logical size of the viewport.
    pub fn set_size(&mut self, logical_size: Vector2<f32>) {
        self.ortho_dirty = true;
        self.ortho_transform_dirty = true;
        self.logical_size = logical_size;
    }

    /// Creates a new transform matix based on the parameters of the LayerTransform. The transform
    /// matrix is built in this order: Scale * Translation * Rotation.
    pub fn matrix(&mut self) -> Matrix4<f32> {
        if self.transform_dirty {
            let mut translation = self.params.translation;
            if self.pixel_perfect {
                translation.x = (translation.x * self.params.scale).floor() / self.params.scale;
                translation.y = (translation.y * self.params.scale).floor() / self.params.scale;
            }
            self.transform = Matrix4::from_scale(self.params.scale)
                * Matrix4::from_translation(translation)
                * Matrix4::from_angle_z(Rad(core::f32::consts::PI * 2.0 * self.params.rotation));
            self.transform_dirty = false;
        }

        if self.ortho_dirty {
            self.ortho = ortho_from_bounds(&self.logical_size);
            self.ortho_dirty = false;
        }

        if self.ortho_transform_dirty {
            self.ortho_transform = self.ortho * self.transform;
            self.ortho_transform_dirty = false;
        }

        self.ortho_transform
    }

    /// Transforms the normalized position into a position in the transform. Typically you'll use
    /// the mouse's normalized position and convert that into world space.
    pub fn screen_to_world(&mut self, normalized_pos: Vector2<f32>) -> Vector2<f32> {
        let matrix = self.matrix().invert().unwrap();
        let value = Vector4::new(normalized_pos.x, normalized_pos.y, 0.0, 1.0);
        let value = matrix * value;
        Vector2::new(value.x, value.y)
    }
}
