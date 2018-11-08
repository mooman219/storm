use cgmath::*;
use channel::bounded_spsc::Producer;
use render::color::*;
use std::mem;
use utility::indexmap::*;

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

pub enum RenderMessage {
    //
    // Quad
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
    QuadClear {},
    //
    // Texture
    //
    TextureCreate {
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
    //
    // Window
    //
    WindowTitle {
        title: String,
    },
}

// ////////////////////////////////////////////////////////
// Messenger
// ////////////////////////////////////////////////////////

pub struct RenderMessenger {
    render_producer: Producer<RenderFrame>,
    frame: RenderFrame,
    map_rect: IndexMap,
    map_texture: IndexMap,
    last_translation: Vector2<f32>,
    last_scale: f32,
}

impl RenderMessenger {
    pub fn new(render_producer: Producer<RenderFrame>) -> RenderMessenger {
        RenderMessenger {
            render_producer: render_producer,
            frame: RenderFrame::new(),
            map_rect: IndexMap::new(),
            map_texture: IndexMap::new(),
            last_translation: Vector2::zero(),
            last_scale: 1f32,
        }
    }

    // Quad Functions

    pub fn quad_create(&mut self, pos: Vector3<f32>, size: Vector2<f32>, color: Color) -> IndexToken {
        let message = RenderMessage::QuadCreate {
            pos: pos,
            size: size,
            color: color,
        };
        self.frame.messages.push(message);
        self.map_rect.add()
    }

    pub fn quad_update(&mut self, token: IndexToken, pos: Vector3<f32>, size: Vector2<f32>, color: Color) {
        let message = RenderMessage::QuadUpdate {
            id: self.map_rect.get(token),
            pos: pos,
            size: size,
            color: color,
        };
        self.frame.messages.push(message);
    }

    pub fn quad_remove(&mut self, token: IndexToken) {
        let message = RenderMessage::QuadRemove {
            id: self.map_rect.remove(token),
        };
        self.frame.messages.push(message);
    }

    pub fn quad_clear(&mut self) {
        self.map_rect.clear();
        let message = RenderMessage::QuadClear {};
        self.frame.messages.push(message);
    }

    // Texture Functions

    pub fn texture_create(&mut self, path: &str) -> IndexToken {
        let message = RenderMessage::TextureCreate {
            path: String::from(path),
        };
        self.frame.messages.push(message);
        self.map_texture.add()
    }

    // Scene Functions

    pub fn translate(&mut self, translation: Vector2<f32>) {
        if self.last_translation == translation {
            return;
        }
        self.last_translation = translation;
        let message = RenderMessage::Translate { pos: translation };
        self.frame.messages.push(message);
    }

    pub fn scale(&mut self, scale: f32) {
        if self.last_scale == scale {
            return;
        }
        self.last_scale = scale;
        let message = RenderMessage::Scale { factor: scale };
        self.frame.messages.push(message);
    }

    // Window Functions

    pub fn window_title(&mut self, title: &str) {
        let message = RenderMessage::WindowTitle {
            title: String::from(title),
        };
        self.frame.messages.push(message);
    }

    // Utility Functions

    pub fn send(&mut self) {
        let mut frame = RenderFrame::new();
        mem::swap(&mut frame, &mut self.frame);
        self.render_producer.push(frame);
    }
}
