/// Bundled sample shaders for basic sprite and text rendering.
pub mod shaders;
/// Rust struct types that are compatible with the GLSL std140 memory layout.
pub mod std140;

mod buffer;
mod opengl;
mod shader;
mod state;
mod texture;
mod texture_section;
mod uniform;
mod vertex_descriptor;
mod window;

pub use self::buffer::Buffer;
pub use self::opengl::{ClearMode, DrawMode};
pub use self::shader::{Shader, ShaderDescriptor};
pub use self::state::{
    clear, default_texture, max_texture_anisotropy, max_texture_size, set_window_display_mode,
    set_window_title, viewport_logical_size, viewport_physical_size, window_cursor_grab,
    window_cursor_visibility, window_logical_size, window_physical_size,
};
pub use self::texture::{Texture, TextureFiltering};
pub use self::texture_section::TextureSection;
pub use self::uniform::Uniform;
pub use self::vertex_descriptor::{VertexAttribute, VertexDescriptor, VertexInputType, VertexOutputType};
pub use self::window::{DisplayMode, Vsync, WindowSettings};

pub(crate) use self::opengl::*;
pub(crate) use self::state::OpenGLState;
pub(crate) use self::vertex_descriptor::configure_vertex;
pub(crate) use self::window::{OpenGLWindow, OpenGLWindowContract};
