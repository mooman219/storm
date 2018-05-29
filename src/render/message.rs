use bounded_spsc_queue::Producer;
use cgmath::*;
use render::color::*;
use std::mem;
use utility::slotmap::*;

// ////////////////////////////////////////////////////////
// Messages
// ////////////////////////////////////////////////////////

pub struct RenderFrame {
    pub geometry: Vec<GeometryMessage>,
    pub translation: Option<Vector2<f32>>,
    pub scale: Option<f32>,
}

impl RenderFrame {
    pub fn new() -> RenderFrame {
        RenderFrame {
            geometry: Vec::new(),
            translation: None,
            scale: None,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum GeometryMessage {
    QuadCreate {
        pos: Vector3<f32>,
        size: Vector2<f32>,
        color: Color,
    },
    QuadUpdate {
        id: usize,
        pos: Vector3<f32>,
        size: Vector2<f32>,
        color: Color,
    },
    QuadRemove {
        id: usize,
    },
}

// ////////////////////////////////////////////////////////
// Enums
// ////////////////////////////////////////////////////////

// ////////////////////////////////////////////////////////
// Messenger
// ////////////////////////////////////////////////////////

pub struct RenderProducer {
    render_producer: Producer<RenderFrame>,
    frame: RenderFrame,
    map_rect: IndexMap,
    map_triangle: IndexMap,
}

impl RenderProducer {
    pub fn new(render_producer: Producer<RenderFrame>) -> RenderProducer {
        RenderProducer {
            render_producer: render_producer,
            frame: RenderFrame::new(),
            map_rect: IndexMap::new(),
            map_triangle: IndexMap::new(),
        }
    }

    // Geometry Functions

    pub fn create_rect(&mut self, pos: Vector3<f32>, size: Vector2<f32>, color: Color) -> IndexToken {
        let message = GeometryMessage::QuadCreate {
            pos: pos,
            size: size,
            color: color,
        };
        self.frame.geometry.push(message);
        self.map_rect.add()
    }

    pub fn update_rect(&mut self, token: &IndexToken, pos: Vector3<f32>, size: Vector2<f32>, color: Color) {
        let message = GeometryMessage::QuadUpdate {
            id: self.map_rect.get(token),
            pos: pos,
            size: size,
            color: color,
        };
        self.frame.geometry.push(message);
    }

    pub fn remove_rect(&mut self, token: IndexToken) {
        let message = GeometryMessage::QuadRemove {
            id: self.map_rect.remove(token),
        };
        self.frame.geometry.push(message);
    }

    // Scene Functions

    pub fn set_translation(&mut self, translation: Vector2<f32>) {
        self.frame.translation = Some(translation);
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.frame.scale = Some(scale);
    }

    // Utility Functions

    pub fn send(&mut self) {
        let mut frame = RenderFrame::new();
        mem::swap(&mut frame, &mut self.frame);
        self.render_producer.push(frame);
    }
}
