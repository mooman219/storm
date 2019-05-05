use render::buffer::geometry::*;
use render::vertex::*;

pub struct GroupedBuffer<T: Vertex + Copy> {
    buffer: GeometryBuffer<T>,
    groups: Vec<Vec<T>>,
    dirty: bool,
}

impl<T: Vertex + Copy> GroupedBuffer<T> {
    pub fn new() -> GroupedBuffer<T> {
        GroupedBuffer {
            buffer: GeometryBuffer::new(),
            groups: Vec::new(),
            dirty: false,
        }
    }

    #[inline]
    pub fn push(&mut self, element: Vec<T>) {
        self.dirty = true;
        self.groups.push(element);
    }

    #[inline]
    pub fn swap_remove(&mut self, index: usize) {
        self.dirty = true;
        self.groups.swap_remove(index);
    }

    #[inline]
    pub fn update(&mut self, index: usize, element: Vec<T>) {
        self.dirty = true;
        self.groups[index] = element;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.dirty = true;
        self.groups.clear();
    }

    #[inline]
    pub fn groups(&self) -> usize {
        self.groups.len()
    }

    #[inline]
    pub fn sync(&mut self) {
        if self.dirty {
            self.dirty = false;
            self.buffer.set_flattened(&self.groups);
        }
        self.buffer.sync();
    }

    #[inline]
    pub fn draw(&mut self) {
        self.buffer.draw();
    }
}
