use crate::render::gl::raw::*;
use crate::render::gl::vertex::*;
use std::marker::PhantomData;

pub struct VertexArray<T: VertexDescription> {
    vao: u32,
    phantom: PhantomData<T>,
}

impl<T: VertexDescription> VertexArray<T> {
    pub fn new() -> VertexArray<T> {
        let vao = gen_vertex_array();
        bind_vertex_array(vao);
        T::configure_vertex_attribute();
        VertexArray {
            vao,
            phantom: PhantomData,
        }
    }

    pub fn bind(&self) {
        bind_vertex_array(self.vao);
    }
}

impl<T: VertexDescription> Drop for VertexArray<T> {
    fn drop(&mut self) {
        delete_vertex_array(self.vao);
    }
}
