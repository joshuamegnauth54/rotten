use std::{borrow::Cow, ffi::CString, path::{Path, PathBuf}};

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
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Cow<'static, str>, GlError> {
        unimplemented!()
    }

    pub fn new(gl: &'gl Gl, descriptor: ShaderDescriptor) -> Result<Self, GlError> {
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
