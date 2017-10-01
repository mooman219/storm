use gl;
use std::mem;

use render::enums::buffer_type::*;
use render::shape::*;
use render::vertex::*;

use render::buffer::*;

pub struct ShapeBuffer<T: Shape> {
    // ebo: u32,
    vertex_buffer: Buffer<T::ShapeType>,
    vao: u32,
}

impl<T: Shape> ShapeBuffer<T> {
    const SHAPE_SIZE: usize = mem::size_of::<T::ShapeType>();
    const VERTEX_SIZE: usize = mem::size_of::<T::VertexType>();
    const VERTEX_COUNT: usize = ShapeBuffer::<T>::SHAPE_SIZE / ShapeBuffer::<T>::VERTEX_SIZE;

    pub fn new() -> ShapeBuffer<T> {
        unsafe {
            // Element Buffer Object
            // let mut ebo = mem::uninitialized::<u32>();
            // gl::GenBuffers(1, &mut ebo);
            // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            // gl::BufferData(
            //     gl::ELEMENT_ARRAY_BUFFER,
            //     ShapeBuffer::<T>::DEFAULT_SIZE as isize,
            //     items.as_ptr() as *const _,
            //     gl::DYNAMIC_DRAW,
            // );
            // Vertex Buffer Object
            let vertex_buffer = Buffer::new(BufferType::ArrayBuffer);
            // Vertex Array Object
            let mut vao = mem::uninitialized::<u32>();
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            T::VertexType::configure_vertex_attribute();
            // Return
            ShapeBuffer {
                // ebo: ebo,
                vertex_buffer: vertex_buffer,
                vao: vao,
            }
        }
    }

    pub fn add(&mut self, element: T::ShapeType) {
        self.vertex_buffer.add(element);
        self.vertex_buffer.sync();
    }

    pub fn draw(&self) {
        unsafe {
            let vertices = self.vertex_buffer.len() * ShapeBuffer::<T>::VERTEX_COUNT;
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(<T>::DRAW_MODE.to_gl_enum(), 0, vertices as i32);
        }
    }
}

impl<T: Shape> Drop for ShapeBuffer<T> {
    fn drop(&mut self) {
        println!("Dropping ShapeBuffer");
        // unsafe {
        //     gl::DeleteVertexArrays(1, self.vao as *const _);
        //     gl::DeleteBuffers(1, self.vbo as *const _);
        // }
        println!("Dropped ShapeBuffer");
    }
}
