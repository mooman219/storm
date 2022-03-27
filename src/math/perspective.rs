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
    aspect: f32,
    fov: Rad<f32>,

    view: Matrix4<f32>,
    view_dirty: bool,
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
            aspect: logical_size.x / logical_size.y,
            fov: Deg(90.0).into(),

            view: IDENTITY_MATRIX,
            view_dirty: false,
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
        self.view_dirty = true;
        self.proj_transform_dirty = true;
        &mut self.params
    }

    /// Logical size of the viewport.
    pub fn set_size(&mut self, logical_size: Vector2<f32>) {
        self.aspect = logical_size.x / logical_size.y;
        self.proj_dirty = true;
        self.proj_transform_dirty = true;
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
            if self.view_dirty {
                self.view = view(self.params.eye, self.params.direction);
                self.view_dirty = false;
            }
            if self.proj_dirty {
                self.proj = projection(self.fov, self.aspect, 0.01);
                self.proj_dirty = false;
            }
            self.proj_transform = self.proj * self.view;
            self.proj_transform_dirty = false;
        }

        self.proj_transform
    }
}

// Right handed, +Y is up.
#[rustfmt::skip]
fn view(eye: Vector3<f32>, dir: Vector3<f32>) -> Matrix4<f32> {
    let f = dir.normalize();

    // let s = f.cross(up).normalize();
    let s = Vector3::new(-f.z, 0.0, f.x);
    let mag = (s.x * s.x + s.z * s.z).sqrt().recip();
    let s = Vector3::new(s.x * mag, 0.0, s.z * mag);
    
    // let u = s.cross(f);
    let u = Vector3::new(
        -(s.z * f.y),
        (s.z * f.x) - (s.x * f.z),
        s.x * f.y,
    );

    Matrix4::new(
        s.x, u.x, -f.x, 0.0,
        s.y, u.y, -f.y, 0.0,
        s.z, u.z, -f.z, 0.0,
        -eye.dot(s), -eye.dot(u), eye.dot(f), 1.0,
    )
}

#[rustfmt::skip]
fn projection(fovy: Rad<f32>, aspect: f32, near: f32) -> Matrix4<f32> {
    let f = Rad::cot(fovy / 2.0);
    Matrix4::new(
        f / aspect,  0.0,  0.0,  0.0,
               0.0,    f,  0.0,  0.0,
               0.0,  0.0,  0.0, -1.0,
               0.0,  0.0, near,  0.0)
}
