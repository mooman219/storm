use crate::render::gl::raw::{resource, OpenGL};
use crate::render::gl::vertex::*;
use std::marker::PhantomData;

pub struct VertexArray<T: VertexDescription> {
    gl: OpenGL,
    vao: resource::VertexArray,
    phantom: PhantomData<T>,
}

impl<T: VertexDescription> VertexArray<T> {
    pub fn new(gl: OpenGL) -> VertexArray<T> {
        let vao = gl.create_vertex_array();
        gl.bind_vertex_array(Some(vao));
        T::configure_vertex_attribute(&gl);
        VertexArray {
            gl,
            vao,
            phantom: PhantomData,
        }
    }

    pub fn bind(&self) {
        self.gl.bind_vertex_array(Some(self.vao));
    }
}

impl<'a, T: VertexDescription> Drop for VertexArray<T> {
    fn drop(&mut self) {
        warn!("Dropping vertex array.");
        self.gl.delete_vertex_array(self.vao);
    }
}
