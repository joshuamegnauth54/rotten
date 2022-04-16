use crate::{
    gl::{
        self,
        types::{GLint, GLuint},
    },
    gl_support::Gl,
    glerror::GlError,
    id::Id,
    shaders::shader::Shader,
};

pub struct ShaderProgram<'gl> {
    gl: &'gl Gl,
    id: GLuint,
}

impl<'gl> ShaderProgram<'gl> {
    pub fn from_shaders(gl: &'gl Gl, shaders: &[Shader]) -> Result<Self, GlError> {
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

            Ok(Self { gl, id: program })
        }
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

impl Drop for ShaderProgram<'_> {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteProgram(self.id) }
    }
}
