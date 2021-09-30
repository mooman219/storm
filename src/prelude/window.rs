/// Configuration settings for the window.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowSettings {
    /// The title of the window.
    pub title: String,
    /// The display mode of the window.
    pub display_mode: DisplayMode,
    /// Vsync mode for the window.
    pub vsync: Vsync,
}

impl Default for WindowSettings {
    fn default() -> WindowSettings {
        WindowSettings {
            title: String::from("Storm Engine"),
            display_mode: DisplayMode::Windowed {
                width: 500,
                height: 500,
                resizable: true,
            },
            vsync: Vsync::Disabled,
        }
    }
}

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
