use glow::HasContext;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum BufferBindingTarget {
    ArrayBuffer = glow::ARRAY_BUFFER,
    AtomicCounterBuffer = glow::ATOMIC_COUNTER_BUFFER,
    CopyReadBuffer = glow::COPY_READ_BUFFER,
    CopyWriteBuffer = glow::COPY_WRITE_BUFFER,
    DispatchIndirectBuffer = glow::DISPATCH_INDIRECT_BUFFER,
    DrawIndirectBuffer = glow::DRAW_INDIRECT_BUFFER,
    ElementArrayBuffer = glow::ELEMENT_ARRAY_BUFFER,
    PixelPackBuffer = glow::PIXEL_PACK_BUFFER,
    PixelUnpackBuffer = glow::PIXEL_UNPACK_BUFFER,
    QueryBuffer = glow::QUERY_BUFFER,
    ShaderStorageBuffer = glow::SHADER_STORAGE_BUFFER,
    TextureBuffer = glow::TEXTURE_BUFFER,
    TransformFeedbackBuffer = glow::TRANSFORM_FEEDBACK_BUFFER,
    UniformBuffer = glow::UNIFORM_BUFFER,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum BufferUsage {
    StreamDraw = glow::STREAM_DRAW,
    StreamRead = glow::STREAM_READ,
    StreamCopy = glow::STREAM_COPY,
    StaticDraw = glow::STATIC_DRAW,
    StaticRead = glow::STATIC_READ,
    StaticCopy = glow::STATIC_COPY,
    DynamicDraw = glow::DYNAMIC_DRAW,
    DynamicRead = glow::DYNAMIC_READ,
    DynamicCopy = glow::DYNAMIC_COPY,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Capability {
    Blend = glow::BLEND,
    ColorLogicOp = glow::COLOR_LOGIC_OP,
    CullFace = glow::CULL_FACE,
    DebugOutput = glow::DEBUG_OUTPUT,
    DebugOutputSynchronous = glow::DEBUG_OUTPUT_SYNCHRONOUS,
    DepthClamp = glow::DEPTH_CLAMP,
    DepthTest = glow::DEPTH_TEST,
    Dither = glow::DITHER,
    FramebufferSrgb = glow::FRAMEBUFFER_SRGB,
    LineSmooth = glow::LINE_SMOOTH,
    Multisample = glow::MULTISAMPLE,
    PolygonOffsetFill = glow::POLYGON_OFFSET_FILL,
    PolygonOffsetLine = glow::POLYGON_OFFSET_LINE,
    PolygonOffsetPoint = glow::POLYGON_OFFSET_POINT,
    PolygonSmooth = glow::POLYGON_SMOOTH,
    PrimitiveRestart = glow::PRIMITIVE_RESTART,
    PrimitiveRestartFixedIndex = glow::PRIMITIVE_RESTART_FIXED_INDEX,
    RasterizerDiscard = glow::RASTERIZER_DISCARD,
    SampleAlphaToCoverage = glow::SAMPLE_ALPHA_TO_COVERAGE,
    SampleAlphaToOne = glow::SAMPLE_ALPHA_TO_ONE,
    SampleCoverage = glow::SAMPLE_COVERAGE,
    SampleShading = glow::SAMPLE_SHADING,
    SampleMask = glow::SAMPLE_MASK,
    ScissorTest = glow::SCISSOR_TEST,
    StencilTest = glow::STENCIL_TEST,
    TextureCubeMapSeamless = glow::TEXTURE_CUBE_MAP_SEAMLESS,
    ProgramPointSize = glow::PROGRAM_POINT_SIZE,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum DepthTest {
    Always = glow::ALWAYS,
    Never = glow::NEVER,
    Equal = glow::EQUAL,
    NotEqual = glow::NOTEQUAL,
    Less = glow::LESS,
    LessEqual = glow::LEQUAL,
    Greater = glow::GREATER,
    GreaterEqual = glow::GEQUAL,
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod ClearBit {
    pub const ColorBuffer: u32 = 0x0000_4000; // glow::COLOR_BUFFER_BIT;
    pub const DepthBuffer: u32 = 0x0000_0100; // glow::DEPTH_BUFFER_BIT;
    pub const StencilBuffer: u32 = 0x0000_0400; // glow::STENCIL_BUFFER_BIT;
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CullFace {
    Front = glow::FRONT,
    Back = glow::BACK,
    FrontBack = glow::FRONT_AND_BACK,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum BlendFactor {
    Zero = glow::ZERO,
    One = glow::ONE,
    SrcColor = glow::SRC_COLOR,
    OneMinusSrcColor = glow::ONE_MINUS_SRC_COLOR,
    DstColor = glow::DST_COLOR,
    OneMinusDstColor = glow::ONE_MINUS_DST_COLOR,
    SrcAlpha = glow::SRC_ALPHA,
    OneMinusSrcAlpha = glow::ONE_MINUS_SRC_ALPHA,
    DstAlpha = glow::DST_ALPHA,
    OneMinusDstAlpha = glow::ONE_MINUS_DST_ALPHA,
    ConstantColor = glow::CONSTANT_COLOR,
    OneMinusConstantColor = glow::ONE_MINUS_CONSTANT_COLOR,
    ConstantAlpha = glow::CONSTANT_ALPHA,
    OneMinusConstantAlpha = glow::ONE_MINUS_CONSTANT_ALPHA,
    SrcAlphaSaturate = glow::SRC_ALPHA_SATURATE,
    Src1Color = glow::SRC1_COLOR,
    OneMinusSrc1Color = glow::ONE_MINUS_SRC1_COLOR,
    Src1Alpha = glow::SRC1_ALPHA,
    OneMinusSrc1Alpha = glow::ONE_MINUS_SRC1_ALPHA,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureUnit {
    Atlas = glow::TEXTURE0,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureLoadTarget {
    Texture2D = glow::TEXTURE_2D,
    ProxyTexture2D = glow::PROXY_TEXTURE_2D,
    Texture1DArray = glow::TEXTURE_1D_ARRAY,
    ProxyTexture1DArray = glow::PROXY_TEXTURE_1D_ARRAY,
    TextureRectangle = glow::TEXTURE_RECTANGLE,
    ProxyTextureRectangle = glow::PROXY_TEXTURE_RECTANGLE,
    TextureCubeMapPositiveX = glow::TEXTURE_CUBE_MAP_POSITIVE_X,
    TextureCubeMapNegativeX = glow::TEXTURE_CUBE_MAP_NEGATIVE_X,
    TextureCubeMapPositiveY = glow::TEXTURE_CUBE_MAP_POSITIVE_Y,
    TextureCubeMapNegativeY = glow::TEXTURE_CUBE_MAP_NEGATIVE_Y,
    TextureCubeMapPositiveZ = glow::TEXTURE_CUBE_MAP_POSITIVE_Z,
    TextureCubeMapNegativeZ = glow::TEXTURE_CUBE_MAP_NEGATIVE_Z,
    ProxyTextureCubeMap = glow::PROXY_TEXTURE_CUBE_MAP,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureBindingTarget {
    TextureBuffer = glow::TEXTURE_BUFFER,
    Texture1D = glow::TEXTURE_1D,
    Texture1DArray = glow::TEXTURE_1D_ARRAY,
    Texture2D = glow::TEXTURE_2D,
    Texture2DArray = glow::TEXTURE_2D_ARRAY,
    Texture2DMultisample = glow::TEXTURE_2D_MULTISAMPLE,
    Texture2DMultisampleArray = glow::TEXTURE_2D_MULTISAMPLE_ARRAY,
    Texture3D = glow::TEXTURE_3D,
    TextureCubeMap = glow::TEXTURE_CUBE_MAP,
    TextureCubeMapArray = glow::TEXTURE_CUBE_MAP_ARRAY,
    TextureRectangle = glow::TEXTURE_RECTANGLE,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureParameterTarget {
    Texture1D = glow::TEXTURE_1D,
    Texture1DArray = glow::TEXTURE_1D_ARRAY,
    Texture2D = glow::TEXTURE_2D,
    Texture2DArray = glow::TEXTURE_2D_ARRAY,
    Texture2DMultisample = glow::TEXTURE_2D_MULTISAMPLE,
    Texture2DMultisampleArray = glow::TEXTURE_2D_MULTISAMPLE_ARRAY,
    Texture3D = glow::TEXTURE_3D,
    TextureCubeMap = glow::TEXTURE_CUBE_MAP,
    TextureCubeMapArray = glow::TEXTURE_CUBE_MAP_ARRAY,
    TextureRectangle = glow::TEXTURE_RECTANGLE,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum PixelFormat {
    RED = glow::RED,
    RG = glow::RG,
    RGB = glow::RGB,
    BGR = glow::BGR,
    RGBA = glow::RGBA,
    BGRA = glow::BGRA,
    RedInteger = glow::RED_INTEGER,
    RGInteger = glow::RG_INTEGER,
    RGBInteger = glow::RGB_INTEGER,
    BGRInteger = glow::BGR_INTEGER,
    RGBAInteger = glow::RGBA_INTEGER,
    BGRAInteger = glow::BGRA_INTEGER,
    StencilIndex = glow::STENCIL_INDEX,
    DepthComponent = glow::DEPTH_COMPONENT,
    DepthStencil = glow::DEPTH_STENCIL,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum PixelInternalFormat {
    DepthComponent = glow::DEPTH_COMPONENT,
    DepthStencil = glow::DEPTH_STENCIL,
    Red = glow::RED,
    RG = glow::RG,
    RGB = glow::RGB,
    RGBA = glow::RGBA,
    R8 = glow::R8,
    R8Snorm = glow::R8_SNORM,
    R16 = glow::R16,
    R16Snorm = glow::R16_SNORM,
    RG8 = glow::RG8,
    RG8Snorm = glow::RG8_SNORM,
    RG16 = glow::RG16,
    RG16Snorm = glow::RG16_SNORM,
    R3G3B2 = glow::R3_G3_B2,
    RGB4 = glow::RGB4,
    RGB5 = glow::RGB5,
    RGB8 = glow::RGB8,
    RGB8Snorm = glow::RGB8_SNORM,
    RGB10 = glow::RGB10,
    RGB12 = glow::RGB12,
    RGB16Snorm = glow::RGB16_SNORM,
    RGBA2 = glow::RGBA2,
    RGBA4 = glow::RGBA4,
    RGB5A1 = glow::RGB5_A1,
    RGBA8 = glow::RGBA8,
    RGBA8Snorm = glow::RGBA8_SNORM,
    RGB10A2 = glow::RGB10_A2,
    RGB10A2ui = glow::RGB10_A2UI,
    RGBA12 = glow::RGBA12,
    RGBA16 = glow::RGBA16,
    SRGB8 = glow::SRGB8,
    SRGB8Alpha8 = glow::SRGB8_ALPHA8,
    R16f = glow::R16F,
    RG16f = glow::RG16F,
    RGB16f = glow::RGB16F,
    RGBA16f = glow::RGBA16F,
    R32f = glow::R32F,
    RG32f = glow::RG32F,
    RGB32f = glow::RGB32F,
    RGBA32f = glow::RGBA32F,
    R11fG11fB10f = glow::R11F_G11F_B10F,
    RGB9E5 = glow::RGB9_E5,
    R8i = glow::R8I,
    R8ui = glow::R8UI,
    R16i = glow::R16I,
    R16ui = glow::R16UI,
    R32i = glow::R32I,
    R32ui = glow::R32UI,
    RG8i = glow::RG8I,
    RG8ui = glow::RG8UI,
    RG16i = glow::RG16I,
    RG16ui = glow::RG16UI,
    RG32i = glow::RG32I,
    RG32ui = glow::RG32UI,
    RGB8i = glow::RGB8I,
    RGB8ui = glow::RGB8UI,
    RGB16i = glow::RGB16I,
    RGB16ui = glow::RGB16UI,
    RGB32i = glow::RGB32I,
    RGB32ui = glow::RGB32UI,
    RGBA8i = glow::RGBA8I,
    RGBA8ui = glow::RGBA8UI,
    RGBA16i = glow::RGBA16I,
    RGBA16ui = glow::RGBA16UI,
    RGBA32i = glow::RGBA32I,
    RGBA32ui = glow::RGBA32UI,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum PixelType {
    UnsignedByte = glow::UNSIGNED_BYTE,
    Byte = glow::BYTE,
    UnsignedShort = glow::UNSIGNED_SHORT,
    Short = glow::SHORT,
    UnsignedInt = glow::UNSIGNED_INT,
    Int = glow::INT,
    Float = glow::FLOAT,
    UnsignedByte3_3_2 = glow::UNSIGNED_BYTE_3_3_2,
    UnsignedByte2_3_3_Rev = glow::UNSIGNED_BYTE_2_3_3_REV,
    UnsignedShort5_6_5 = glow::UNSIGNED_SHORT_5_6_5,
    UnsignedShort5_6_5_Rev = glow::UNSIGNED_SHORT_5_6_5_REV,
    UnsignedShort4_4_4_4 = glow::UNSIGNED_SHORT_4_4_4_4,
    UnsignedShort4_4_4_4_Rev = glow::UNSIGNED_SHORT_4_4_4_4_REV,
    UnsignedShort5_5_5_1 = glow::UNSIGNED_SHORT_5_5_5_1,
    UnsignedShort1_5_5_5_Rev = glow::UNSIGNED_SHORT_1_5_5_5_REV,
    UnsignedInt8_8_8_8 = glow::UNSIGNED_INT_8_8_8_8,
    UnsignedInt8_8_8_8_Rev = glow::UNSIGNED_INT_8_8_8_8_REV,
    UnsignedInt10_10_10_2 = glow::UNSIGNED_INT_10_10_10_2,
    UnsignedInt2_10_10_10_Rev = glow::UNSIGNED_INT_2_10_10_10_REV,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureParameterName {
    DepthStencilTextureMode = glow::DEPTH_STENCIL_TEXTURE_MODE,
    TextureBaseLevel = glow::TEXTURE_BASE_LEVEL,
    TextureCompareFunc = glow::TEXTURE_COMPARE_FUNC,
    TextureCompareMode = glow::TEXTURE_COMPARE_MODE,
    TextureLodBias = glow::TEXTURE_LOD_BIAS,
    TextureMinFilter = glow::TEXTURE_MIN_FILTER,
    TextureMagFilter = glow::TEXTURE_MAG_FILTER,
    TextureMinLod = glow::TEXTURE_MIN_LOD,
    TextureMaxLod = glow::TEXTURE_MAX_LOD,
    TextureMaxLevel = glow::TEXTURE_MAX_LEVEL,
    TextureSwizzleR = glow::TEXTURE_SWIZZLE_R,
    TextureSwizzleG = glow::TEXTURE_SWIZZLE_G,
    TextureSwizzleB = glow::TEXTURE_SWIZZLE_B,
    TextureSwizzleA = glow::TEXTURE_SWIZZLE_A,
    TextureWrapS = glow::TEXTURE_WRAP_S,
    TextureWrapT = glow::TEXTURE_WRAP_T,
    TextureWrapR = glow::TEXTURE_WRAP_R,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureWrapValue {
    ClampToEdge = glow::CLAMP_TO_EDGE,
    ClampToBorder = glow::CLAMP_TO_BORDER,
    MirroredRepeat = glow::MIRRORED_REPEAT,
    Repeat = glow::REPEAT,
    MirrorClampToEdge = glow::MIRROR_CLAMP_TO_EDGE,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureMinFilterValue {
    Nearest = glow::NEAREST,
    Linear = glow::LINEAR,
    NearestMipmapNearest = glow::NEAREST_MIPMAP_NEAREST,
    LinearMipmapNearest = glow::LINEAR_MIPMAP_NEAREST,
    NearestMipmapLinear = glow::NEAREST_MIPMAP_LINEAR,
    LinearMipmapLinear = glow::LINEAR_MIPMAP_LINEAR,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureMagFilterValue {
    Nearest = glow::NEAREST,
    Linear = glow::LINEAR,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum AttributeType {
    Byte = glow::BYTE,
    UnsignedByte = glow::UNSIGNED_BYTE,
    Short = glow::SHORT,
    UnsignedShort = glow::UNSIGNED_SHORT,
    Int = glow::INT,
    UnsignedInt = glow::UNSIGNED_INT,
    HalfFloat = glow::HALF_FLOAT,
    Float = glow::FLOAT,
    Double = glow::DOUBLE,
    Fixed = glow::FIXED,
    Int2_10_10_10_Rev = glow::INT_2_10_10_10_REV,
    UnsignedInt2_10_10_10_Rev = glow::UNSIGNED_INT_2_10_10_10_REV,
    UnsignedInt10f_11f_11f_Rev = glow::UNSIGNED_INT_10F_11F_11F_REV,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IndiceType {
    UnsignedByte = glow::UNSIGNED_BYTE,
    UnsignedShort = glow::UNSIGNED_SHORT,
    UnsignedInt = glow::UNSIGNED_INT,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum DrawMode {
    Points = glow::POINTS,
    LineStrip = glow::LINE_STRIP,
    LineLoop = glow::LINE_LOOP,
    Lines = glow::LINES,
    LineStripAdjacency = glow::LINE_STRIP_ADJACENCY,
    LinesAdjacency = glow::LINES_ADJACENCY,
    TriangleStrip = glow::TRIANGLE_STRIP,
    TriangleFan = glow::TRIANGLE_FAN,
    Triangles = glow::TRIANGLES,
    TriangleStripAdjacency = glow::TRIANGLE_STRIP_ADJACENCY,
    TrianglesAdjacency = glow::TRIANGLES_ADJACENCY,
    Patches = glow::PATCHES,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum StringTarget {
    Vendor = glow::VENDOR,
    Renderer = glow::RENDERER,
    Version = glow::VERSION,
    ShadingLanguageVersion = glow::SHADING_LANGUAGE_VERSION,
    Extensions = glow::EXTENSIONS,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ShaderType {
    Vertex = glow::VERTEX_SHADER,
    Fragment = glow::FRAGMENT_SHADER,
    Geometry = glow::GEOMETRY_SHADER,
    Compute = glow::COMPUTE_SHADER,
}

pub mod resource {
    pub type Shader = glow::Shader;
    pub type Program = glow::Program;
    pub type Buffer = glow::Buffer;
    pub type VertexArray = glow::VertexArray;
    pub type Texture = glow::Texture;
    pub type Sampler = glow::Sampler;
    pub type Fence = glow::Fence;
    pub type Framebuffer = glow::Framebuffer;
    pub type Renderbuffer = glow::Renderbuffer;
    pub type Query = glow::Query;
    pub type UniformLocation = glow::UniformLocation;
    pub type TransformFeedback = glow::TransformFeedback;
}

use std::rc::Rc;
pub struct OpenGL {
    gl: Rc<glow::Context>,
}

impl OpenGL {
    pub fn new(gl: glow::Context) -> OpenGL {
        OpenGL {
            gl: Rc::new(gl),
        }
    }

    pub fn clone(&self) -> OpenGL {
        OpenGL {
            gl: self.gl.clone(),
        }
    }

    pub fn get_string(&self, parameter: StringTarget) -> String {
        unsafe { self.gl.get_parameter_string(parameter as u32) }
    }

    pub fn get_error(&self, context: &str) {
        let error = unsafe { self.gl.get_error() };
        if error > 0 {
            warn!("GL ERROR: {} at {}", error, context);
        }
    }

    pub fn create_program(&self) -> resource::Program {
        unsafe { self.gl.create_program().unwrap() }
    }

    pub fn create_shader(&self, shader_type: ShaderType) -> resource::Shader {
        unsafe { self.gl.create_shader(shader_type as u32).unwrap() }
    }

    pub fn attach_shader(&self, program: resource::Program, shader: resource::Shader) {
        unsafe { self.gl.attach_shader(program, shader) };
    }

    pub fn link_program(&self, program: resource::Program) {
        unsafe { self.gl.link_program(program) };
    }

    pub fn delete_shader(&self, shader: resource::Shader) {
        unsafe { self.gl.delete_shader(shader) };
    }

    pub fn use_program(&self, program: Option<resource::Program>) {
        unsafe { self.gl.use_program(program) };
    }

    pub fn delete_program(&self, program: resource::Program) {
        unsafe { self.gl.delete_program(program) };
    }

    pub fn shader_source(&self, shader: resource::Shader, source: &str) {
        unsafe { self.gl.shader_source(shader, source) };
    }

    pub fn compile_shader(&self, shader: resource::Shader) {
        unsafe { self.gl.compile_shader(shader) };
    }

    pub fn check_program(&self, program: resource::Program) -> Result<(), String> {
        unsafe {
            if self.gl.get_program_link_status(program) {
                Ok(())
            } else {
                Err(self.gl.get_program_info_log(program))
            }
        }
    }

    pub fn check_shader(&self, shader: resource::Shader) -> Result<(), String> {
        unsafe {
            if self.gl.get_shader_compile_status(shader) {
                Ok(())
            } else {
                Err(self.gl.get_shader_info_log(shader))
            }
        }
    }

    pub fn get_uniform_location(
        &self,
        program: resource::Program,
        name: &str,
    ) -> Option<resource::UniformLocation> {
        unsafe { self.gl.get_uniform_location(program, name) }
    }

    pub fn uniform_matrix_4fv(
        &self,
        location: Option<&resource::UniformLocation>,
        transpose: bool,
        v: &[f32; 16],
    ) {
        unsafe {
            self.gl.uniform_matrix_4_f32_slice(location, transpose, v);
        }
    }

    pub fn uniform_1i(&self, location: Option<&resource::UniformLocation>, x: i32) {
        unsafe { self.gl.uniform_1_i32(location, x) };
    }

    pub fn create_vertex_array(&self) -> resource::VertexArray {
        unsafe { self.gl.create_vertex_array().unwrap() }
    }

    pub fn bind_vertex_array(&self, vertex_array: Option<resource::VertexArray>) {
        unsafe { self.gl.bind_vertex_array(vertex_array) };
    }

    pub fn delete_vertex_array(&self, vertex_array: resource::VertexArray) {
        unsafe { self.gl.delete_vertex_array(vertex_array) };
    }

    pub fn enable_vertex_attrib_array(&self, index: u32) {
        unsafe { self.gl.enable_vertex_attrib_array(index) };
    }

    pub fn vertex_attrib_divisor(&self, index: u32, divisor: u32) {
        unsafe { self.gl.vertex_attrib_divisor(index, divisor) };
    }

    pub fn vertex_attrib_pointer_f32(
        &self,
        index: u32,
        size: i32,
        data_type: AttributeType,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        unsafe {
            self.gl.vertex_attrib_pointer_f32(index, size, data_type as u32, normalized, stride, offset)
        };
    }

    pub fn create_buffer(&self) -> resource::Buffer {
        unsafe { self.gl.create_buffer().unwrap() }
    }

    pub fn bind_buffer(&self, target: BufferBindingTarget, buffer: Option<resource::Buffer>) {
        unsafe { self.gl.bind_buffer(target as u32, buffer) };
    }

    pub fn delete_buffer(&self, buffer: resource::Buffer) {
        unsafe { self.gl.delete_buffer(buffer) };
    }

    pub fn buffer_data_empty(&self, target: BufferBindingTarget, size: i32, usage: BufferUsage) {
        unsafe { self.gl.buffer_data_size(target as u32, size, usage as u32) };
    }

    pub fn buffer_data<T: Sized>(&self, target: BufferBindingTarget, data: &[T], usage: BufferUsage) {
        unsafe {
            let len = core::mem::size_of::<T>() * data.len();
            let ptr = data.as_ptr() as *const u8;
            let slice = core::slice::from_raw_parts(ptr, len);
            self.gl.buffer_data_u8_slice(target as u32, slice, usage as u32)
        };
    }

    pub fn draw_arrays_instanced(&self, mode: DrawMode, first: i32, count: i32, instance_count: i32) {
        unsafe { self.gl.draw_arrays_instanced(mode as u32, first, count, instance_count) };
    }

    pub fn create_texture(&self) -> resource::Texture {
        unsafe { self.gl.create_texture().unwrap() }
    }

    pub fn active_texture(&self, unit: TextureUnit) {
        unsafe { self.gl.active_texture(unit as u32) };
    }

    pub fn delete_texture(&self, texture: resource::Texture) {
        unsafe { self.gl.delete_texture(texture) };
    }

    pub fn bind_texture(&self, target: TextureBindingTarget, texture: Option<resource::Texture>) {
        unsafe { self.gl.bind_texture(target as u32, texture) };
    }

    pub fn tex_image_2d<T: Sized>(
        &self,
        target: TextureLoadTarget,
        level: i32,
        width: i32,
        height: i32,
        border: i32,
        internal_format: PixelInternalFormat,
        format: PixelFormat,
        ty: PixelType,
        pixels: &[T],
    ) {
        unsafe {
            let len = core::mem::size_of::<T>() * pixels.len();
            let ptr = pixels.as_ptr() as *const u8;
            let slice = core::slice::from_raw_parts(ptr, len);
            self.gl.tex_image_2d(
                target as u32,
                level,
                internal_format as u32 as i32,
                width,
                height,
                border,
                format as u32,
                ty as u32,
                Some(slice),
            )
        };
    }

    pub fn tex_parameter_wrap_s(&self, target: TextureParameterTarget, value: TextureWrapValue) {
        unsafe {
            self.gl.tex_parameter_i32(target as u32, TextureParameterName::TextureWrapS as u32, value as i32)
        };
    }

    pub fn tex_parameter_wrap_t(&self, target: TextureParameterTarget, value: TextureWrapValue) {
        unsafe {
            self.gl.tex_parameter_i32(target as u32, TextureParameterName::TextureWrapT as u32, value as i32)
        };
    }

    pub fn tex_parameter_wrap_r(&self, target: TextureParameterTarget, value: TextureWrapValue) {
        unsafe {
            self.gl.tex_parameter_i32(target as u32, TextureParameterName::TextureWrapR as u32, value as i32)
        };
    }

    pub fn tex_parameter_min_filter(&self, target: TextureParameterTarget, value: TextureMinFilterValue) {
        unsafe {
            self.gl.tex_parameter_i32(
                target as u32,
                TextureParameterName::TextureMinFilter as u32,
                value as i32,
            )
        };
    }

    pub fn tex_parameter_mag_filter(&self, target: TextureParameterTarget, value: TextureMagFilterValue) {
        unsafe {
            self.gl.tex_parameter_i32(
                target as u32,
                TextureParameterName::TextureMagFilter as u32,
                value as i32,
            )
        };
    }

    pub fn enable(&self, capability: Capability) {
        unsafe { self.gl.enable(capability as u32) };
    }

    pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe { self.gl.clear_color(red, green, blue, alpha) };
    }

    pub fn depth_func(&self, test: DepthTest) {
        unsafe { self.gl.depth_func(test as u32) };
    }

    pub fn blend_func(&self, src: BlendFactor, dst: BlendFactor) {
        unsafe { self.gl.blend_func(src as u32, dst as u32) };
    }

    pub fn cull_face(&self, face: CullFace) {
        unsafe { self.gl.cull_face(face as u32) };
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { self.gl.viewport(x, y, width, height) };
    }

    pub fn clear(&self, mask: u32) {
        unsafe { self.gl.clear(mask) };
    }
}
