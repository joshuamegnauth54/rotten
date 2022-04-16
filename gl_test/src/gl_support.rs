use glutin::{Context, PossiblyCurrent};
use std::{
    ffi::{CStr, CString},
    ops::{BitOr, Deref},
};

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[allow(unused)]
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum DebugSeverity {
    DontCare = gl::DONT_CARE,
    High = gl::DEBUG_SEVERITY_HIGH,
    Medium = gl::DEBUG_SEVERITY_MEDIUM,
    Low = gl::DEBUG_SEVERITY_LOW,
    Notification = gl::DEBUG_SEVERITY_NOTIFICATION,
}

impl DebugSeverity {
    pub fn all() -> u32 {
        DebugSeverity::High
            | DebugSeverity::Medium
            | DebugSeverity::Low
            | DebugSeverity::Notification
    }
}

impl Default for DebugSeverity {
    fn default() -> Self {
        DebugSeverity::DontCare
    }
}

impl From<u32> for DebugSeverity {
    fn from(other: u32) -> Self {
        match other {
            gl::DEBUG_SEVERITY_HIGH => DebugSeverity::High,
            gl::DEBUG_SEVERITY_MEDIUM => DebugSeverity::Medium,
            gl::DEBUG_SEVERITY_LOW => DebugSeverity::Low,
            gl::DEBUG_SEVERITY_NOTIFICATION => DebugSeverity::Notification,
            _ => DebugSeverity::DontCare,
        }
    }
}

impl BitOr<DebugSeverity> for DebugSeverity {
    type Output = u32;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    }
}

impl BitOr<DebugSeverity> for u32 {
    type Output = u32;

    fn bitor(self, rhs: DebugSeverity) -> Self::Output {
        self | rhs as u32
    }
}

#[allow(unused)]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DebugSource {
    DontCare = gl::DONT_CARE,
    Api = gl::DEBUG_SOURCE_API,
    WindowSystem = gl::DEBUG_SOURCE_WINDOW_SYSTEM,
    ShaderCompiler = gl::DEBUG_SOURCE_SHADER_COMPILER,
    ThirdParty = gl::DEBUG_SOURCE_THIRD_PARTY,
    Application = gl::DEBUG_SOURCE_APPLICATION,
    Other = gl::DEBUG_SOURCE_OTHER,
}

impl DebugSource {
    pub fn all() -> u32 {
        DebugSource::Api
            | DebugSource::WindowSystem
            | DebugSource::ShaderCompiler
            | DebugSource::Application
            | DebugSource::Other
    }
}

impl Default for DebugSource {
    fn default() -> Self {
        DebugSource::DontCare
    }
}

impl From<u32> for DebugSource {
    fn from(other: u32) -> Self {
        match other {
            gl::DEBUG_SOURCE_API => DebugSource::Api,
            gl::DEBUG_SOURCE_SHADER_COMPILER => DebugSource::WindowSystem,
            gl::DEBUG_SOURCE_THIRD_PARTY => DebugSource::ThirdParty,
            gl::DEBUG_SOURCE_APPLICATION => DebugSource::Application,
            gl::DEBUG_SOURCE_OTHER => DebugSource::Other,
            _ => DebugSource::DontCare,
        }
    }
}

impl BitOr<DebugSource> for DebugSource {
    type Output = u32;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    }
}

impl BitOr<DebugSource> for u32 {
    type Output = u32;

    fn bitor(self, rhs: DebugSource) -> Self::Output {
        self as u32 | rhs as u32
    }
}

#[allow(unused)]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DebugType {
    DontCare = gl::DONT_CARE,
    Error = gl::DEBUG_TYPE_ERROR,
    Deprecated = gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR,
    Undefined = gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR,
    Portability = gl::DEBUG_TYPE_PORTABILITY,
    Performance = gl::DEBUG_TYPE_PERFORMANCE,
    Marker = gl::DEBUG_TYPE_MARKER,
    Push = gl::DEBUG_TYPE_PUSH_GROUP,
    Pop = gl::DEBUG_TYPE_POP_GROUP,
    Other = gl::DEBUG_TYPE_OTHER,
}

impl Default for DebugType {
    fn default() -> Self {
        DebugType::DontCare
    }
}

impl BitOr<DebugType> for DebugType {
    type Output = u32;

    fn bitor(self, rhs: DebugType) -> Self::Output {
        self as u32 | rhs as u32
    }
}

impl BitOr<DebugType> for u32 {
    type Output = u32;

    fn bitor(self, rhs: DebugType) -> Self::Output {
        self | rhs as u32
    }
}

impl From<u32> for DebugType {
    fn from(other: u32) -> Self {
        match other {
            gl::DEBUG_TYPE_ERROR => DebugType::Error,
            gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => DebugType::Deprecated,
            gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => DebugType::Undefined,
            gl::DEBUG_TYPE_PORTABILITY => DebugType::Portability,
            gl::DEBUG_TYPE_PERFORMANCE => DebugType::Performance,
            gl::DEBUG_TYPE_MARKER => DebugType::Marker,
            gl::DEBUG_TYPE_PUSH_GROUP => DebugType::Push,
            gl::DEBUG_TYPE_POP_GROUP => DebugType::Pop,
            gl::DEBUG_TYPE_OTHER => DebugType::Other,
            _ => DebugType::DontCare,
        }
    }
}

impl DebugType {
    pub fn all() -> u32 {
        DebugType::Error
            | DebugType::Deprecated
            | DebugType::Undefined
            | DebugType::Portability
            | DebugType::Performance
            | DebugType::Marker
            | DebugType::Push
            | DebugType::Pop
            | DebugType::Other
    }
}

//#[derive(Debug)]
pub struct Gl {
    inner: gl::Gl,
}

impl Gl {
    /// Load OpenGL function pointers.
    pub fn load_gl(gl_context: &Context<PossiblyCurrent>) -> Self {
        let inner = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
        Self { inner }
    }

    /// Creates a CString consisting of all whitespace with size len + 1
    pub fn create_whitespace_cstring(len: usize) -> CString {
        let buffer = vec![b' '; len + 1];
        unsafe { CString::from_vec_unchecked(buffer) }
    }

    pub unsafe fn register_debug_callback(&self) {
        self.Enable(gl::DEBUG_OUTPUT | gl::DEBUG_OUTPUT_SYNCHRONOUS);
        self.DebugMessageCallback(Some(Gl::gl_debug_callback), std::ptr::null_mut());
    }

    pub fn debug_message_control(
        &self,
        debug_source: DebugSource,
        debug_type: DebugType,
        debug_severity: DebugSeverity,
    ) {
        unsafe {
            self.DebugMessageControl(
                debug_source as u32,
                debug_type as u32,
                debug_severity as u32,
                0,
                std::ptr::null(),
                gl::TRUE,
            )
        }
    }

    extern "system" fn gl_debug_callback(
        source: gl::types::GLenum,
        type_e: gl::types::GLenum,
        id: gl::types::GLuint,
        severity: gl::types::GLenum,
        _length: gl::types::GLsizei,
        message: *const gl::types::GLchar,
        _user: *mut std::ffi::c_void,
    ) {
        let message = unsafe { CStr::from_ptr(message).to_string_lossy() };
        let source: DebugSource = source.into();
        let type_e: DebugType = type_e.into();
        let severity: DebugSeverity = severity.into();

        println!("[{id}: {severity:?}][{source:?}: {type_e:?}] - {message}")
    }
}

// Implementing Deref for Gl makes it a million times less annoying to use the inner Gl struct.
// Also...the Nercury tutorial does so too.
impl Deref for Gl {
    type Target = gl::Gl;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}