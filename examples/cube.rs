use core::time::Duration;
use face::*;
use storm::cgmath::*;
use storm::{color::*, event::*, graphics::*, math::*, *};

mod face;

static TEXTURE: &[u8] = include_bytes!("resources/1.png");

/// Run with: cargo run --example cube --release
fn main() {
    start::<CubeApp>(WindowSettings {
        title: String::from("Storm: Cube"),
        display_mode: DisplayMode::Windowed {
            width: 1280,
            height: 1024,
            resizable: true,
        },
        vsync: Vsync::Disabled,
    });
}

pub struct CubeApp {
    camera: Camera,
    texture: Texture,
    buffer: Buffer<Face>,
    shader: FaceShader,
}

impl App for CubeApp {
    fn new(ctx: &mut Context<Self>) -> Self {
        ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));
        let texture = Texture::from_png(ctx, TEXTURE, TextureFiltering::none());
        let pos: Vector3<f32> = Vector3::zero();
        let mut faces: Vec<Face> = Vec::new();
        faces.push(Face {
            pos,
            kind: FaceKind::BlockPosX,
            texture: TextureSection::default(),
        });
        faces.push(Face {
            pos,
            kind: FaceKind::BlockNegX,
            texture: TextureSection::default(),
        });
        faces.push(Face {
            pos,
            kind: FaceKind::BlockPosY,
            texture: TextureSection::default(),
        });
        faces.push(Face {
            pos,
            kind: FaceKind::BlockNegY,
            texture: TextureSection::default(),
        });
        faces.push(Face {
            pos,
            kind: FaceKind::BlockPosZ,
            texture: TextureSection::default(),
        });
        faces.push(Face {
            pos,
            kind: FaceKind::BlockNegZ,
            texture: TextureSection::default(),
        });
        let mut buffer: Buffer<Face> = Buffer::new(ctx);
        buffer.set(&faces);
        let shader = FaceShader::new(ctx);

        CubeApp {
            camera: Camera::new(ctx),
            texture,
            buffer,
            shader,
        }
    }

    fn on_update(&mut self, ctx: &mut Context<Self>, delta: f32) {
        ctx.clear(ClearMode::new().with_color(RGBA8::BLUE).with_depth(0.0, DepthTest::Greater));
        self.camera.update(delta);
        self.shader.draw(self.camera.uniform(), &self.texture, &[&self.buffer])
    }

    fn on_close_requested(&mut self, ctx: &mut Context<Self>) {
        ctx.request_stop();
    }

    fn on_key_pressed(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton, is_repeat: bool) {
        if is_repeat {
            return;
        }
        match key {
            KeyboardButton::Escape => ctx.request_stop(),
            KeyboardButton::W => {
                self.camera.forward_speed += 1.0;
            }
            KeyboardButton::S => {
                self.camera.forward_speed -= 1.0;
            }
            KeyboardButton::A => {
                self.camera.strafe_speed -= 1.0;
            }
            KeyboardButton::D => {
                self.camera.strafe_speed += 1.0;
            }
            KeyboardButton::Space => {
                self.camera.vertical_speed += 1.0;
            }
            KeyboardButton::LShift => {
                self.camera.vertical_speed -= 1.0;
            }
            KeyboardButton::LControl => {
                self.camera.multiplier += 4.0;
            }
            _ => {}
        }
    }

    fn on_key_released(&mut self, _ctx: &mut Context<Self>, key: event::KeyboardButton) {
        match key {
            KeyboardButton::W => {
                self.camera.forward_speed -= 1.0;
            }
            KeyboardButton::S => {
                self.camera.forward_speed += 1.0;
            }
            KeyboardButton::A => {
                self.camera.strafe_speed += 1.0;
            }
            KeyboardButton::D => {
                self.camera.strafe_speed -= 1.0;
            }
            KeyboardButton::Space => {
                self.camera.vertical_speed -= 1.0;
            }
            KeyboardButton::LShift => {
                self.camera.vertical_speed += 1.0;
            }
            KeyboardButton::LControl => {
                self.camera.multiplier -= 4.0;
            }
            _ => {}
        }
    }

    fn on_cursor_delta(&mut self, _ctx: &mut Context<Self>, delta: cgmath::Vector2<f32>, _focused: bool) {
        self.camera.look(delta);
    }

    fn on_window_resized(
        &mut self,
        _ctx: &mut Context<Self>,
        _physical_size: cgmath::Vector2<f32>,
        _logical_size: cgmath::Vector2<f32>,
        _scale_factor: f32,
    ) {
    }
}

pub struct Camera {
    /// Transform matix.
    transform: PerspectiveCamera,
    /// Transform uniform.
    uniform: Uniform<FaceUniform>,
    /// Position vector.
    pos: Vector3<f32>,
    /// Unnormalized direction vector.
    dir: Vector3<f32>,
    /// Normalized horizontal xz plane direction vector.
    forward: Vector2<f32>,
    yaw: f32,
    pitch: f32,
    /// Positive is forward.
    pub forward_speed: f32,
    /// Positive is right.
    pub strafe_speed: f32,
    /// Positive is up.
    pub vertical_speed: f32,
    pub multiplier: f32,
}

impl Camera {
    pub fn new(ctx: &mut Context<CubeApp>) -> Camera {
        let mut transform = PerspectiveCamera::new(ctx.window_logical_size());
        let uniform = Uniform::new(ctx, &mut transform);
        Camera {
            transform,
            uniform,
            pos: Vector3::zero(),
            dir: Vector3::zero(),
            forward: Vector2::zero(),
            yaw: 0.0,
            pitch: 0.0,
            forward_speed: 0.0,
            strafe_speed: 0.0,
            vertical_speed: 0.0,
            multiplier: 2.0,
        }
    }

    pub fn resize(&mut self, logical_size: Vector2<f32>) {
        self.transform.set_size(logical_size);
        self.uniform.set(&mut self.transform);
    }

    pub fn look(&mut self, cursor_delta: Vector2<f32>) {
        const SENSITIVITY: f32 = 0.12; // Degrees per delta unit.

        self.yaw += cursor_delta.x * SENSITIVITY;
        if self.yaw < 0.0 {
            self.yaw = 360.0 - self.yaw;
        } else if self.yaw > 360.0 {
            self.yaw = self.yaw - 360.0;
        }

        self.pitch += cursor_delta.y * SENSITIVITY;
        if self.pitch < -90.0 {
            self.pitch = -90.0;
        } else if self.pitch > 89.0 {
            self.pitch = 89.0;
        }

        let cos_pitch = self.pitch.cos_deg_fast();
        self.forward = Vector2::new(self.yaw.cos_deg_fast(), self.yaw.sin_deg_fast());
        let x = cos_pitch * self.forward.x;
        let y = self.pitch.sin_deg_fast();
        let z = cos_pitch * self.forward.y;
        self.dir = Vector3::new(x, y, z);
        self.transform.set().direction = self.dir;
        self.uniform.set(&mut self.transform);
    }

    pub fn update(&mut self, time_delta: f32) {
        let forward_speed = time_delta * self.forward_speed * self.multiplier;
        let strafe_speed = time_delta * self.strafe_speed * self.multiplier;
        let vertical_speed = time_delta * self.vertical_speed * self.multiplier;
        self.pos.x += (self.forward.x * forward_speed) + (-self.forward.y * strafe_speed);
        self.pos.z += (self.forward.y * forward_speed) + (self.forward.x * strafe_speed);
        self.pos.y += vertical_speed;
        self.transform.set().eye = self.pos;
        self.uniform.set(&mut self.transform);
    }

    pub fn uniform(&self) -> &Uniform<FaceUniform> {
        &self.uniform
    }
}
