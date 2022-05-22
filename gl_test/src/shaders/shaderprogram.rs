use super::{Shader, ShaderDescriptor};
use crate::{
    gl_support::{
        gl::{
            self,
            types::{GLint, GLuint},
        },
        Gl,
    },
    glerror::GlError,
    label::Label,
};
use log::{error, info};
use std::rc::Rc;

pub struct ShaderProgram {
    gl: Rc<Gl>,
    id: GLuint,
    label: Rc<str>,
}

impl ShaderProgram {
    fn new<S>(gl: Rc<Gl>, shaders: &[Shader], label: S) -> Result<Self, GlError>
    where
        S: Into<Rc<str>>,
    {
        let label = label.into();
        info!("Creating shader program '{}'", label);

        // Create shader program
        let program = unsafe { gl.CreateProgram() };

        // CreateProgram returns 0 on errors, but errors only occur if something is really broken (like a Context error).
        if program == 0 {
            error!("CreateProgram failed to generate an object id");
            Err(GlError::ShaderProgram(
                "CreateProgram returned an object id of 0".into(),
            ))?
        }

        // Attach each shader to program
        for shader in shaders {
            info!("Attaching {} shader", shader.kind());
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

            Ok(Self {
                gl,
                id: program,
                label,
            })
        }
    }

    pub fn from_shaders<S>(gl: Rc<Gl>, shaders: &[Shader], label: S) -> Result<Self, GlError>
    where
        S: Into<Rc<str>>,
    {
        ShaderProgram::new(gl, shaders, label)
    }

    pub fn from_raw<S, I>(gl: Rc<Gl>, raw_shaders: I, label: S) -> Result<Self, GlError>
    where
        S: Into<Rc<str>>,
        I: IntoIterator<Item = ShaderDescriptor>,
    {
        let program = {
            // Compile all of the shaders from ShaderDescriptors
            // Shader has Drop implemented so compiled shaders are cleaned up if one of the compilations fail
            let shaders: Vec<_> = raw_shaders
                .into_iter()
                .map(|descriptor| Shader::new(gl.clone(), descriptor))
                .collect::<Result<_, _>>()?;
            ShaderProgram::new(gl, &shaders, label)?
        };
        Ok(program)
    }

    pub fn set_used(&self) {
        unsafe { self.gl.UseProgram(self.id) }
    }
}

impl Label for ShaderProgram {
    type Output = Rc<str>;

    fn label(&self) -> Self::Output {
        self.label.clone()
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteProgram(self.id) }
    }
}
