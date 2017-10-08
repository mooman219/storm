pub mod pos2;

pub trait Vertex {
    const VERTEX_SIZE: usize;

    fn configure_vertex_attribute();
}
