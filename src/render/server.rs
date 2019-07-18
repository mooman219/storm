use crate::render::gl::OpenGLState;
use crate::render::message::*;
use crate::time::*;
use crate::types::WindowSettings;
use crate::utility::swap_spsc;
use beryllium::SDLToken;

pub struct RenderServer {
    render_consumer: swap_spsc::Consumer<RenderState>,
    state: OpenGLState,
    timer_render: Timer,
}

impl RenderServer {
    pub fn new(
        desc: &WindowSettings,
        sdl: &SDLToken,
        render_consumer: swap_spsc::Consumer<RenderState>,
    ) -> RenderServer {
        RenderServer {
            render_consumer: render_consumer,
            state: OpenGLState::new(desc, sdl),
            timer_render: Timer::new("[R] Frame"),
        }
    }

    pub fn tick(&mut self) {
        if self.render_consumer.try_next() {
            self.timer_render.start();
            self.update();
            self.state.draw();
            self.timer_render.stop();
        }
    }

    fn update(&mut self) {
        let messages = self.render_consumer.get();
        if let Some(texture_atlas) = messages.texture_atlas.take() {
            self.state.upload_texture_atlas(&texture_atlas);
        }
        if let Some(font_atlas) = messages.font_atlas.take() {
            self.state.upload_font_atlas(&font_atlas);
        }
        if let Some(title) = messages.window.title.take() {
            self.state.window_title(&title);
        }
        if let Some(clear_color) = messages.window.clear_color.take() {
            self.state.window_clear_color(clear_color);
        }
        if let Some(display_mode) = messages.window.display_mode.take() {
            self.state.window_display_mode(display_mode);
        }
        if let Some(vsync) = messages.window.vsync.take() {
            self.state.window_vsync(vsync);
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
