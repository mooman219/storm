pub mod chunked;
pub mod dynamic;
pub mod geometry;
pub mod immutable;

pub trait RawBuffer<T> {
    fn add(&mut self, item: T) -> usize;

    fn remove(&mut self, index: usize);

    fn update(&mut self, index: usize, item: T);

    fn offset_index(&self) -> usize;

    fn len(&self) -> usize;

    fn bind(&self);

    fn sync(&mut self);
}
