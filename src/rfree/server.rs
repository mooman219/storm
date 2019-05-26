use cgmath::*;
use glutin;
use glutin::dpi::*;
use glutin::ContextTrait;
use render::manager::*;
use render::raw::*;
use render::texture::*;
use render::vertex::*;
use render::*;
use time::*;
use utility::bucket_spsc;
use utility::control;

pub struct Window {
    inner: glutin::WindowedContext,
}

// Mark the display as send. In some systems, glutin::GlWindow isn't send so we
// make it as such. This might be a problem later, but not today.
unsafe impl Send for Window {}

impl Window {
    pub fn new(window: glutin::WindowedContext) -> Window {
        Window {
            inner: window,
        }
    }

    /// Initialize the display. The display is bound in the thread we're going
    /// to be making opengl calls in. Behavior is undefined is the display is
    /// bound outside of the thread and usually segfaults.
    pub fn bind(&self) {
        unsafe {
            self.inner.context().make_current().unwrap();
        }
        load_with(|symbol| self.inner.get_proc_address(symbol) as *const _);
        info!("Render: OpenGL version {}", get_string(StringTarget::Version));
    }

    #[inline]
    pub fn get_logical_size(&self) -> Vector2<f64> {
        let logical_size = self.inner.get_inner_size().expect("Window no longer exists.");
        Vector2::new(logical_size.width, logical_size.height)
    }

    /// Resizes the window context using logical dimensions (unscaled by
    /// hidpi).
    #[inline]
    pub fn resize(&self, dimensions: &Vector2<f64>) {
        let dimensions = self.to_physical_size(dimensions);
        self.inner.resize(PhysicalSize::from((dimensions.x, dimensions.y)));
    }

    /// Swaps the buffers in case of double or triple buffering. You should
    /// call this function every time you have finished rendering, or the
    /// image may not be displayed on the screen.
    #[inline]
    pub fn swap_buffers(&self) {
        self.inner.swap_buffers().expect("Error while swapping buffers.");
    }

    /// Converts logical window dimensions into physical window dimensions.
    #[inline]
    pub fn to_physical_size(&self, dimensions: &Vector2<f64>) -> Vector2<f64> {
        dimensions * self.inner.get_hidpi_factor()
    }
}

pub struct RenderServer {
    window: Window,
    render_consumer: bucket_spsc::Consumer<RenderState>,
    render_control: control::Consumer,

    scene: SceneManager,
    texture: TextureManager,
    text: TextManager,
    current_size: Vector2<f64>,
}

impl RenderServer {
    pub fn new(
        window: Window,
        render_consumer: bucket_spsc::Consumer<RenderState>,
        render_control: control::Consumer,
    ) -> RenderServer {
        // Initialize the display. The display is bound in the thread we're going to be making
        // opengl calls in. Behavior is undefined is the display is bound outside of the
        // thread and usually segfaults.
        window.bind();

        let current_size = window.get_logical_size();
        let server = RenderServer {
            window: window,
            render_consumer: render_consumer,
            render_control: render_control,
            scene: SceneManager::new(&current_size),
            texture: TextureManager::new(),
            text: TextManager::new(),
            current_size: current_size,
        };

        // Setup cabilities.
        enable(Capability::CullFace);
        enable(Capability::Blend);
        enable(Capability::DepthTest);
        clear_color(1.0, 1.0, 1.0, 1.0);
        depth_func(DepthTest::Less);
        blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        cull_face(CullFace::Back);

        server
    }

    pub fn run_forever(&mut self) {
        let mut timer_render = Timer::new("[R] Frame");
        loop {
            if self.render_consumer.try_next() {
                timer_render.start();
                self.resize();
                self.handle_messages();
                self.scene.draw();
                self.window.swap_buffers();
                timer_render.stop();
            } else {
                self.render_control.wait();
            }
        }
    }

    fn resize(&mut self) {
        let new_size = self.window.get_logical_size();
        if self.current_size != new_size {
            self.current_size = new_size;
            self.scene.resize(&new_size);
            self.window.resize(&new_size);

            // The viewport function takes in physical dimensions.
            let dimensions = self.window.to_physical_size(&new_size);
            viewport(0, 0, dimensions.x as i32, dimensions.y as i32);
        }
    }

    fn handle_messages(&mut self) {
        let state = self.render_consumer.get();
        for message in state.textures.drain(..) {
            match message {
                TextureMessage::Load {
                    path,
                } => self.texture.add_path(&path),
            }
        }
        for message in state.fonts.drain(..) {
            match message {
                FontMessage::Load {
                    path,
                } => self.text.add_font_path(&path),
            }
        }
        for message in state.window.drain(..) {
            match message {
                WindowMessage::Title {
                    title,
                } => self.window.inner.set_title(title.as_str()),
            }
        }
        for message in state.batch_changes.drain(..) {
            match message {
                BatchMessage::Create => self.scene.batch_create(),
                BatchMessage::Remove {
                    index,
                } => self.scene.batch_remove(index),
            }
        }
        let mut batch_index = 0;
        for batch in &mut state.batches {
            if batch.dirty_desc {
                batch.dirty_desc = false;
                self.scene.batch_update(batch_index, &batch.desc);
            }
            if batch.dirty_sprites {
                batch.dirty_sprites = false;
            }
            if batch.dirty_strings {
                batch.dirty_strings = false;
            }
            batch_index += 1;
        }
        // for message in self.render_consumer.get().drain(..) {
        //     // TODO: Also trim the buffer
        //     match message {
        //         // Layer
        //         RenderMessage::LayerCreate {
        //             layer,
        //             desc,
        //         } => {
        //             self.scene.layer_create(layer, &desc);
        //         },
        //         RenderMessage::LayerUpdate {
        //             layer,
        //             desc,
        //         } => {
        //             self.scene.layer_update(layer, &desc);
        //         },
        //         RenderMessage::LayerRemove {
        //             layer,
        //         } => {
        //             self.scene.layer_remove(layer);
        //         },
        //         RenderMessage::LayerClear {
        //             layer,
        //         } => {
        //             self.scene.layer_clear(layer);
        //         },

        //         // Sprite
        //         RenderMessage::SpriteCreate {
        //             layer,
        //             desc,
        //         } => {
        //             let uv = self.texture.get_uv(&desc.texture);
        //             let quad = TextureVertex::new(desc.pos, desc.size, uv, desc.color, desc.rotation);
        //             self.scene.sprite_create(layer, &quad);
        //         },
        //         RenderMessage::SpriteUpdate {
        //             layer,
        //             sprite,
        //             desc,
        //         } => {
        //             let uv = self.texture.get_uv(&desc.texture);
        //             let quad = TextureVertex::new(desc.pos, desc.size, uv, desc.color, desc.rotation);
        //             self.scene.sprite_update(layer, sprite, &quad);
        //         },
        //         RenderMessage::SpriteRemove {
        //             layer,
        //             sprite,
        //         } => {
        //             self.scene.sprite_remove(layer, sprite);
        //         },

        //         // Texture
        //         RenderMessage::TextureLoad {
        //             path,
        //         } => {
        //             self.texture.add_path(&path);
        //         },

        //         // Text
        //         RenderMessage::FontLoad {
        //             path,
        //         } => {
        //             self.text.add_font_path(&path);
        //         },
        //         RenderMessage::TextCreate {
        //             layer_index,
        //             text,
        //             desc,
        //         } => {
        //             self.scene.text_create(layer_index, self.text.rasterize(&text, &desc));
        //         },
        //         RenderMessage::TextUpdate {
        //             layer_index,
        //             text_index,
        //             text,
        //             desc,
        //         } => {
        //             self.scene.text_update(layer_index, text_index, self.text.rasterize(&text, &desc));
        //         },
        //         RenderMessage::TextRemove {
        //             layer_index,
        //             text_index,
        //         } => {
        //             self.scene.text_remove(layer_index, text_index);
        //         },

        //         // Window
        //         RenderMessage::WindowTitle {
        //             title,
        //         } => {
        //             self.window.inner.set_title(title.as_str());
        //         },
        //     }
        // }

        // Re-sync our managers after handling state changes.
        self.texture.sync();
        self.text.sync();
        self.scene.sync();
    }
}
