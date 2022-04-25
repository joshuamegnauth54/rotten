use std::{
    borrow::Cow,
    ffi::CString,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use crate::{
    gl_support::{
        gl::{self, types::GLuint},
        Gl,
    },
    glerror::GlError,
    id::Id,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ShaderKind {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
    Geometry = gl::GEOMETRY_SHADER,
    Spirv = gl::SHADER_BINARY_FORMAT_SPIR_V,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShaderFrom {
    Source(Cow<'static, str>),
    FilePath(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShaderDescriptor {
    pub kind: ShaderKind,
    pub from: ShaderFrom,
}

pub struct Shader<'gl> {
    gl: &'gl Gl,
    id: GLuint,
    kind: ShaderKind,
}

impl<'gl> Shader<'gl> {
    /// Helper function to load shader source code
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Cow<'static, str>, GlError> {
        let file = File::open(path.as_ref()).map_err(|e| {
            let error = format!("{e}\nPath: {}", path.as_ref().to_string_lossy());
            GlError::Shader(error)
        })?;
        let mut buf_reader = BufReader::new(file);

        let mut source = String::new();
        buf_reader
            .read_to_string(&mut source)
            .map_err(|e| GlError::Shader(e.to_string()))?;

        Ok(source.into())
    }

    /// Compile a shader from source.
    pub(super) fn new(gl: &'gl Gl, descriptor: ShaderDescriptor) -> Result<Self, GlError> {
        // Load shader source code from file if necessary.
        let source = match descriptor.from {
            ShaderFrom::FilePath(path) => Shader::from_file(path)?,
            ShaderFrom::Source(source) => source,
        };
        // Convert source to a CString for FFI
        let source = CString::new(&*source)
            .map_err(|_| GlError::Shader("Invalid CString from shader source".to_string()))?;

        // Create shader object and compile source
        let id = unsafe { gl.CreateShader(descriptor.kind as _) };
        unsafe {
            gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl.CompileShader(id);
        }

        // Check for compile status errors
        let mut success = gl::TRUE as _;
        unsafe {
            gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success != gl::FALSE as _ {
            Ok(Self {
                gl,
                id,
                kind: descriptor.kind,
            })
        } else {
            // Retrieve error string from OpenGL if compilation failed
            let mut len: gl::types::GLint = 0;
            unsafe { gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len) };
            let error = Gl::create_whitespace_cstring(len as usize);
            unsafe {
                gl.GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                )
            };
            Err(GlError::Shader(error.to_string_lossy().into_owned()))
        }
    }

    pub fn kind(&self) -> ShaderKind {
        self.kind
    }
}

impl Id for Shader<'_> {
    fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader<'_> {
    fn drop(&mut self) {
        // Signals that a shader may be deleted but does not delete if attached to a program.
        // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteShader.xhtml
        unsafe { self.gl.DeleteShader(self.id) }
    }
}
