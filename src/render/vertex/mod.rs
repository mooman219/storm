pub mod shape;

use gl;
use std::marker::PhantomData;
use std::mem;

pub trait Vertex {
    const VERTEX_SIZE: usize;

    fn configure_vertex_attribute();
}

pub struct VertexArray<T: Vertex> {
    vao: u32,
    phantom: PhantomData<T>,
}

impl<T: Vertex> VertexArray<T> {
    pub fn new() -> VertexArray<T> {
        let mut vao = 0u32;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }
        T::configure_vertex_attribute();
        VertexArray {
            vao: vao,
            phantom: PhantomData,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }
}

impl<T: Vertex> Drop for VertexArray<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao as *const _);
        }
    }
}
