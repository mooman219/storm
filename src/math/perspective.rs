use cgmath::*;

pub const IDENTITY_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, //
    0.0, 1.0, 0.0, 0.0, //
    0.0, 0.0, 1.0, 0.0, //
    0.0, 0.0, 0.0, 1.0, //
);

/// Parameters
pub struct TransformParameters {
    /// The translation of the layer.
    pub translation: Vector3<f32>,
    /// The zoom level of the layer. This is 1.0 by default, meaning 1 pixel takes up 1x1 pixels on
    /// screen.
    pub scale: f32,
    /// Rotation is measured in turns from [0, 1). Values outside of the range are wrapped into the
    /// range. For example, 1.75 is wrapped into 0.75, -0.4 is wrapped into 0.6.
    pub rotation: f32,
}

pub struct PerspectiveCamera {
    params: TransformParameters,
    logical_size: Vector2<f32>,

    transform: Matrix4<f32>,
    transform_dirty: bool,
    proj: Matrix4<f32>,
    proj_dirty: bool,
    proj_transform: Matrix4<f32>,
    proj_transform_dirty: bool,
}

impl PerspectiveCamera {
    pub fn new(logical_size: Vector2<f32>) -> PerspectiveCamera {
        PerspectiveCamera {
            params: TransformParameters {
                translation: Vector3::new(0.0, 0.0, 0.0),
                scale: 1.0,
                rotation: 0.0,
            },
            logical_size,

            transform: IDENTITY_MATRIX,
            transform_dirty: false,
            proj: IDENTITY_MATRIX,
            proj_dirty: true,
            proj_transform: IDENTITY_MATRIX,
            proj_transform_dirty: true,
        }
    }

    pub fn get(&mut self) -> &TransformParameters {
        &self.params
    }

    pub fn set(&mut self) -> &mut TransformParameters {
        self.transform_dirty = true;
        self.proj_transform_dirty = true;
        &mut self.params
    }

    /// Logical size of the viewport.
    pub fn set_size(&mut self, logical_size: Vector2<f32>) {
        self.proj_dirty = true;
        self.proj_transform_dirty = true;
        self.logical_size = logical_size;
    }

    /// Creates a new transform matix based on the parameters of the LayerTransform. The transform
    /// matrix is built in this order: Scale * Translation * Rotation.
    pub fn matrix(&mut self) -> Matrix4<f32> {
        if self.proj_transform_dirty {
            if self.transform_dirty {
                self.transform = Matrix4::from_scale(self.params.scale)
                    * Matrix4::from_translation(self.params.translation)
                    * Matrix4::from_angle_z(Rad(core::f32::consts::PI * 2.0 * self.params.rotation));
                self.transform_dirty = false;
            }
            if self.proj_dirty {
                let a = self.logical_size.x / self.logical_size.y;
                self.proj = perspective(Deg::<f32>(100.0), a, 0.001, 100.0);
                self.proj_dirty = false;
            }
            self.proj_transform = self.proj * self.transform;
            self.proj_transform_dirty = false;
        }

        self.proj_transform
    }
}
