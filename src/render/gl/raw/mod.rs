mod buffer;
mod capability;
mod draw;
mod texture;
mod uniform;
mod utility;
mod vertex;

pub use crate::render::gl::raw::buffer::*;
pub use crate::render::gl::raw::capability::*;
pub use crate::render::gl::raw::draw::*;
pub use crate::render::gl::raw::texture::*;
pub use crate::render::gl::raw::uniform::*;
pub use crate::render::gl::raw::utility::*;
pub use crate::render::gl::raw::vertex::*;

/// Set the viewport.
///
/// # Arguments
///
/// `x, y` - Specify the lower left corner of the viewport rectangle, in pixels. The initial value
/// is (0,0). `width, height` - Specify the width and height of the viewport. When a GL context is
/// first attached to a window, width and height are set to the dimensions of that window.
#[cfg(not(macos))]
#[inline]
pub fn viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        gl::Viewport(x, y, width, height);
    }
}

#[cfg(macos)]
#[inline]
pub fn viewport(_x: i32, _y: i32, _width: i32, _height: i32) {}

#[inline(always)]
fn bool_to_enum(value: bool) -> u8 {
    if value {
        gl::TRUE
    } else {
        gl::FALSE
    }
}
