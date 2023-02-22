mod data;

pub use self::data::TrianglePoint;

impl storm::graphics::ShaderDescriptor for TriangleShader {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const TEXTURE_NAMES: &'static [&'static str] = &[];
    const UNIFORM_NAMES: &'static [&'static str] = &["vertex"];
}

pub struct TriangleShader();
