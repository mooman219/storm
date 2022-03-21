use crate::math::IDENTITY_MATRIX;
use cgmath::*;

/// Parameters
pub struct PerspectiveParams {
    /// The position of the camera..
    pub eye: Vector3<f32>,
    /// The direction the camera is looking
    pub direction: Vector3<f32>,
}

/// Simple camera for perspective projections. +Y is up, right handed system. Depth is inverted.
pub struct PerspectiveCamera {
    params: PerspectiveParams,
    logical_size: Vector2<f32>,
    fov: Rad<f32>,

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
            params: PerspectiveParams {
                eye: Vector3::new(0.0, 0.0, 0.0),
                direction: Vector3::new(1.0, 0.0, 0.0),
            },
            logical_size,
            fov: Deg(90.0).into(),

            transform: IDENTITY_MATRIX,
            transform_dirty: false,
            proj: IDENTITY_MATRIX,
            proj_dirty: true,
            proj_transform: IDENTITY_MATRIX,
            proj_transform_dirty: true,
        }
    }

    /// Gets an immutable reference to the transform parameters.
    pub fn get(&self) -> &PerspectiveParams {
        &self.params
    }

    /// Gets an mutable reference to the transform parameters.
    pub fn set(&mut self) -> &mut PerspectiveParams {
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

    /// Sets the FOV in degrees.
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = Deg(fov).into();
        self.proj_dirty = true;
        self.proj_transform_dirty = true;
    }

    /// Creates a new transform matix based on the parameters of the LayerTransform. The transform
    /// matrix is built in this order: Scale * Translation * Rotation.
    pub fn matrix(&mut self) -> Matrix4<f32> {
        if self.proj_transform_dirty {
            if self.transform_dirty {
                let eye = unsafe { core::mem::transmute(self.params.eye) };
                self.transform = Matrix4::look_to_rh(eye, self.params.direction, Vector3::new(0.0, 1.0, 0.0));
                self.transform_dirty = false;
            }
            if self.proj_dirty {
                let a = self.logical_size.x / self.logical_size.y;
                self.proj = perspective(self.fov, a, 0.01);
                self.proj_dirty = false;
            }
            self.proj_transform = self.proj * self.transform;
            self.proj_transform_dirty = false;
        }

        self.proj_transform
    }
}

#[rustfmt::skip]
fn perspective(fovy: Rad<f32>, aspect: f32, near: f32) -> Matrix4<f32> {
    let f = Rad::cot(fovy / 2.0);
    Matrix4::new(
        f / aspect,  0.0,  0.0,  0.0,
               0.0,    f,  0.0,  0.0,
               0.0,  0.0,  0.0, -1.0,
               0.0,  0.0, near,  0.0)
}
