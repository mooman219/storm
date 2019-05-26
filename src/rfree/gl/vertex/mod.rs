mod texture;
mod vertex_array;

pub use render::gl::vertex::texture::*;
pub use render::gl::vertex::vertex_array::*;

pub trait Vertex {
    const VERTEX_SIZE: usize;

    fn configure_vertex_attribute();
}
