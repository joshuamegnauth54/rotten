use super::{Shader, ShaderDescriptor};
use crate::{
    cleanup::Cleanup,
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
};
use log::info;
use std::rc::Rc;

pub struct ShaderProgram {
    id: GLuint,
    label: Rc<str>,
}

impl ShaderProgram {
    fn new<S>(gl: &Gl, shaders: &[Shader], label: S) -> Result<Self, GlError>
    where
        S: Into<Rc<str>>,
    {
        let label = label.into();
        info!("Creating shader program '{}'", label);

        // Create shader program and attach shaders
        let program = unsafe { gl.CreateProgram() };
        for shader in shaders {
            unsafe { gl.AttachShader(program, shader.id()) }
        }

        // Link shader program
        unsafe { gl.LinkProgram(program) }

        // Handle LinkProgram errors.
        let mut success = gl::TRUE as _;
        unsafe {
            gl.GetProgramiv(program, gl::LINK_STATUS, &mut success);
        }
        if success == gl::FALSE as _ {
            let mut len: GLint = 0;
            unsafe {
                gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            }

            // Retrieve error string from OpenGL
            let error = Gl::create_whitespace_cstring(len as _);
            unsafe {
                gl.GetProgramInfoLog(
                    program,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                )
            }
            // And return the error
            Err(GlError::ShaderProgram(error.to_string_lossy().to_string()))
        } else {
            // Detach shaders so they may be deleted later when dropped.
            for shader in shaders {
                unsafe { gl.DetachShader(program, shader.id()) }
            }

            Ok(Self { id: program, label })
        }
    }

    pub fn from_shaders<'gl, I, S>(
        gl: &'gl mut Gl,
        shaders: &[Shader],
        label: S,
    ) -> Result<&'gl Self, GlError>
    where
        I: IntoIterator<Item = Shader<'gl>>,
        S: Into<Rc<str>>,
    {
        let program = ShaderProgram::new(gl, shaders, label)?;
        Ok(gl.insert_shader(program))
    }

    pub fn from_raw<S, I>(gl: &mut Gl, raw_shaders: I, label: S) -> Result<&Self, GlError>
    where
        S: Into<Rc<str>>,
        I: IntoIterator<Item = ShaderDescriptor>,
    {
        let program = {
            // Compile all of the shaders from ShaderDescriptors
            // Shader has Drop implemented so compiled shaders are cleaned up if one of the compilations fail
            let shaders: Vec<_> = raw_shaders
                .into_iter()
                .map(|descriptor| Shader::new(gl, descriptor))
                .collect::<Result<_, _>>()?;
            ShaderProgram::new(gl, &shaders, label)?
        };
        Ok(gl.insert_shader(program))
    }

    pub fn set_used(&self, gl: &Gl) {
        unsafe { gl.UseProgram(self.id) }
    }
}

impl Id for ShaderProgram {
    fn id(&self) -> GLuint {
        self.id
    }
}

impl Label for ShaderProgram {
    type Output = Rc<str>;

    fn label(&self) -> Self::Output {
        self.label.clone()
    }
}

impl Cleanup for ShaderProgram {
    fn cleanup(&self, gl: &Gl) {
        unsafe { gl.DeleteProgram(self.id()) }
    }
}
