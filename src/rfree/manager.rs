use batch::*;
use cgmath::*;
use render::buffer::geometry::*;
use render::raw::*;
use render::shader::*;
use render::vertex::*;

struct Batch {
    desc: BatchDescription,
    sprites: GeometryBuffer<TextureVertex>,
    strings: GeometryBuffer<TextureVertex>,
    matrix_translate_scaled: Matrix4<f32>,
    matrix_full: Matrix4<f32>,
}

pub struct SceneManager {
    shader: TextureShader,
    batches: Vec<Batch>,
    matrix_bounds: Matrix4<f32>,
}

fn matrix_from_bounds(bounds: &Vector2<f64>) -> Matrix4<f32> {
    let w = bounds.x as f32 / 2.0;
    let h = bounds.y as f32 / 2.0;
    ortho(-w.floor(), w.ceil(), -h.floor(), h.ceil(), -1000.0, 1000.0)
}

fn matrix_from_translate_scaled(translation: &Vector2<f32>, scale: f32) -> Matrix4<f32> {
    Matrix4::from_translation(translation.extend(0.0)) * Matrix4::from_scale(scale)
}

impl SceneManager {
    pub fn new(bounds: &Vector2<f64>) -> SceneManager {
        let manager = SceneManager {
            shader: TextureShader::new(),
            batches: Vec::new(),
            matrix_bounds: matrix_from_bounds(bounds),
        };

        // Shader setup
        manager.shader.bind();
        manager.shader.ortho(manager.matrix_bounds);

        manager
    }

    pub fn batch_create(&mut self) {
        let desc = BatchDescription::default();
        let matrix_translate_scaled = matrix_from_translate_scaled(&desc.translation, desc.scale);
        let matrix_full = self.matrix_bounds * matrix_translate_scaled;
        let slot = Batch {
            desc: desc,
            sprites: GeometryBuffer::new(),
            strings: GeometryBuffer::new(),
            matrix_translate_scaled: matrix_translate_scaled,
            matrix_full: matrix_full,
        };
        self.batches.push(slot);
    }

    pub fn batch_update(&mut self, index: usize, desc: &BatchDescription) {
        let batch = &mut self.batches[index];
        let matrix_translate_scaled = matrix_from_translate_scaled(&desc.translation, desc.scale);
        let matrix_full = self.matrix_bounds * matrix_translate_scaled;
        batch.desc = *desc;
        batch.matrix_translate_scaled = matrix_translate_scaled;
        batch.matrix_full = matrix_full;
    }

    pub fn batch_remove(&mut self, index: usize) {
        self.batches.swap_remove(index);
    }

    pub fn batch_clear(&mut self, index: usize) {
        let batch = &mut self.batches[index];
        batch.sprites.clear();
        batch.strings.clear();
    }

    pub fn sprite_add(&mut self, batch_index: usize, vertex: TextureVertex) {
        self.batches[batch_index].sprites.push(vertex);
    }

    pub fn sprite_clear(&mut self, batch_index: usize) {
        self.batches[batch_index].sprites.clear();
    }

    pub fn string_add(&mut self, batch_index: usize, vertices: Vec<TextureVertex>) {
        self.batches[batch_index].strings.push_range(vertices);
    }

    pub fn string_clear(&mut self, batch_index: usize) {
        self.batches[batch_index].strings.clear();
    }

    pub fn resize(&mut self, bounds: &Vector2<f64>) {
        self.matrix_bounds = matrix_from_bounds(bounds);
        for batch in &mut self.batches {
            batch.matrix_full = self.matrix_bounds * batch.matrix_translate_scaled;
        }
    }

    /// Sync the state of the batches with their respective vertex buffers. Call this after
    /// batch/sprite modifications for the frame.
    pub fn sync(&mut self) {
        for batch in &mut self.batches {
            if batch.desc.visible {
                batch.sprites.sync();
                batch.strings.sync();
            }
        }
    }

    /// Draw the last synced batch state to the screen. This clears the color and depth buffers.
    /// This does not swap the frame buffer.
    pub fn draw(&mut self) {
        clear(ClearBit::ColorBuffer | ClearBit::DepthBuffer);
        for batch in &mut self.batches {
            if batch.desc.visible {
                self.shader.ortho(batch.matrix_full);
                if batch.sprites.len() > 0 {
                    self.shader.texture(TextureUnit::Atlas);
                    batch.sprites.draw();
                }
                if batch.strings.len() > 0 {
                    self.shader.texture(TextureUnit::Font);
                    batch.strings.draw();
                }
            }
        }
    }
}
