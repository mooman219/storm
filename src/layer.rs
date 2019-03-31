use cgmath::*;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, Eq)]
pub struct LayerReference {
    depth: usize,
    key: usize,
}

impl Ord for LayerReference {
    fn cmp(&self, other: &LayerReference) -> Ordering {
        let mut ordering = self.depth.cmp(&other.depth);
        if ordering == Ordering::Equal {
            ordering = self.key.cmp(&other.key);
        }
        ordering
    }
}

impl PartialOrd for LayerReference {
    fn partial_cmp(&self, other: &LayerReference) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LayerReference {
    fn eq(&self, other: &LayerReference) -> bool {
        self.depth == other.depth && self.key == other.key
    }
}

impl LayerReference {
    pub(crate) fn new(depth: usize, key: usize) -> LayerReference {
        LayerReference { depth: depth, key: key }
    }

    pub(crate) fn depth(&self) -> usize {
        self.depth
    }

    pub(crate) fn key(&self) -> usize {
        self.key
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LayerDescription {
    pub translation: Vector2<f32>,
    pub scale: f32,
    pub visible: bool,
}

impl Default for LayerDescription {
    fn default() -> LayerDescription {
        LayerDescription {
            translation: Vector2::new(0f32, 0f32),
            scale: 1f32,
            visible: true,
        }
    }
}
