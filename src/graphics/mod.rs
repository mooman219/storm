mod buffer;
mod shader;
mod texture;
mod texture_section;
mod uniform;
mod vertex_descriptor;

/// Bundled sample shaders for basic sprite and text rendering.
pub mod shaders;

pub(crate) use self::vertex_descriptor::configure_vertex;

pub use self::buffer::Buffer;
pub use self::shader::{DrawMode, Shader, ShaderDescriptor};
pub use self::texture::Texture;
pub use self::texture_section::TextureSection;
pub use self::uniform::Uniform;
pub use self::vertex_descriptor::{VertexAttribute, VertexDescriptor, VertexInputType, VertexOutputType};
/// Macro for working with data adhering to GLSL’s std140 layout specification.
pub use crevice::std140::AsStd140;
