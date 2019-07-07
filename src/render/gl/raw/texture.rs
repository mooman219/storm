use std::os::raw::c_void;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureUnit {
    Atlas = gl::TEXTURE0,
    Font = gl::TEXTURE1,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureLoadTarget {
    Texture2D = gl::TEXTURE_2D,
    ProxyTexture2D = gl::PROXY_TEXTURE_2D,
    Texture1DArray = gl::TEXTURE_1D_ARRAY,
    ProxyTexture1DArray = gl::PROXY_TEXTURE_1D_ARRAY,
    TextureRectangle = gl::TEXTURE_RECTANGLE,
    ProxyTextureRectangle = gl::PROXY_TEXTURE_RECTANGLE,
    TextureCubeMapPositiveX = gl::TEXTURE_CUBE_MAP_POSITIVE_X,
    TextureCubeMapNegativeX = gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
    TextureCubeMapPositiveY = gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
    TextureCubeMapNegativeY = gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
    TextureCubeMapPositiveZ = gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
    TextureCubeMapNegativeZ = gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
    ProxyTextureCubeMap = gl::PROXY_TEXTURE_CUBE_MAP,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureBindingTarget {
    TextureBuffer = gl::TEXTURE_BUFFER,
    Texture1D = gl::TEXTURE_1D,
    Texture1DArray = gl::TEXTURE_1D_ARRAY,
    Texture2D = gl::TEXTURE_2D,
    Texture2DArray = gl::TEXTURE_2D_ARRAY,
    Texture2DMultisample = gl::TEXTURE_2D_MULTISAMPLE,
    Texture2DMultisampleArray = gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
    Texture3D = gl::TEXTURE_3D,
    TextureCubeMap = gl::TEXTURE_CUBE_MAP,
    TextureCubeMapArray = gl::TEXTURE_CUBE_MAP_ARRAY,
    TextureRectangle = gl::TEXTURE_RECTANGLE,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureParameterTarget {
    Texture1D = gl::TEXTURE_1D,
    Texture1DArray = gl::TEXTURE_1D_ARRAY,
    Texture2D = gl::TEXTURE_2D,
    Texture2DArray = gl::TEXTURE_2D_ARRAY,
    Texture2DMultisample = gl::TEXTURE_2D_MULTISAMPLE,
    Texture2DMultisampleArray = gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
    Texture3D = gl::TEXTURE_3D,
    TextureCubeMap = gl::TEXTURE_CUBE_MAP,
    TextureCubeMapArray = gl::TEXTURE_CUBE_MAP_ARRAY,
    TextureRectangle = gl::TEXTURE_RECTANGLE,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum PixelFormat {
    RED = gl::RED,
    RG = gl::RG,
    RGB = gl::RGB,
    BGR = gl::BGR,
    RGBA = gl::RGBA,
    BGRA = gl::BGRA,
    RedInteger = gl::RED_INTEGER,
    RGInteger = gl::RG_INTEGER,
    RGBInteger = gl::RGB_INTEGER,
    BGRInteger = gl::BGR_INTEGER,
    RGBAInteger = gl::RGBA_INTEGER,
    BGRAInteger = gl::BGRA_INTEGER,
    StencilIndex = gl::STENCIL_INDEX,
    DepthComponent = gl::DEPTH_COMPONENT,
    DepthStencil = gl::DEPTH_STENCIL,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum PixelInternalFormat {
    DepthComponent = gl::DEPTH_COMPONENT,
    DepthStencil = gl::DEPTH_STENCIL,
    Red = gl::RED,
    RG = gl::RG,
    RGB = gl::RGB,
    RGBA = gl::RGBA,
    R8 = gl::R8,
    R8Snorm = gl::R8_SNORM,
    R16 = gl::R16,
    R16Snorm = gl::R16_SNORM,
    RG8 = gl::RG8,
    RG8Snorm = gl::RG8_SNORM,
    RG16 = gl::RG16,
    RG16Snorm = gl::RG16_SNORM,
    R3G3B2 = gl::R3_G3_B2,
    RGB4 = gl::RGB4,
    RGB5 = gl::RGB5,
    RGB8 = gl::RGB8,
    RGB8Snorm = gl::RGB8_SNORM,
    RGB10 = gl::RGB10,
    RGB12 = gl::RGB12,
    RGB16Snorm = gl::RGB16_SNORM,
    RGBA2 = gl::RGBA2,
    RGBA4 = gl::RGBA4,
    RGB5A1 = gl::RGB5_A1,
    RGBA8 = gl::RGBA8,
    RGBA8Snorm = gl::RGBA8_SNORM,
    RGB10A2 = gl::RGB10_A2,
    RGB10A2ui = gl::RGB10_A2UI,
    RGBA12 = gl::RGBA12,
    RGBA16 = gl::RGBA16,
    SRGB8 = gl::SRGB8,
    SRGB8Alpha8 = gl::SRGB8_ALPHA8,
    R16f = gl::R16F,
    RG16f = gl::RG16F,
    RGB16f = gl::RGB16F,
    RGBA16f = gl::RGBA16F,
    R32f = gl::R32F,
    RG32f = gl::RG32F,
    RGB32f = gl::RGB32F,
    RGBA32f = gl::RGBA32F,
    R11fG11fB10f = gl::R11F_G11F_B10F,
    RGB9E5 = gl::RGB9_E5,
    R8i = gl::R8I,
    R8ui = gl::R8UI,
    R16i = gl::R16I,
    R16ui = gl::R16UI,
    R32i = gl::R32I,
    R32ui = gl::R32UI,
    RG8i = gl::RG8I,
    RG8ui = gl::RG8UI,
    RG16i = gl::RG16I,
    RG16ui = gl::RG16UI,
    RG32i = gl::RG32I,
    RG32ui = gl::RG32UI,
    RGB8i = gl::RGB8I,
    RGB8ui = gl::RGB8UI,
    RGB16i = gl::RGB16I,
    RGB16ui = gl::RGB16UI,
    RGB32i = gl::RGB32I,
    RGB32ui = gl::RGB32UI,
    RGBA8i = gl::RGBA8I,
    RGBA8ui = gl::RGBA8UI,
    RGBA16i = gl::RGBA16I,
    RGBA16ui = gl::RGBA16UI,
    RGBA32i = gl::RGBA32I,
    RGBA32ui = gl::RGBA32UI,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum PixelType {
    UnsignedByte = gl::UNSIGNED_BYTE,
    Byte = gl::BYTE,
    UnsignedShort = gl::UNSIGNED_SHORT,
    Short = gl::SHORT,
    UnsignedInt = gl::UNSIGNED_INT,
    Int = gl::INT,
    Float = gl::FLOAT,
    UnsignedByte3_3_2 = gl::UNSIGNED_BYTE_3_3_2,
    UnsignedByte2_3_3_Rev = gl::UNSIGNED_BYTE_2_3_3_REV,
    UnsignedShort5_6_5 = gl::UNSIGNED_SHORT_5_6_5,
    UnsignedShort5_6_5_Rev = gl::UNSIGNED_SHORT_5_6_5_REV,
    UnsignedShort4_4_4_4 = gl::UNSIGNED_SHORT_4_4_4_4,
    UnsignedShort4_4_4_4_Rev = gl::UNSIGNED_SHORT_4_4_4_4_REV,
    UnsignedShort5_5_5_1 = gl::UNSIGNED_SHORT_5_5_5_1,
    UnsignedShort1_5_5_5_Rev = gl::UNSIGNED_SHORT_1_5_5_5_REV,
    UnsignedInt8_8_8_8 = gl::UNSIGNED_INT_8_8_8_8,
    UnsignedInt8_8_8_8_Rev = gl::UNSIGNED_INT_8_8_8_8_REV,
    UnsignedInt10_10_10_2 = gl::UNSIGNED_INT_10_10_10_2,
    UnsignedInt2_10_10_10_Rev = gl::UNSIGNED_INT_2_10_10_10_REV,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureParameterName {
    DepthStencilTextureMode = gl::DEPTH_STENCIL_TEXTURE_MODE,
    TextureBaseLevel = gl::TEXTURE_BASE_LEVEL,
    TextureCompareFunc = gl::TEXTURE_COMPARE_FUNC,
    TextureCompareMode = gl::TEXTURE_COMPARE_MODE,
    TextureLodBias = gl::TEXTURE_LOD_BIAS,
    TextureMinFilter = gl::TEXTURE_MIN_FILTER,
    TextureMagFilter = gl::TEXTURE_MAG_FILTER,
    TextureMinLod = gl::TEXTURE_MIN_LOD,
    TextureMaxLod = gl::TEXTURE_MAX_LOD,
    TextureMaxLevel = gl::TEXTURE_MAX_LEVEL,
    TextureSwizzleR = gl::TEXTURE_SWIZZLE_R,
    TextureSwizzleG = gl::TEXTURE_SWIZZLE_G,
    TextureSwizzleB = gl::TEXTURE_SWIZZLE_B,
    TextureSwizzleA = gl::TEXTURE_SWIZZLE_A,
    TextureWrapS = gl::TEXTURE_WRAP_S,
    TextureWrapT = gl::TEXTURE_WRAP_T,
    TextureWrapR = gl::TEXTURE_WRAP_R,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureWrapValue {
    ClampToEdge = gl::CLAMP_TO_EDGE,
    ClampToBorder = gl::CLAMP_TO_BORDER,
    MirroredRepeat = gl::MIRRORED_REPEAT,
    Repeat = gl::REPEAT,
    MirrorClampToEdge = gl::MIRROR_CLAMP_TO_EDGE,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureMinFilterValue {
    Nearest = gl::NEAREST,
    Linear = gl::LINEAR,
    NearestMipmapNearest = gl::NEAREST_MIPMAP_NEAREST,
    LinearMipmapNearest = gl::LINEAR_MIPMAP_NEAREST,
    NearestMipmapLinear = gl::NEAREST_MIPMAP_LINEAR,
    LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureMagFilterValue {
    Nearest = gl::NEAREST,
    Linear = gl::LINEAR,
}

/// [2.0] Generate a texture name.
#[inline]
pub fn gen_texture() -> u32 {
    unsafe {
        let mut id = 0;
        gl::GenTextures(1, &mut id);
        id
    }
}

/// [2.0] Select active texture unit.
///
/// # Arguments
///
/// `texture` - Specifies which texture unit to make active. The number of texture units is
/// implementation dependent, but must be at least 80.
#[inline]
pub fn active_texture(texture: TextureUnit) {
    unsafe {
        gl::ActiveTexture(texture as u32);
    }
}

/// [2.0] Bind a named texture to a texturing target.
///
/// # Arguments
///
/// `target` - Specifies the target to which the texture is bound.
/// `texture` - Specifies the name of a texture.
#[inline]
pub fn bind_texture(target: TextureBindingTarget, texture: u32) {
    unsafe {
        gl::BindTexture(target as u32, texture);
    }
}

/// [2.0] Delete named textures.
///
/// # Arguments
///
/// `name` - Specifies a textures to be deleted.
#[inline]
pub fn delete_texture(name: u32) {
    unsafe {
        gl::DeleteTextures(1, &name as *const _);
    }
}

/// [2.0] Specify a two-dimensional texture image.
///
/// # Arguments
///
/// `target` - Specifies the target texture.
/// `level` - Specifies the level-of-detail number. Level 0 is the base image level. Level n is the
/// nth mipmap reduction image. `width` - Specifies the width of the texture image. All
/// implementations support texture images that are at least 1024 texels wide. `height` - Specifies
/// the height of the texture image, or the number of layers in a texture array, in the case of the
/// GL_TEXTURE_1D_ARRAY and GL_PROXY_TEXTURE_1D_ARRAY targets. All implementations support 2D
/// texture images that are at least 1024 texels high, and texture arrays that are at least 256
/// layers deep. `internal_format` - Specifies the number of color components in the texture.
/// `format` - Specifies the format of the pixel data.
/// `pixel_type` - Specifies the data type of the pixel data.
/// `data` - Specifies a pointer to the image data in memory.
#[inline]
pub fn tex_image_2D(
    target: TextureLoadTarget,
    level: i32,
    width: i32,
    height: i32,
    internal_format: PixelInternalFormat,
    format: PixelFormat,
    pixel_type: PixelType,
    data: *const c_void,
) {
    unsafe {
        gl::TexImage2D(
            target as u32,
            level,
            internal_format as i32,
            width,
            height,
            0, // Border is ignored and must be 0.
            format as u32,
            pixel_type as u32,
            data,
        );
    }
}

/// [2.0] Sets the wrap parameter for texture coordinate s.
///
/// # Arguments
///
/// `target` - Specifies the target texture.
/// `param` - The wrap parameter for texture coordinate.
#[inline]
pub fn tex_parameter_wrap_s(target: TextureParameterTarget, param: TextureWrapValue) {
    unsafe {
        gl::TexParameteri(target as u32, gl::TEXTURE_WRAP_S, param as i32);
    }
}

/// [2.0] Sets the wrap parameter for texture coordinate t.
///
/// # Arguments
///
/// `target` - Specifies the target texture.
/// `param` - The wrap parameter for texture coordinate.
#[inline]
pub fn tex_parameter_wrap_t(target: TextureParameterTarget, param: TextureWrapValue) {
    unsafe {
        gl::TexParameteri(target as u32, gl::TEXTURE_WRAP_T, param as i32);
    }
}

/// [2.0] Sets the wrap parameter for texture coordinate r.
///
/// # Arguments
///
/// `target` - Specifies the target texture.
/// `param` - The wrap parameter for texture coordinate.
#[inline]
pub fn tex_parameter_wrap_r(target: TextureParameterTarget, param: TextureWrapValue) {
    unsafe {
        gl::TexParameteri(target as u32, gl::TEXTURE_WRAP_R, param as i32);
    }
}

/// [2.0] The texture minifying function is used whenever the level-of-detail function used when
/// sampling from the texture determines that the texture should be minified.
///
/// # Arguments
///
/// `target` - Specifies the target texture.
/// `param` - The texture minifying function.
#[inline]
pub fn tex_parameter_min_filter(target: TextureParameterTarget, param: TextureMinFilterValue) {
    unsafe {
        gl::TexParameteri(target as u32, gl::TEXTURE_MIN_FILTER, param as i32);
    }
}

/// [2.0] The texture magnification function is used whenever the level-of-detail function used when
/// sampling from the texture determines that the texture should be magified.
///
/// # Arguments
///
/// `target` - Specifies the target texture.
/// `param` - The texture magnification function.
#[inline]
pub fn tex_parameter_mag_filter(target: TextureParameterTarget, param: TextureMagFilterValue) {
    unsafe {
        gl::TexParameteri(target as u32, gl::TEXTURE_MAG_FILTER, param as i32);
    }
}
