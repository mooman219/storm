#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Capability {
    Blend = gl::BLEND,
    ColorLogicOp = gl::COLOR_LOGIC_OP,
    CullFace = gl::CULL_FACE,
    DebugOutput = gl::DEBUG_OUTPUT,
    DebugOutputSynchronous = gl::DEBUG_OUTPUT_SYNCHRONOUS,
    DepthClamp = gl::DEPTH_CLAMP,
    DepthTest = gl::DEPTH_TEST,
    Dither = gl::DITHER,
    FramebufferSrgb = gl::FRAMEBUFFER_SRGB,
    LineSmooth = gl::LINE_SMOOTH,
    Multisample = gl::MULTISAMPLE,
    PolygonOffsetFill = gl::POLYGON_OFFSET_FILL,
    PolygonOffsetLine = gl::POLYGON_OFFSET_LINE,
    PolygonOffsetPoint = gl::POLYGON_OFFSET_POINT,
    PolygonSmooth = gl::POLYGON_SMOOTH,
    PrimitiveRestart = gl::PRIMITIVE_RESTART,
    PrimitiveRestartFixedIndex = gl::PRIMITIVE_RESTART_FIXED_INDEX,
    RasterizerDiscard = gl::RASTERIZER_DISCARD,
    SampleAlphaToCoverage = gl::SAMPLE_ALPHA_TO_COVERAGE,
    SampleAlphaToOne = gl::SAMPLE_ALPHA_TO_ONE,
    SampleCoverage = gl::SAMPLE_COVERAGE,
    SampleShading = gl::SAMPLE_SHADING,
    SampleMask = gl::SAMPLE_MASK,
    ScissorTest = gl::SCISSOR_TEST,
    StencilTest = gl::STENCIL_TEST,
    TextureCubeMapSeamless = gl::TEXTURE_CUBE_MAP_SEAMLESS,
    ProgramPointSize = gl::PROGRAM_POINT_SIZE,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum DepthTest {
    Always = gl::ALWAYS,
    Never = gl::NEVER,
    Equal = gl::EQUAL,
    NotEqual = gl::NOTEQUAL,
    Less = gl::LESS,
    LessEqual = gl::LEQUAL,
    Greater = gl::GREATER,
    GreaterEqual = gl::GEQUAL,
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod ClearBit {
    pub const ColorBuffer: u32 = 0x0000_4000; // gl::COLOR_BUFFER_BIT;
    pub const DepthBuffer: u32 = 0x0000_0100; // gl::DEPTH_BUFFER_BIT;
    pub const StencilBuffer: u32 = 0x0000_0400; // gl::STENCIL_BUFFER_BIT;
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CullFace {
    Front = gl::FRONT,
    Back = gl::BACK,
    FrontBack = gl::FRONT_AND_BACK,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum BlendFactor {
    Zero = gl::ZERO,
    One = gl::ONE,
    SrcColor = gl::SRC_COLOR,
    OneMinusSrcColor = gl::ONE_MINUS_SRC_COLOR,
    DstColor = gl::DST_COLOR,
    OneMinusDstColor = gl::ONE_MINUS_DST_COLOR,
    SrcAlpha = gl::SRC_ALPHA,
    OneMinusSrcAlpha = gl::ONE_MINUS_SRC_ALPHA,
    DstAlpha = gl::DST_ALPHA,
    OneMinusDstAlpha = gl::ONE_MINUS_DST_ALPHA,
    ConstantColor = gl::CONSTANT_COLOR,
    OneMinusConstantColor = gl::ONE_MINUS_CONSTANT_COLOR,
    ConstantAlpha = gl::CONSTANT_ALPHA,
    OneMinusConstantAlpha = gl::ONE_MINUS_CONSTANT_ALPHA,
    SrcAlphaSaturate = gl::SRC_ALPHA_SATURATE,
    Src1Color = gl::SRC1_COLOR,
    OneMinusSrc1Color = gl::ONE_MINUS_SRC1_COLOR,
    Src1Alpha = gl::SRC1_ALPHA,
    OneMinusSrc1Alpha = gl::ONE_MINUS_SRC1_ALPHA,
}

/// Specify the value used for depth buffer comparisons.
///
/// # Arguments
///
/// `func` - Specifies the depth comparison function. Symbolic constants GL_NEVER, GL_LESS,
/// GL_EQUAL, GL_LEQUAL, GL_GREATER, GL_NOTEQUAL, GL_GEQUAL, and GL_ALWAYS are accepted. The initial
/// value is GL_LESS.
#[inline]
pub fn depth_func(func: DepthTest) {
    unsafe {
        gl::DepthFunc(func as u32);
    }
}

/// Specify whether front- or back-facing facets can be culled.
///
/// # Arguments
///
/// `mode` - Specifies whether front- or back-facing facets are candidates for culling. Symbolic
/// constants GL_FRONT, GL_BACK, and GL_FRONT_AND_BACK are accepted. The initial value is GL_BACK.
#[inline]
pub fn cull_face(mode: CullFace) {
    unsafe {
        gl::CullFace(mode as u32);
    }
}

/// Enable server-side GL capabilities.
///
/// # Arguments
///
/// `cap` - Specifies a symbolic constant indicating a GL capability.
#[inline]
pub fn enable(cap: Capability) {
    unsafe {
        gl::Enable(cap as u32);
    }
}

/// Disable server-side GL capabilities.
///
/// # Arguments
///
/// `cap` - Specifies a symbolic constant indicating a GL capability.
#[inline]
pub fn disable(cap: Capability) {
    unsafe {
        gl::Disable(cap as u32);
    }
}

/// Specify the clear value for the depth buffer.
///
/// # Arguments
///
/// `depth` - Specifies the depth value used when the depth buffer is cleared. The initial value is
/// 1.
#[inline]
pub fn clear_depth_f(depth: f32) {
    unsafe {
        gl::ClearDepthf(depth);
    }
}

/// Clear buffers to preset values.
///
/// # Arguments
///
/// `mask` - Bitwise OR of masks that indicate the buffers to be cleared. The three masks are
/// GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, and GL_STENCIL_BUFFER_BIT.
#[inline]
pub fn clear(mask: u32) {
    unsafe {
        gl::Clear(mask);
    }
}

/// Specify clear values for the color buffers.
///
/// # Arguments
///
/// `red, green, blue, alpha` - Specify the red, green, blue, and alpha values used when the color
/// buffers are cleared. The initial values are all 0.
#[inline]
pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        gl::ClearColor(red, green, blue, alpha);
    }
}

/// Specify pixel arithmetic.
///
/// # Arguments
///
/// `sfactor` - Specifies how the red, green, blue, and alpha source blending factors are computed.
/// `dfactor` - Specifies how the red, green, blue, and alpha destination blending factors are
/// computed.
#[inline]
pub fn blend_func(sfactor: BlendFactor, dfactor: BlendFactor) {
    unsafe {
        gl::BlendFunc(sfactor as u32, dfactor as u32);
    }
}
