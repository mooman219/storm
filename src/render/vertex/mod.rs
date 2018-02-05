pub mod shape;

use std::mem;

pub trait Vertex {
    const VERTEX_SIZE: usize;

    fn configure_vertex_attribute();
}
