/// Enumeration for window display options.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DisplayMode {
    /// Normal windowed mode.
    Windowed {
        /// The height of the window.
        width: i32,
        /// The height of the window.
        height: i32,
        /// If the window is resizable.
        resizable: bool,
    },
    /// For "fake" fullscreen that takes the size of the desktop.
    WindowedFullscreen,
    /// For "real" fullscreen with a videomode change.
    Fullscreen,
}

/// Enumeration for all possible vsync settings.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Vsync {
    /// Vsync will be disabled.
    Disabled,
    /// Vsync will be enabled.
    Enabled,
}
