mod color;
mod texture;
mod vertex_array;

pub use render::vertex::color::*;
pub use render::vertex::texture::*;
pub use render::vertex::vertex_array::*;

pub trait Vertex {
    const VERTEX_SIZE: usize;

    fn configure_vertex_attribute();
}
