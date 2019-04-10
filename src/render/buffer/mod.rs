pub mod dynamic;
pub mod geometry;

use render::raw::*;

pub trait RawBuffer<T> {
    fn new(buffer_type: BufferBindingTarget, capacity: usize) -> Self;

    fn add(&mut self, item: T) -> usize;

    fn remove(&mut self, index: usize);

    fn update(&mut self, index: usize, item: T);

    fn clear(&mut self);

    fn len(&self) -> usize;

    fn bind(&self);

    fn sync(&mut self);
}
