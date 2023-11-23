pub struct FrameBuffer {}

impl FrameBuffer {
    /// A manually sized framebuffer.
    fn from_size() -> FrameBuffer {
        FrameBuffer {}
    }

    /// An automatically resizing framebuffer that always matches the screen size.
    fn from_screen() -> FrameBuffer {
        FrameBuffer {}
    }
}
