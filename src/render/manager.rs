use cgmath::*;
use layer::*;
use render::buffer::geometry::*;
use render::buffer::grouped::*;
use render::raw::*;
use render::shader::*;
use render::vertex::*;

struct Layer {
    desc: LayerDescription,
    sprites: GeometryBuffer<TextureVertex>,
    text: GroupedBuffer<TextureVertex>,
    matrix_translate_scaled: Matrix4<f32>,
    matrix_full: Matrix4<f32>,
}

pub struct SceneManager {
    shader: TextureShader,
    layers: Vec<Layer>,
    matrix_bounds: Matrix4<f32>,
}

fn matrix_from_bounds(bounds: &Vector2<f64>) -> Matrix4<f32> {
    let w = bounds.x as f32 / 2.0;
    let h = bounds.y as f32 / 2.0;
    ortho(-w.floor(), w.ceil(), -h.floor(), h.ceil(), std::f32::MIN, std::f32::MAX)
}

fn matrix_from_translate_scaled(translation: &Vector2<f32>, scale: f32) -> Matrix4<f32> {
    Matrix4::from_translation(translation.extend(0.0)) * Matrix4::from_scale(scale)
}

impl SceneManager {
    pub fn new(bounds: &Vector2<f64>) -> SceneManager {
        let manager = SceneManager {
            shader: TextureShader::new(),
            layers: Vec::new(),
            matrix_bounds: matrix_from_bounds(bounds),
        };

        // Shader setup
        manager.shader.bind();
        manager.shader.ortho(manager.matrix_bounds);

        manager
    }

    pub fn layer_create(&mut self, index: usize, desc: &LayerDescription) {
        let matrix_translate_scaled = matrix_from_translate_scaled(&desc.translation, desc.scale);
        let matrix_full = self.matrix_bounds * matrix_translate_scaled;
        let slot = Layer {
            desc: *desc,
            sprites: GeometryBuffer::new(),
            text: GroupedBuffer::new(),
            matrix_translate_scaled: matrix_translate_scaled,
            matrix_full: matrix_full,
        };
        self.layers.insert(index, slot);
    }

    pub fn layer_update(&mut self, index: usize, desc: &LayerDescription) {
        let layer = &mut self.layers[index];
        let matrix_translate_scaled = matrix_from_translate_scaled(&desc.translation, desc.scale);
        let matrix_full = self.matrix_bounds * matrix_translate_scaled;
        layer.desc = *desc;
        layer.matrix_translate_scaled = matrix_translate_scaled;
        layer.matrix_full = matrix_full;
    }

    pub fn layer_remove(&mut self, index: usize) {
        self.layers.remove(index);
    }

    pub fn layer_clear(&mut self, index: usize) {
        let layer = &mut self.layers[index];
        layer.sprites.clear();
        layer.text.clear();
    }

    pub fn sprite_create(&mut self, layer_index: usize, vertex: &TextureVertex) {
        self.layers[layer_index].sprites.push(*vertex);
    }

    pub fn sprite_update(&mut self, layer_index: usize, sprite_index: usize, vertex: &TextureVertex) {
        self.layers[layer_index].sprites.update(sprite_index, *vertex);
    }

    pub fn sprite_remove(&mut self, layer_index: usize, sprite_index: usize) {
        self.layers[layer_index].sprites.swap_remove(sprite_index);
    }

    pub fn text_create(&mut self, layer_index: usize, vertices: Vec<TextureVertex>) {
        self.layers[layer_index].text.push(vertices);
    }

    pub fn text_update(&mut self, layer_index: usize, text_index: usize, vertices: Vec<TextureVertex>) {
        self.layers[layer_index].text.update(text_index, vertices);
    }

    pub fn text_remove(&mut self, layer_index: usize, text_index: usize) {
        self.layers[layer_index].text.swap_remove(text_index);
    }

    pub fn resize(&mut self, bounds: &Vector2<f64>) {
        self.matrix_bounds = matrix_from_bounds(bounds);
        for layer in &mut self.layers {
            layer.matrix_full = self.matrix_bounds * layer.matrix_translate_scaled;
        }
    }

    /// Sync the state of the layers with their respective vertex buffers. Call this after
    /// layer/sprite modifications for the frame.
    pub fn sync(&mut self) {
        for layer in &mut self.layers {
            if layer.desc.visible {
                layer.sprites.sync();
                layer.text.sync();
            }
        }
    }

    /// Draw the last synced layer state to the screen. This clears the color and depth buffers.
    /// This does not swap the frame buffer.
    pub fn draw(&mut self) {
        clear(ClearBit::ColorBuffer);
        for layer in &mut self.layers {
            if layer.desc.visible {
                clear(ClearBit::DepthBuffer);
                self.shader.ortho(layer.matrix_full);
                if layer.sprites.vertices() > 0 {
                    self.shader.texture(TextureUnit::Atlas);
                    layer.sprites.draw();
                }
                if layer.text.groups() > 0 {
                    self.shader.texture(TextureUnit::Font);
                    layer.text.draw();
                }
            }
        }
    }
}
