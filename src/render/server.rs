use render::gl::OpenGLState;
use render::message::*;
use render::*;
use time::*;
use utility::bucket_spsc;

pub struct RenderServer {
    render_consumer: bucket_spsc::Consumer<RenderState>,
    state: OpenGLState,
}

impl RenderServer {
    pub fn new(window: Window, render_consumer: bucket_spsc::Consumer<RenderState>) -> RenderServer {
        RenderServer {
            render_consumer: render_consumer,
            state: OpenGLState::new(window),
        }
    }

    pub fn run_forever(&mut self) {
        let mut timer_render = Timer::new("[R] Frame");
        loop {
            self.render_consumer.spin_next();
            timer_render.start();
            self.update();
            self.state.draw();
            timer_render.stop();
        }
    }

    pub fn update(&mut self) {
        let messages = self.render_consumer.get();
        if let Some(texture_atlas) = messages.texture_atlas {
            self.state.upload_texture_atlas(&texture_atlas);
            messages.texture_atlas = None;
        }
        if let Some(font_atlas) = messages.font_atlas {
            self.state.upload_font_atlas(&font_atlas);
            messages.font_atlas = None;
        }
        for message in messages.batch_changes.drain(..) {
            match message {
                BatchMessage::Create {
                    desc,
                } => self.state.batch_create(&desc),
                BatchMessage::Update {
                    index,
                    desc,
                } => self.state.batch_update(index, &desc),
                BatchMessage::Remove {
                    index,
                } => self.state.batch_remove(index),
            }
        }
        let mut index = 0;
        for batch in &mut messages.batches {
            if batch.dirty_sprites {
                self.state.batch_sprite_set(index, &batch.sprites);
                batch.dirty_sprites = false;
            }
            if batch.dirty_strings {
                self.state.batch_string_set(index, &batch.strings);
                batch.dirty_strings = false;
            }
            index += 1;
        }
    }
}
