use self::gl::types::{GLboolean, GLchar, GLenum, GLsizei, GLuint};
use super::{Rect, Size};
use crate::glenums::{ClearKind, DebugSeverity, DebugSource, DebugType, DrawMode, GetString};
use log::{error, info, warn};
use std::{
    borrow::Cow,
    ffi::{c_void, CStr, CString},
    ops::Deref,
    rc::Rc,
};

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Clone)]
pub struct Gl {
    context: gl::Gl,
}

impl Gl {
    /// Load OpenGL function pointers.
    pub fn load_gl<F>(gl_loader: F) -> Rc<Self>
    where
        F: FnMut(&'static str) -> *const c_void,
    {
        let context = gl::Gl::load_with(gl_loader);
        Self { context }.into()
    }

    /// Insert compiled shaders
    /*pub fn insert_shader(&mut self, program: ShaderProgram) -> &ShaderProgram {
        let label = program.label();
        self.shaders.insert(label.clone(), program);

        self.get_shader(&label).unwrap_or_else(|| {
            panic!(
                "Impossible: Shader doesn't exist despite just being added. Label: {}",
                &label
            )
        })
    }

    /// Retrieve shader program by name
    pub fn get_shader(&self, name: &str) -> Option<&ShaderProgram> {
        self.shaders.get(name)
    }*/

    pub fn get_string<'gl>(
        &'gl self,
        what: GetString,
        index: Option<usize>,
    ) -> Option<Cow<'gl, str>> {
        // GetString::Extensions requires an index of which string to retrieve while the rest of
        // the enumerations don't
        let get_str = match what {
            GetString::Vendor
            | GetString::Renderer
            | GetString::Version
            | GetString::GlslVersion => unsafe { self.GetString(what.bits()) },
            GetString::Extensions => {
                if let Some(index) = index {
                    unsafe { self.GetStringi(what.bits(), index as _) }
                } else {
                    std::ptr::null()
                }
            }
            _ => unreachable!(),
        };

        // GetString may return a null pointer on error
        if get_str == std::ptr::null() {
            None
        } else {
            let get_str = unsafe { CStr::from_ptr(get_str as _) };
            Some(get_str.to_string_lossy())
        }
    }

    pub fn viewport(&self, rect: Rect) {
        unsafe {
            // Viewport = actual viewing area. Multiple viewports are allowed which is useful for
            // splitscreen.
            self.Viewport(
                rect.x as _,
                rect.y as _,
                rect.size.width as _,
                rect.size.height as _,
            );
        }
    }

    /// Maximum supported viewport dimensions
    pub fn viewport_max(&self) -> Size {
        let mut data = [0; 2];
        unsafe {
            // Writes width and height in that order into the buffer
            self.GetIntegerv(gl::MAX_VIEWPORT_DIMS, data.as_mut_ptr());
        }

        Size {
            width: data[0] as _,
            height: data[1] as _,
        }
    }

    /// Clear current buffer(s) with values set by `super::Clear`
    pub fn clear(&self, clear: ClearKind) {
        unsafe { self.Clear(clear.bits()) }
    }

    pub fn draw_elements(&self, mode: DrawMode, count: u32, offset: u32) {
        unsafe {
            self.DrawElements(mode.bits(), count as _, gl::UNSIGNED_INT, offset as _);
        }
    }

    /// Creates a CString consisting of all whitespace with size len + 1
    pub fn create_whitespace_cstring(len: usize) -> CString {
        let buffer = vec![b' '; len + 1];
        unsafe { CString::from_vec_unchecked(buffer) }
    }

    pub fn enable_debug_output(&self) {
        unsafe {
            self.Enable(gl::DEBUG_OUTPUT);
            self.Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            self.DebugMessageCallback(Some(Gl::gl_debug_callback), std::ptr::null_mut());
        }
    }

    /// Enable or disable specific debug message classes
    pub fn debug_message_control(
        &self,
        debug_source: DebugSource,
        debug_type: DebugType,
        debug_severity: DebugSeverity,
        enable: bool,
    ) {
        unsafe {
            self.DebugMessageControl(
                debug_source.bits(),
                debug_type.bits(),
                debug_severity.bits(),
                0,
                std::ptr::null(),
                enable as GLboolean,
            )
        }
    }

    /// Default debug message handler.
    extern "system" fn gl_debug_callback(
        source: GLenum,
        debug_type: GLenum,
        id: GLuint,
        severity: GLenum,
        _length: GLsizei,
        message: *const GLchar,
        _user: *mut c_void,
    ) {
        // Wrap C types
        let message = unsafe { CStr::from_ptr(message).to_string_lossy() };
        let source = DebugSource::from_bits(source).unwrap_or_default();
        let debug_type = DebugType::from_bits(debug_type).unwrap_or_default();
        let severity = DebugSeverity::from_bits(severity).unwrap_or_default();

        let debug_str = format!("[{id}]: [{source}; {debug_type}] - {message}");

        match severity {
            DebugSeverity::High => error!("{debug_str}"),
            DebugSeverity::Medium => warn!("{debug_str}"),
            DebugSeverity::Low => warn!("{debug_str}"),
            DebugSeverity::Notification => info!("{debug_str}"),
            _ => unreachable!("Undefined flag passed by OpenGL for DebugSeverity"),
        }
    }
}

// Implementing Deref for Gl makes it a million times less annoying to use the inner Gl struct.
// Also...the Nercury tutorial does so too.
impl Deref for Gl {
    type Target = gl::Gl;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}
