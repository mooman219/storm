use bounded_spsc_queue::Producer;
use cgmath::*;
use render::color::*;
use std::mem;
use utility::slotmap::*;

// ////////////////////////////////////////////////////////
// Messages
// ////////////////////////////////////////////////////////

pub struct RenderFrame {
    pub messages: Vec<RenderMessage>,
}

impl RenderFrame {
    pub fn new() -> RenderFrame {
        RenderFrame { messages: Vec::new() }
    }
}

#[repr(u8)]
pub enum RenderMessage {
    //
    // Geometry
    //
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
    //
    // Texture
    //
    CreateTexture {
        path: String,
    },
    //
    // Scene
    //
    Translate {
        pos: Vector2<f32>,
    },
    Scale {
        factor: f32,
    },
}

// ////////////////////////////////////////////////////////
// Messenger
// ////////////////////////////////////////////////////////

pub struct RenderProducer {
    render_producer: Producer<RenderFrame>,
    frame: RenderFrame,
    map_rect: IndexMap,
    map_triangle: IndexMap,
    map_texture: IndexMap,
}

impl RenderProducer {
    pub fn new(render_producer: Producer<RenderFrame>) -> RenderProducer {
        RenderProducer {
            render_producer: render_producer,
            frame: RenderFrame::new(),
            map_rect: IndexMap::new(),
            map_triangle: IndexMap::new(),
            map_texture: IndexMap::new(),
        }
    }

    // Geometry Functions

    pub fn create_rect(&mut self, pos: Vector3<f32>, size: Vector2<f32>, color: Color) -> IndexToken {
        let message = RenderMessage::QuadCreate {
            pos: pos,
            size: size,
            color: color,
        };
        self.frame.messages.push(message);
        self.map_rect.add()
    }

    pub fn update_rect(&mut self, token: &IndexToken, pos: Vector3<f32>, size: Vector2<f32>, color: Color) {
        let message = RenderMessage::QuadUpdate {
            id: self.map_rect.get(token),
            pos: pos,
            size: size,
            color: color,
        };
        self.frame.messages.push(message);
    }

    pub fn remove_rect(&mut self, token: IndexToken) {
        let message = RenderMessage::QuadRemove {
            id: self.map_rect.remove(token),
        };
        self.frame.messages.push(message);
    }

    // Texture Functions

    pub fn create_texture(&mut self, path: &str) -> IndexToken {
        let message = RenderMessage::CreateTexture {
            path: String::from(path),
        };
        self.frame.messages.push(message);
        self.map_texture.add()
    }

    // Scene Functions

    pub fn set_translation(&mut self, translation: Vector2<f32>) {
        let message = RenderMessage::Translate { pos: translation };
        self.frame.messages.push(message);
    }

    pub fn set_scale(&mut self, scale: f32) {
        let message = RenderMessage::Scale { factor: scale };
        self.frame.messages.push(message);
    }

    // Utility Functions

    pub fn send(&mut self) {
        let mut frame = RenderFrame::new();
        mem::swap(&mut frame, &mut self.frame);
        self.render_producer.push(frame);
    }
}
