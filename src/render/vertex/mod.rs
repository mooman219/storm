mod sprite;

pub use self::sprite::*;

use crate::render::raw::OpenGL;

pub trait VertexDescription {
    const VERTEX_SIZE: usize;

    fn configure_vertex_attribute(gl: &OpenGL);
}
