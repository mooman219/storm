mod batch;
mod buffer;
mod raw;
mod shader;
mod texture_handle;
mod vertex;
mod window;

use crate::render::raw::{BlendFactor, Capability, ClearBit, CullFace, DepthTest, OpenGL, TextureUnit};
use crate::render::shader::TextureShader;
use crate::render::texture_handle::*;
use crate::render::window::*;
use crate::text::*;
use crate::texture::*;
use crate::types::*;
use cgmath::*;

pub use self::batch::Batch;

fn matrix_from_bounds(bounds: &Vector2<f32>) -> Matrix4<f32> {
    let w = bounds.x / 2.0;
    let h = bounds.y / 2.0;
    ortho(-w.floor(), w.ceil(), -h.floor(), h.ceil(), -1.0, 1.0)
}

pub struct Renderer {
    window: OpenGLWindow,
    gl: OpenGL,
    shader: TextureShader,
    texture_atlas: TextureHandle,
    batches: Vec<Batch>,
    matrix_bounds: Matrix4<f32>,
    logical_size: Vector2<f32>,
    atlas: TextureAtlas,
    text_cache: TextCache,
}

impl Renderer {
    pub(crate) fn new(desc: &WindowSettings, event_loop: &winit::event_loop::EventLoop<()>) -> Renderer {
        let (window, gl) = OpenGLWindow::new(desc, event_loop);

        // Setup cabilities.
        let gl = OpenGL::new(gl);
        gl.enable(Capability::CullFace);
        gl.enable(Capability::Blend);
        gl.enable(Capability::DepthTest);
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.depth_func(DepthTest::Less);
        gl.blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        gl.cull_face(CullFace::Back);

        // Bind shader once.
        let shader = TextureShader::new(gl.clone());
        shader.bind();
        shader.texture(TextureUnit::Atlas);

        let texture_atlas = TextureHandle::new(gl.clone(), TextureUnit::Atlas);
        let logical_size = window.logical_size();

        Renderer {
            window,
            gl,
            shader,
            texture_atlas,
            batches: Vec::new(),
            matrix_bounds: matrix_from_bounds(&logical_size),
            logical_size,
            atlas: TextureAtlas::new(),
            text_cache: TextCache::new(),
        }
    }

    // ////////////////////////////////////////////////////////
    // Batch
    // ////////////////////////////////////////////////////////

    pub fn batch_create(&mut self) -> Batch {
        let (a, b) = Batch::new(self.gl.clone(), &self.matrix_bounds);
        self.batches.push(a);
        b
    }

    // ////////////////////////////////////////////////////////
    // String
    // ////////////////////////////////////////////////////////

    pub fn font_create(&mut self, bytes: &[u8]) -> FontToken {
        FontToken::new(self.text_cache.add_font_bytes(bytes))
    }

    pub fn text_append(&mut self, descs: &Vec<Text>, output: &mut Vec<Sprite>) {
        for desc in descs {
            self.text_cache.rasterize(&mut self.atlas, desc, output);
        }
    }

    pub fn text_clear(&mut self, descs: &Vec<Text>, output: &mut Vec<Sprite>) {
        unsafe { output.set_len(0) };
        for desc in descs {
            self.text_cache.rasterize(&mut self.atlas, desc, output);
        }
    }

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    pub fn texture_create(&mut self, bytes: &[u8], format: TextureFormat) -> Texture {
        let image = Image::from_raw(bytes, format);
        let uv = self.atlas.add(image);
        Texture(uv)
    }

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    pub fn window_check_resize(&mut self) {
        let new_logical_size = self.window.logical_size();
        if self.logical_size != new_logical_size {
            self.logical_size = new_logical_size;
            let new_physical_size = self.window.physical_size();
            self.gl.viewport(0, 0, new_physical_size.x as i32, new_physical_size.y as i32);

            trace!("Window resized: Physical({:?}) Logical({:?})", new_physical_size, new_logical_size);

            // Update the full transform of all the batches.
            self.matrix_bounds = matrix_from_bounds(&new_logical_size);
            for batch in &mut self.batches {
                batch.set_ortho(&self.matrix_bounds);
            }
        }
    }

    pub fn window_logical_size(&self) -> Vector2<f32> {
        self.logical_size
    }

    pub fn window_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn window_display_mode(&mut self, display_mode: DisplayMode) {
        self.window.set_display_mode(display_mode);
    }

    pub fn clear_color(&mut self, clear_color: RGBA8) {
        let color: Vector4<f32> = clear_color.into();
        self.gl.clear_color(color.x, color.y, color.z, color.w);
    }

    pub fn draw(&mut self) {
        self.gl.clear(ClearBit::ColorBuffer | ClearBit::DepthBuffer);
        if let Some(atlas) = self.atlas.sync() {
            self.texture_atlas.set_texture(atlas);
        }
        for batch in &mut self.batches {
            // todo, cleanup batches with a count of 1
            // can do it in drop
            batch.draw(&self.shader);
        }
        self.window.swap_buffers();
    }
}
