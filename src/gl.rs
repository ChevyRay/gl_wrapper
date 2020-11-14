extern crate gl;
use std::os::raw::c_void;

pub fn load<F: Fn(&'static str) -> *const c_void>(load_fn: F) {
    gl::load_with(load_fn);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ErrorType
{
    NoError,
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    Unknown,
}
impl From<u32> for ErrorType {
    fn from(val: u32) -> Self {
        match val {
            0 => Self::NoError,
            0x0500 => Self::InvalidEnum,
            0x0501 => Self::InvalidValue,
            0x0502 => Self::InvalidOperation,
            0x0505 => Self::OutOfMemory,
            0x0506 => Self::InvalidFramebufferOperation,
            _ => Self::Unknown,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EnableCap {
    Blend = 0x0BE2,
    CullFace = 0x0B44,
    DepthTest = 0x0B71,
    Dither = 0x0BD0,
    PolygonOffsetFill = 0x8037,
    RasterizerDiscard = 0x8C89,
    SampleAlphaToCoverage = 0x809E,
    SampleCoverage = 0x80A0,
    ScissorTest = 0x0C11,
    StencilTest = 0x0B90,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CullFace {
    Front = 0x0404,
    Back = 0x0405,
    FrontAndBack = 0x0408,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FrontFace {
    Clockwise = 0x0900,
    CounterClockwise = 0x0901,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlendEquation {
    Add = 32774,
    Subtract = 32778,
    ReverseSubtract = 32779,
    Max = 32776,
    Min = 32775,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlendFactor {
    Zero = 0,
    One = 1,
    SrcColor = 0x0300,
    OneMinusSrcColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    DstColor,
    OneMinusDstcolor,
    SrcAlphaSaturate,
    ConstantColor = 0x8001,
    OneMinusConstantColor,
    ConstantAlpha,
    OneMinusConstantAlpha,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextureTarget {
    Texture2D = 0x0DE1,
    Texture3D = 0x806F,
    Texture2dArray = 0x8C1A,
    TextureCubeMap = 0x8513,
    TextureCubeMapPosX = 0x8515,
    TextureCubeMapNegX = 0x8516,
    TextureCubeMapPosY = 0x8517,
    TextureCubeMapNegY = 0x8518,
    TextureCubeMapPosZ = 0x8519,
    TextureCubeMapNegZ = 0x851A,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextureParam {
    BaseLevel = 0x813C,
    CompareFunc = 0x884D,
    CompareMode = 0x884C,
    MinFilter = 0x2801,
    MagFilter = 0x2800,
    MinLOD = 0x813A,
    MaxLOD = 0x813B,
    MaxLevel = 0x813D,
    SwizzleR = 0x8E42,
    SwizzleG = 0x8E43,
    SwizzleB = 0x8E44,
    SwizzleA = 0x8E45,
    WrapS = 0x2802,
    WrapT = 0x2803,
    WrapR = 0x8072,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextureFormat {
    //Depth textures
    Depth = 0x1902,
    Depth16 = 0x81A5,
    Depth24 = 0x81A6,
    Depth32 = 0x81A7,
    Depth32F = 0x81A8,

    DepthStencil = 0x84F9,
    Depth24Stencil8 = 0x88F0,

    //Red textures
    R = 0x1903,
    R8 = 0x8229,
    R8SNorm = 0x8F94,
    R16F = 0x822D,
    R32F = 0x822E,
    R8I = 0x8231,
    R8UI = 0x8232,
    R16I = 0x8233,
    R16UI = 0x8234,
    R32I = 0x8235,
    R32UI = 0x8236,

    //RG textures
    RG = 0x8227,
    RG8 = 0x822B,
    RG8SNorm = 0x8F95,
    RG16F = 0x822F,
    RG32F = 0x8230,
    RG8I = 0x8237,
    RG8UI = 0x8238,
    RG16I = 0x8239,
    RG16UI = 0x823A,
    RG32I = 0x823B,
    RG32UI = 0x823C,

    //RGB textures
    RGB = 0x1907,
    RGB8 = 0x8051,
    RGB8SNorm = 0x8F96,
    RGB16F = 0x881B,
    RGB32F = 0x8815,
    RGB8I = 0x8D8F,
    RGB8UI = 0x8D7D,
    RGB16I = 0x8D89,
    RGB16UI = 0x8D77,
    RGB32I = 0x8D83,
    RGB32UI = 0x8D71,
    R3G3B2 = 0x2A10,
    R5G6B5 = 0x8D62,
    R11G11B10F = 0x8C3A,

    //RGBA textures
    RGBA = 0x1908,
    RGBA8 = 0x8058,
    RGBA8SNorm = 0x8F97,
    RGBA16F = 0x881A,
    RGBA32F = 0x8814,
    RGBA8I = 0x8D8E,
    RGBA8UI = 0x8D7C,
    RGBA16I = 0x8D88,
    RGBA16UI = 0x8D76,
    RGBA32I = 0x8D82,
    RGBA32UI = 0x8D70,
    RGBA2 = 0x8055,
    RGBA4 = 0x8056,
    RGB5A1 = 0x8057,
    RGB10A2 = 0x8059,
    RGB10A2UI = 0x906F,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PixelFormat {
    R = 0x1903,
    RG = 0x8227,
    RGB = 0x1907,
    RGBA = 0x1908,
    Depth = 0x1902,
}
impl PixelFormat { 
    fn component_count(&self) -> usize {
        match self {
            Self::R => 1,
            Self::RG => 2,
            Self::RGB => 3,
            Self::RGBA => 4,
            Self::Depth => 1,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PixelType { 
    Byte = 0x1400,
    UnsignedByte = 0x1401,
    Short = 0x1402,
    UnsignedShort = 0x1403,
    Int = 0x1404,
    UnsignedInt = 0x1405,
    Float = 0x1406
}
impl PixelType {
    fn size_in_bytes(&self) -> usize {
        match self {
            Self::Byte => 1,
            Self::UnsignedByte => 1,
            Self::Short => 2,
            Self::UnsignedShort => 2,
            Self::Int => 4,
            Self::UnsignedInt => 4,
            Self::Float => 4,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FramebufferTarget {
    Framebuffer = 0x8D40,
    Draw = 0x8CA9,
    Read = 0x8CA8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DrawBuffer {
    None = 0,
    Back = 0x0405,
    Color0 = 0x8CE0,
    Color1,
    Color2,
    Color3,
    Color4,
    Color5,
    Color6,
    Color7,
    Color8,
    Color9,
    Color10,
    Color11,
    Color12,
    Color13,
    Color14,
    Color15,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ReadBuffer {
    Depth = 0x8D00,
    Color0 = 0x8CE0,
    Color1,
    Color2,
    Color3,
    Color4,
    Color5,
    Color6,
    Color7,
    Color8,
    Color9,
    Color10,
    Color11,
    Color12,
    Color13,
    Color14,
    Color15,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VertexType {
    Byte = 0x1400,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Float
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BufferTarget {
    Array = 0x8892,
    ElementArray = 0x8893,
    CopyRead = 0x8F36,
    CopyWrite = 0x8F37,
    PixelPack = 0x88EB,
    PixelUnpack = 0x88EC,
    TransformFeedback = 0x8C8E,
    Uniform = 0x8A11
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BufferUsage {
    StreamDraw = 0x88E0,
    StreamRead = 0x88E1,
    StreamCopy = 0x88E2,
    StaticDraw = 0x88E4,
    StaticRead = 0x88E5,
    StaticCopy = 0x88E6,
    DynamicDraw = 0x88E8,
    DynamicRead = 0x88E9,
    DynamicCopy = 0x88EA
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShaderType {
    Vertex = 0x8B31,
    Fragment = 0x8B30
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShaderParam {
    ShaderType = 0x8B4F,
    DeleteStatus = 0x8B80,
    CompileStatus = 0x8B81,
    InfoLogLength = 0x8B84,
    ShaderSourceLength = 0x8B88
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProgramParam {
    DeleteStatus = 0x8B80,
    LinkStatus = 0x8B82,
    ValidateStatus = 0x8B83,
    InfoLogLength = 0x8B84,
    AttachedShaders = 0x8B85,
    ActiveAttributes = 0x8B89,
    ActiveAttributeMaxLength = 0x8B8A,
    ActiveUniforms = 0x8B86,
    ActiveUniformMaxLength = 0x8B87
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UniformType {
    Bool = 0x8B56,
    Int = 0x1404,
    Float = 0x1406,
    Vec2 = 0x8B50,
    Vec3 = 0x8B51,
    Vec4 = 0x8B52,
    Mat3 = 0x8B5B,
    Mat4 = 0x8B5C,
    Sampler2D = 0x8B5E,
    SamplerCube = 0x8B60,
}
impl UniformType {
    pub fn from(val: u32) -> Result<UniformType, String> {
        match val {
            0x8B56 => Ok(Self::Bool),
            0x1404 => Ok(Self::Int),
            0x1406 => Ok(Self::Float),
            0x8B50 => Ok(Self::Vec2),
            0x8B51 => Ok(Self::Vec3),
            0x8B52 => Ok(Self::Vec4),
            0x8B5B => Ok(Self::Mat3),
            0x8B5C => Ok(Self::Mat4),
            0x8B5E => Ok(Self::Sampler2D),
            0x8B60 => Ok(Self::SamplerCube),
            _ => Err("unknown uniform type".to_string()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PrimitiveType {
    Points = 0x0000,
    Lines = 0x0001,
    LineLoop = 0x0002,
    LineStrip = 0x0003,
    Triangles = 0x0004,
    TriangleStrip = 0x0005,
    TriangleFan = 0x0006,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IndexType {
    UnsignedByte = 0x1401,
    UnsignedShort = 0x1403,
    UnsignedInt = 0x1405,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ClearMode {
    Color = 0x4000,
    Depth = 0x0100,
    ColorAndDepth = 0x0100 | 0x4000,
}

#[inline]
fn check_error() -> Result<(), String> {
    let err_code = unsafe { gl::GetError() };
    let err_type = ErrorType::from(err_code);
    if err_type == ErrorType::NoError {
        Ok(())
    } else {
        let err_str = format!("{:?}", err_type);
        Err(err_str)
    }
}

#[inline]
fn get_integer_v(name: u32) -> Result<i32, String> {
    let mut val: i32 = 0;
    unsafe { gl::GetIntegerv(name, &mut val as *mut i32) };
    check_error()?;
    Ok(val)
}

#[inline]
pub fn version_str() -> Result<String, String> {
    let major = major_version()?;
    let minor = minor_version()?;
    Ok(format!("{}.{}", major, minor))
}

#[inline]
pub fn major_version() -> Result<i32, String> {
    get_integer_v(0x821B)
}

#[inline]
pub fn minor_version() -> Result<i32, String> {
    get_integer_v(0x821C)
}

#[inline]
pub fn max_color_attachments() -> Result<usize, String> {
    get_integer_v(0x8CDF).map(|n| n as usize)
}

#[inline]
pub fn max_cube_map_texture_size() -> Result<usize, String> {
    get_integer_v(0x851C).map(|n| n as usize)
}

#[inline]
pub fn max_draw_buffers() -> Result<usize, String> {
    get_integer_v(0x8824).map(|n| n as usize)
}

#[inline]
pub fn max_element_indices() -> Result<usize, String> {
    get_integer_v(0x80E9).map(|n| n as usize)
}

#[inline]
pub fn max_element_vertices() -> Result<usize, String> {
    get_integer_v(0x80E8).map(|n| n as usize)
}

#[inline]
pub fn max_renderbuffer_size() -> Result<usize, String> {
    get_integer_v(0x84E8).map(|n| n as usize)
}

#[inline]
pub fn max_samples() -> Result<usize, String> {
    get_integer_v(0x8D57).map(|n| n as usize)
}

#[inline]
pub fn max_texture_size() -> Result<usize, String> {
    get_integer_v(0x0D33).map(|n| n as usize)
}

#[inline]
pub fn max_texture_image_units() -> Result<usize, String> {
    get_integer_v(0x8872).map(|n| n as usize)
}

#[inline]
pub fn enable(mode: EnableCap) -> Result<(), String> {
    unsafe { gl::Enable(mode as u32) };
    check_error()
}

#[inline]
pub fn clear(mode: ClearMode) -> Result<(), String> {
    unsafe { gl::Clear(mode as u32) };
    check_error()
}

#[inline]
pub fn clear_color(r: f32, g: f32, b: f32, a: f32) -> Result<(), String> {
    unsafe { gl::ClearColor(r, g, b, a) };
    check_error()
}

#[inline]
pub fn cull_face(face: CullFace) -> Result<(), String> {
    unsafe { gl::CullFace(face as u32) };
    check_error()
}

#[inline]
pub fn front_face(face: FrontFace) -> Result<(), String> {
    unsafe { gl::FrontFace(face as u32) };
    check_error()
}

#[inline]
pub fn blend_equation(eq: BlendEquation) -> Result<(), String> {
    unsafe { gl::BlendEquation(eq as u32) };
    check_error()
}

#[inline]
pub fn blend_func(src: BlendFactor, dst: BlendFactor) -> Result<(), String> {
    unsafe { gl::BlendFunc(src as u32, dst as u32) };
    check_error()
}

#[inline]
pub fn gen_texture() -> Result<u32, String> {
    let mut tex: u32 = 0;
    unsafe { gl::GenTextures(1, &mut tex as *mut u32) };
    check_error()?;
    Ok(tex)
}

#[inline]
pub fn delete_texture(tex: u32) -> Result<(), String> {
    unsafe { gl::DeleteTextures(1, &tex as *const u32) };
    check_error()
}

#[inline]
pub fn active_texture(unit: u32) -> Result<(), String> {
    unsafe { gl::ActiveTexture(0x84C0 + unit) };
    check_error()
}

#[inline]
pub fn bind_texture(target: TextureTarget, tex: u32) -> Result<(), String> {
    unsafe { gl::BindTexture(target as u32, tex) };
    check_error()
}

#[inline]
pub fn tex_parameter_i(target: TextureTarget, name: TextureParam, param: i32) -> Result<(), String> {
    unsafe { gl::TexParameteri(target as u32, name as u32, param) };
    check_error()
}

#[inline]
pub fn get_tex_parameter_i(target: TextureTarget, name: TextureParam) -> Result<i32, String> {
    let mut val: i32 = 0;
    unsafe { gl::GetTexParameteriv(target as u32, name as u32, &mut val as *mut i32) };
    check_error()?;
    Ok(val)
}

#[inline]
pub fn tex_image_2d(
    target: TextureTarget,
    level: i32, 
    internal_format: TextureFormat, 
    width: i32, 
    height: i32, 
    border: i32, 
    format: PixelFormat, 
    ty: PixelType,
    data: &[u8]
) -> Result<(), String> {
    let min_size = format.component_count() * ty.size_in_bytes();
    if data.len() < min_size {
        return Err("not enough data to fill texture".to_string());
    }
    unsafe {
        gl::TexImage2D(
            target as u32,
            level,
            internal_format as i32,
            width,
            height,
            border,
            format as u32,
            ty as u32,
            data.as_ptr() as *const c_void
        ) 
    };
    check_error()
}

/*#[inline]
pub fn get_tex_image(
    target: TextureTarget, 
    level: i32, 
    format: TextureFormat, 
    ty: PixelType,
    data: &mut Vec<u8>
) -> Result<(), String> {
    let max_size = get_integer_v(0x0D33)? as usize;
    let vec_size = max_size * 4 * 4;

    unsafe {

    }
    check_error()
}*/

#[inline]
pub fn gen_framebuffer() -> Result<u32, String> {
    let mut fbo: u32 = 0;
    unsafe { gl::GenFramebuffers(1, &mut fbo as *mut u32) };
    check_error()?;
    Ok(fbo)
}

#[inline]
pub fn delete_framebuffer(fbo: u32) -> Result<(), String> {
    unsafe { gl::DeleteFramebuffers(1, &fbo as *const u32) };
    check_error()
}

#[inline]
pub fn bind_framebuffer(target: FramebufferTarget, fbo: u32) -> Result<(), String> {
    unsafe { gl::BindFramebuffer(target as u32, fbo) };
    check_error()
}

#[inline]
pub fn draw_buffers(bufs: &[DrawBuffer]) -> Result<(), String> {
    unsafe { gl::DrawBuffers(bufs.len() as i32, bufs.as_ptr() as *const u32) };
    check_error()
}

#[inline]
pub fn read_buffer(buffer: ReadBuffer) -> Result<(), String> {
    unsafe { gl::ReadBuffer(buffer as u32) };
    check_error()
}

#[inline]
pub fn vertex_attrib_pointer(
    index: u32,
    size: i32,
    vertex_type: VertexType,
    normalized: bool,
    stride: i32,
    pointer_offset: usize
) -> Result<(), String> {
    unsafe {
        gl::VertexAttribPointer(
            index,
            size,
            vertex_type as u32,
            normalized as u8,
            stride,
            pointer_offset as *const c_void
        )
    };
    check_error()
}

#[inline]
pub fn enable_vertex_attrib_array(index: u32) -> Result<(), String> {
    unsafe { gl::EnableVertexAttribArray(index) };
    check_error()
}

#[inline]
pub fn disable_vertex_attrib_array(index: u32) -> Result<(), String> {
    unsafe { gl::DisableVertexAttribArray(index) };
    check_error()
}

#[inline]
pub fn gen_buffer() -> Result<u32, String> {
    let mut buf: u32 = 0;
    unsafe { gl::GenBuffers(1, &mut buf as *mut u32) };
    check_error()?;
    Ok(buf)
}

#[inline]
pub fn delete_buffer(buffer: u32) -> Result<(), String> {
    unsafe { gl::DeleteBuffers(1, &buffer as *const u32) };
    check_error()
}

#[inline]
pub fn bind_buffer(target: BufferTarget, buffer: u32) -> Result<(), String> {
    unsafe { gl::BindBuffer(target as u32, buffer) };
    check_error()
}

#[inline]
pub fn gen_vertex_array() -> Result<u32, String> {
    let mut arr: u32 = 0;
    unsafe { gl::GenVertexArrays(1, &mut arr as *mut u32) };
    check_error()?;
    Ok(arr)
}

#[inline]
pub fn delete_vertex_array(arr: u32) -> Result<(), String> {
    unsafe { gl::DeleteVertexArrays(1, &arr as *const u32) };
    check_error()
}

#[inline]
pub fn bind_vertex_array(arr: u32) -> Result<(), String> {
    unsafe { gl::BindVertexArray(arr) };
    check_error()
}

#[inline]
pub fn buffer_data<T>(target: BufferTarget, data: &[T], usage: BufferUsage) -> Result<(), String> {
    unsafe { gl::BufferData(target as u32, data.len() as isize, data.as_ptr() as *const c_void, usage as u32) };
    check_error()
}

#[inline]
pub fn create_shader(shader_type: ShaderType) -> Result<u32, String> {
    let shader = unsafe { gl::CreateShader(shader_type as u32) };
    check_error()?;
    Ok(shader)
}

#[inline]
pub fn delete_shader(shader: u32) -> Result<(), String> {
    unsafe { gl::DeleteShader(shader) };
    check_error()
}

#[inline]
pub fn attach_shader(program: u32, shader: u32) -> Result<(), String> {
    unsafe { gl::AttachShader(program, shader) };
    check_error()
}

#[inline]
pub fn detach_shader(program: u32, shader: u32) -> Result<(), String> {
    unsafe { gl::DetachShader(program, shader) };
    check_error()
}

#[inline]
pub fn shader_source(shader: u32, source: &str) -> Result<(), String> {
    let sources: [*const i8; 1] = [source.as_ptr() as *const i8];
    let lengths: [i32; 1] = [1];
    unsafe { gl::ShaderSource(shader, 1, sources.as_ptr(), lengths.as_ptr()) };
    check_error()
}

#[inline]
pub fn compile_shader(shader: u32) -> Result<(), String> {
    unsafe { gl::CompileShader(shader) };
    check_error()
}

#[inline]
pub fn get_shader(shader: u32, param: ShaderParam) -> Result<i32, String> {
    let mut val: i32 = 0;
    unsafe { gl::GetShaderiv(shader, param as u32, &mut val as *mut i32) };
    check_error()?;
    Ok(val)
}

#[inline]
pub fn get_shader_info_log(shader: u32) -> Result<String, String> {
    let mut len = get_shader(shader, ShaderParam::InfoLogLength)?;
    let mut bytes = vec![0; len as usize];
    unsafe { gl::GetShaderInfoLog(shader, len, &mut len as *mut i32, bytes.as_mut_ptr() as *mut i8) };
    check_error()?;
    let log = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    Ok(log)
}

#[inline]
pub fn create_program() -> Result<u32, String> {
    let program = unsafe { gl::CreateProgram() };
    check_error()?;
    Ok(program)
}

#[inline]
pub fn delete_program(program: u32) -> Result<(), String> {
    unsafe { gl::DeleteProgram(program) };
    check_error()
}

#[inline]
pub fn link_program(program: u32) -> Result<(), String> {
    unsafe { gl::LinkProgram(program); }
    check_error()
}

#[inline]
pub fn get_program(program: u32, param: ProgramParam) -> Result<i32, String> {
    let mut val: i32 = 0;
    unsafe { gl::GetProgramiv(program, param as u32, &mut val as *mut i32) };
    check_error()?;
    Ok(val)
}

#[inline]
pub fn get_program_info_log(program: u32) -> Result<String, String> {
    let mut len = get_program(program, ProgramParam::InfoLogLength)?;
    let mut bytes = vec![0; len as usize];
    unsafe { gl::GetProgramInfoLog(program, len, &mut len as *mut i32, bytes.as_mut_ptr() as *mut i8) };
    check_error()?;
    let log = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    Ok(log)
}

pub struct Uniform {
    size: usize,
    ty: UniformType,
    name: String,
}
impl Uniform {
    pub fn size(&self) -> usize { self.size }
    pub fn ty(&self) -> UniformType { self.ty }
    pub fn name(&self) -> &str { &self.name }
}

#[inline]
pub fn get_active_uniform(program: u32, index: u32) -> Result<Uniform, String> {
    let max_name_len = get_program(program, ProgramParam::ActiveUniformMaxLength)?;
    let mut len: i32 = 0;
    let mut size: i32 = 0;
    let mut ty: u32 = 0;
    let mut name: Vec<u8> = vec![0; max_name_len as usize];
    unsafe {
        gl::GetActiveUniform(
            program,
            index,
            max_name_len,
            &mut len as *mut i32,
            &mut size as *mut i32,
            &mut ty as *mut u32,
            name.as_mut_ptr() as *mut i8
        )
    }
    check_error()?;
    let name = String::from_utf8(name).map_err(|e| e.to_string())?;
    let ty = UniformType::from(ty)?;
    Ok(Uniform {
        size: size as usize,
        ty: ty,
        name: name,
    })
}

#[inline]
pub fn get_uniform_location(program: u32, name: &str) -> Result<i32, String> {
    let loc = unsafe { gl::GetUniformLocation(program, name.as_ptr() as *const i8) };
    check_error()?;
    Ok(loc)
}

#[inline]
pub fn scissor(x: i32, y: i32, w: i32, h: i32) -> Result<(), String> {
    unsafe { gl::Scissor(x, y, w, h) };
    check_error()
}

#[inline]
pub fn viewport(x: i32, y: i32, w: i32, h: i32) -> Result<(), String> {
    unsafe { gl::Viewport(x, y, w, h) };
    check_error()
}

#[inline]
pub fn use_program(program: u32) -> Result<(), String> {
    unsafe { gl::UseProgram(program) };
    check_error()
}

#[inline]
pub fn uniform_1f(location: i32, v0: f32) -> Result<(), String> {
    unsafe { gl::Uniform1f(location, v0) };
    check_error()
}

#[inline]
pub fn uniform_2f(location: i32, v0: f32, v1: f32) -> Result<(), String> {
    unsafe { gl::Uniform2f(location, v0, v1) };
    check_error()
}

#[inline]
pub fn uniform_3f(location: i32, v0: f32, v1: f32, v2: f32) -> Result<(), String> {
    unsafe { gl::Uniform3f(location, v0, v1, v2) };
    check_error()
}

#[inline]
pub fn uniform_4f(location: i32, v0: f32, v1: f32, v2: f32, v3: f32) -> Result<(), String> {
    unsafe { gl::Uniform4f(location, v0, v1, v2, v3) };
    check_error()
}

#[inline]
pub fn uniform_1fv(location: i32, v: &[f32]) -> Result<(), String> {
    unsafe { gl::Uniform1fv(location, v.len() as i32, v.as_ptr() as *const f32) };
    check_error()
}

#[inline]
pub fn uniform_2fv(location: i32, v: &[[f32; 2]]) -> Result<(), String> {
    unsafe { gl::Uniform2fv(location, v.len() as i32, v.as_ptr() as *const f32) };
    check_error()
}

#[inline]
pub fn uniform_3fv(location: i32, v: &[[f32; 3]]) -> Result<(), String> {
    unsafe { gl::Uniform3fv(location, v.len() as i32, v.as_ptr() as *const f32) };
    check_error()
}

#[inline]
pub fn uniform_4fv(location: i32, v: &[[f32; 4]]) -> Result<(), String> {
    unsafe { gl::Uniform4fv(location, v.len() as i32, v.as_ptr() as *const f32) };
    check_error()
}

#[inline]
pub fn uniform_matrix_2f(location: i32, transpose: bool, v: &[f32; 4]) -> Result<(), String> {
    unsafe { gl::UniformMatrix2fv(location, 1, transpose as u8, v.as_ptr() as *const f32) };
    check_error()
}

#[inline]
pub fn uniform_matrix_3f(location: i32, transpose: bool, v: &[f32; 9]) -> Result<(), String> {
    unsafe { gl::UniformMatrix2fv(location, 1, transpose as u8, v.as_ptr() as *const f32) };
    check_error()
}

#[inline]
pub fn uniform_matrix_4f(location: i32, transpose: bool, v: &[f32; 16]) -> Result<(), String> {
    unsafe { gl::UniformMatrix2fv(location, 1, transpose as u8, v.as_ptr() as *const f32) };
    check_error()
}

#[inline]
pub fn uniform_matrix_2fv(location: i32, transpose: bool, v: &[[f32; 4]]) -> Result<(), String> {
    unsafe { gl::UniformMatrix2fv(location, v.len() as i32, transpose as u8, v.as_ptr() as *const f32) };
    check_error()
}

#[inline]
pub fn uniform_matrix_3fv(location: i32, transpose: bool, v: &[[f32; 9]]) -> Result<(), String> {
    unsafe { gl::UniformMatrix3fv(location, v.len() as i32, transpose as u8, v.as_ptr() as *const f32) };
    check_error()
}

#[inline]
pub fn uniform_matrix_4fv(location: i32, transpose: bool, v: &[[f32; 16]]) -> Result<(), String> {
    unsafe { gl::UniformMatrix3fv(location, v.len() as i32, transpose as u8, v.as_ptr() as *const f32) };
    check_error()
}

#[inline]
pub fn draw_elements(mode: PrimitiveType, count: usize, index_type: IndexType) -> Result<(), String> {
    unsafe { gl::DrawElements(mode as u32, count as i32, index_type as u32, std::ptr::null()) };
    check_error()
}