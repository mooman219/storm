use gl;
use std::os::raw::c_void;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IndiceType {
    UnsignedByte = gl::UNSIGNED_BYTE,
    UnsignedShort = gl::UNSIGNED_SHORT,
    UnsignedInt = gl::UNSIGNED_INT,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum DrawMode {
    Points = gl::POINTS,
    LineStrip = gl::LINE_STRIP,
    LineLoop = gl::LINE_LOOP,
    Lines = gl::LINES,
    LineStripAdjacency = gl::LINE_STRIP_ADJACENCY,
    LinesAdjacency = gl::LINES_ADJACENCY,
    TriangleStrip = gl::TRIANGLE_STRIP,
    TriangleFan = gl::TRIANGLE_FAN,
    Triangles = gl::TRIANGLES,
    TriangleStripAdjacency = gl::TRIANGLE_STRIP_ADJACENCY,
    TrianglesAdjacency = gl::TRIANGLES_ADJACENCY,
    Patches = gl::PATCHES,
}

/// Render primitives from array data with a per-element offset.
///
/// # Arguments
///
/// `mode` - Specifies what kind of primitives to render.
/// `count` - Specifies the number of elements to be rendered.
/// `type` - Specifies the type of the values in indices.
/// `indices` - Specifies a pointer to the location where the indices are stored.
/// `basevertex` - Specifies a constant that should be added to each element of indices when chosing elements from the enabled vertex arrays.
#[inline]
pub fn draw_elements_base_vertex(
    mode: DrawMode,
    count: i32,
    indice_type: IndiceType,
    indices: *const c_void,
    base_vertex: i32,
) {
    unsafe {
        gl::DrawElementsBaseVertex(mode as u32, count, indice_type as u32, indices, base_vertex);
    }
}
