use cgmath::*;
use color;
use glutin;
use glutin::dpi::*;
use glutin::ContextTrait;
use render::manager::*;
use render::raw::*;
use render::texture::*;
use render::vertex::*;
use render::*;
use time::*;
use utility::bounded_spsc;
use utility::control;

pub struct Window {
    inner: glutin::WindowedContext,
}

// Mark the display as send. In some systems, glutin::GlWindow isn't send so we
// make it as such. This might be a problem later, but not today.
unsafe impl Send for Window {}

impl Window {
    pub fn new(window: glutin::WindowedContext) -> Window {
        Window { inner: window }
    }

    /// Initialize the display. The display is bound in the thread we're going
    /// to be making opengl calls in. Behavior is undefined is the display is
    /// bound outside of the thread and usually segfaults.
    fn bind(&self) {
        unsafe {
            self.inner.context().make_current().unwrap();
        }
        load_with(|symbol| self.inner.get_proc_address(symbol) as *const _);
        info!("Render: OpenGL version {}", get_string(StringTarget::Version));
    }

    #[inline]
    fn get_logical_size(&self) -> Vector2<f64> {
        let logical_size = self.inner.get_inner_size().expect("Window no longer exists.");
        Vector2::new(logical_size.width, logical_size.height)
    }

    /// Resizes the window context using logical dimensions (unscaled by
    /// hidpi).
    #[inline]
    fn resize(&self, dimensions: &Vector2<f64>) {
        let dimensions = self.to_physical_size(dimensions);
        self.inner.resize(PhysicalSize::from((dimensions.x, dimensions.y)));
    }

    /// Swaps the buffers in case of double or triple buffering. You should
    /// call this function every time you have finished rendering, or the
    /// image may not be displayed on the screen.
    #[inline]
    fn swap_buffers(&self) {
        self.inner.swap_buffers().expect("Error while swapping buffers.");
    }

    /// Converts logical window dimensions into physical window dimensions.
    #[inline]
    fn to_physical_size(&self, dimensions: &Vector2<f64>) -> Vector2<f64> {
        dimensions * self.inner.get_hidpi_factor()
    }
}

pub struct RenderServer {
    window: Window,
    render_consumer: bounded_spsc::Consumer<Vec<RenderMessage>>,
    render_control: control::Consumer,

    scene: SceneManager,
    texture: TextureManager,
    text: TextManager,
    current_size: Vector2<f64>,
}

impl RenderServer {
    pub fn new(
        window: Window,
        render_consumer: bounded_spsc::Consumer<Vec<RenderMessage>>,
        render_control: control::Consumer,
    ) -> RenderServer {
        // Initialize the display. The display is bound in the thread we're going to be making opengl
        // calls in. Behavior is undefined is the display is bound outside of the thread and usually
        // segfaults.
        window.bind();

        let current_size = window.get_logical_size();
        let mut state = RenderServer {
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
        enable(Capability::Multisample);
        enable(Capability::DepthTest);
        clear_color(0.0, 0.0, 0.2, 1.0);
        depth_func(DepthTest::Less);
        cull_face(CullFace::Back);

        // Default texture/font setup
        state.texture.add(Texture::from_default(color::WHITE, 8, 8));
        state.texture.sync();
        state.text.add_font_path("./src/render/texture/font/unscii-16.ttf");
        state.text.sync();

        state
    }

    pub fn run_forever(&mut self) {
        let mut timer_render = Timer::new("[R] Frame");
        loop {
            match self.render_consumer.try_pop().as_mut() {
                Some(mut messages) => {
                    timer_render.start();
                    self.resize();
                    self.handle_messages(&mut messages);
                    self.scene.draw();
                    self.window.swap_buffers();
                    timer_render.stop();
                },
                None => {
                    self.render_control.wait();
                },
            }
        }
    }

    #[inline]
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

    #[inline]
    fn handle_messages(&mut self, messages: &mut Vec<RenderMessage>) {
        for message in messages.drain(..) {
            match message {
                // Layer
                RenderMessage::LayerCreate { layer, desc } => {
                    self.scene.layer_create(layer, &desc);
                },
                RenderMessage::LayerUpdate { layer, desc } => {
                    self.scene.layer_update(layer, &desc);
                },
                RenderMessage::LayerRemove { layer } => {
                    self.scene.layer_remove(layer);
                },
                RenderMessage::LayerClear { layer } => {
                    self.scene.layer_clear(layer);
                },

                // Sprite
                RenderMessage::SpriteCreate { layer, desc } => {
                    let uv = self.texture.get_uv(&desc.texture);
                    let quad = TextureVertex::new(desc.pos, desc.size, uv, desc.color);
                    self.scene.sprite_create(layer, &quad);
                },
                RenderMessage::SpriteUpdate { layer, sprite, desc } => {
                    let uv = self.texture.get_uv(&desc.texture);
                    let quad = TextureVertex::new(desc.pos, desc.size, uv, desc.color);
                    self.scene.sprite_update(layer, sprite, &quad);
                },
                RenderMessage::SpriteRemove { layer, sprite } => {
                    self.scene.sprite_remove(layer, sprite);
                },

                // Texture
                RenderMessage::TextureLoad { path } => {
                    self.texture.add_path(&path);
                },

                // Text
                RenderMessage::FontLoad { path } => {
                    self.text.add_font_path(&path);
                },

                // Window
                RenderMessage::WindowTitle { title } => {
                    self.window.inner.set_title(title.as_str());
                },
            }
        }

        // Re-sync our managers after handling state changes.
        self.texture.sync();
        self.text.sync();
        self.scene.sync();
    }
}
