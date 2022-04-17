use std::ffi::CStr;

use crate::{
    gl_support::{
        gl::{
            self,
            types::{GLint, GLuint},
        },
        Gl,
    },
    glerror::GlError,
    id::Id,
    label::Label,
    shaders::shader::{Shader, ShaderKind},
};

pub struct ShaderProgram<'gl> {
    gl: &'gl Gl<'gl>,
    id: GLuint,
    label: String,
}

impl<'gl> ShaderProgram<'gl> {
    pub fn from_shaders<S: Into<String>>(
        gl: &'gl Gl,
        shaders: &[Shader],
        label: S,
    ) -> Result<Self, GlError> {
        let program = unsafe { gl.CreateProgram() };
        for shader in shaders {
            unsafe { gl.AttachShader(program, shader.id()) }
        }

        unsafe { gl.LinkProgram(program) }
        let mut success = gl::TRUE as _;
        unsafe {
            gl.GetProgramiv(program, gl::LINK_STATUS, &mut success);
        }

        // Handle LinkProgram errors.
        if success == gl::FALSE as _ {
            let mut len: GLint = 0;
            unsafe {
                gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = Gl::create_whitespace_cstring(len as _);
            unsafe {
                gl.GetProgramInfoLog(
                    program,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                )
            }
            Err(GlError::ShaderProgram(error.to_string_lossy().to_string()))
        } else {
            for shader in shaders {
                unsafe { gl.DetachShader(program, shader.id()) }
            }

            Ok(Self {
                gl,
                id: program,
                label: label.into(),
            })
        }
    }

    pub fn from_raw<S: Into<String>>(
        gl: &'gl Gl,
        raw_shaders: &[(&CStr, ShaderKind)],
        label: S,
    ) -> Result<Self, GlError> {
        let shaders: Vec<_> = raw_shaders
            .into_iter()
            .map(|&(source, kind)| Shader::from_source(gl, source, kind))
            .collect::<Result<_, _>>()?;
        Ok(ShaderProgram::from_shaders(gl, &shaders, label)?)
    }

    pub fn set_used(&self) {
        unsafe { self.gl.UseProgram(self.id) }
    }
}

impl Id for ShaderProgram<'_> {
    fn id(&self) -> GLuint {
        self.id
    }
}

impl Label for ShaderProgram<'_> {
    fn label(&self) -> &str {
        &self.label
    }
}

impl Drop for ShaderProgram<'_> {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteProgram(self.id) }
    }
}
