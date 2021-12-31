use crate::graphics::{DisplayMode, Vsync};
use alloc::string::String;

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
