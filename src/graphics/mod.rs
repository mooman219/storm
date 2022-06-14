/// Bundled sample shaders for basic sprite and text rendering.
pub mod shaders;
/// Rust struct types that are compatible with the GLSL std140 memory layout.
pub mod std140;

mod buffer;
mod opengl;
mod shader;
mod state;
mod texture;
mod texture_atlas;
mod texture_section;
mod uniform;
mod vertex_descriptor;
mod window;

pub use self::buffer::Buffer;
pub use self::opengl::{ClearMode, DepthTest, DrawMode, IndiceType};
pub use self::shader::{Shader, ShaderDescriptor};
pub use self::texture::{Texture, TextureFiltering};
pub use self::texture_atlas::TextureAtlas;
pub use self::texture_section::TextureSection;
pub use self::uniform::Uniform;
pub use self::vertex_descriptor::{
    VertexAttribute, VertexDescriptor, VertexInputType, VertexInstancing, VertexOutputType,
};
pub use self::window::{DisplayMode, Vsync, WindowSettings};

pub(crate) use self::opengl::*;
pub(crate) use self::state::{graphics, OpenGLState};
pub(crate) use self::vertex_descriptor::configure_vertex;
pub(crate) use self::window::{OpenGLWindow, OpenGLWindowContract};
