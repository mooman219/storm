mod buffer;
mod capability;
mod draw;
mod texture;
mod uniform;
mod utility;
mod vertex;

pub use render::raw::buffer::*;
pub use render::raw::capability::*;
pub use render::raw::draw::*;
pub use render::raw::texture::*;
pub use render::raw::uniform::*;
pub use render::raw::utility::*;
pub use render::raw::vertex::*;

use gl;

/// Set the viewport.
///
/// # Arguments
///
/// `x, y` - Specify the lower left corner of the viewport rectangle, in pixels. The initial value
/// is (0,0). `width, height` - Specify the width and height of the viewport. When a GL context is
/// first attached to a window, width and height are set to the dimensions of that window.
#[inline]
pub fn viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        gl::Viewport(x, y, width, height);
    }
}

#[inline(always)]
fn bool_to_enum(value: bool) -> u8 {
    if value {
        gl::TRUE
    } else {
        gl::FALSE
    }
}
