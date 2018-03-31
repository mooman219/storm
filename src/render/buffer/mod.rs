pub mod chunked;
pub mod dynamic;
pub mod fixed;
pub mod geometry;
pub mod immutable;

use render::enums::*;

pub trait RawBuffer<T> {
    fn new(buffer_type: BufferType, capacity: usize) -> Self;

    fn add(&mut self, item: T) -> usize;

    fn remove(&mut self, index: usize);

    fn update(&mut self, index: usize, item: T);

    fn offset_index(&self) -> usize;

    fn len(&self) -> usize;

    fn bind(&self);

    fn sync(&mut self);
}
