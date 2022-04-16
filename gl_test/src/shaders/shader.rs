use std::ffi::CStr;

use crate::{
    gl_support::{
        gl::{self, types::GLuint},
        Gl,
    },
    glerror::GlError,
    id::Id,
};

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum ShaderKind {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
    Spirv = gl::SHADER_BINARY_FORMAT_SPIR_V,
}

//#[derive(Debug)]
pub struct Shader<'gl> {
    gl: &'gl Gl,
    id: GLuint,
    kind: ShaderKind,
}

impl<'gl> Shader<'gl> {
    pub fn from_source(gl: &'gl Gl, source: &CStr, kind: ShaderKind) -> Result<Self, GlError> {
        let id = unsafe { gl.CreateShader(kind as _) };
        unsafe {
            gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl.CompileShader(id);
        }

        let mut success = gl::TRUE as _;
        unsafe {
            gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success != gl::FALSE as _ {
            Ok(Self {gl, id, kind })
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
        unsafe { self.gl.DeleteShader(self.id) }
    }
}
