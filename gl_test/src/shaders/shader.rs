use std::{
    borrow::Cow,
    ffi::CString,
    fmt,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    rc::Rc,
};

use log::{error, info};

use crate::{
    context::{
        gl::{self, types::GLuint},
        Gl,
    },
    glerror::GlError,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ShaderKind {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
    Geometry = gl::GEOMETRY_SHADER,
    Spirv = gl::SHADER_BINARY_FORMAT_SPIR_V,
}

impl fmt::Display for ShaderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ShaderKind::*;
        match *self {
            Vertex => write!(f, "Vertex"),
            Fragment => write!(f, "Fragment"),
            Geometry => write!(f, "Geometry"),
            Spirv => write!(f, "SPIR-V"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShaderFrom {
    // Shader source code
    Source(Cow<'static, str>),
    // File containing shader source
    FilePath(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShaderDescriptor {
    pub kind: ShaderKind,
    pub from: ShaderFrom,
}

pub struct Shader {
    gl: Rc<Gl>,
    id: GLuint,
    kind: ShaderKind,
}

impl Shader {
    /// Helper function to load shader source code
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Cow<'static, str>, GlError> {
        let path = path.as_ref();
        let path_name = path.to_string_lossy();
        info!("Loading shader from file: {}", path_name);

        // Open source code file.
        let file = File::open(path).map_err(|e| {
            let error = format!("{e}\nPath: {}", path_name);
            GlError::Shader(error)
        })?;
        // Try to create an empty string with the size of the file
        let mut source = match file.metadata() {
            Ok(metadata) => String::with_capacity(metadata.len() as _),
            Err(e) => {
                error!(
                    "Failed retrieving file metadata for shader source.\nError: {}",
                    e
                );
                String::new()
            }
        };
        // And make the reader buffered
        let mut buf_reader = BufReader::new(file);

        // Read file to String
        buf_reader
            .read_to_string(&mut source)
            .map_err(|e| GlError::Shader(e.to_string()))?;

        Ok(source.into())
    }

    /// Compile a shader from source.
    pub(super) fn new(gl: Rc<Gl>, descriptor: ShaderDescriptor) -> Result<Self, GlError> {
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
        if id == 0 {
            error!("CreateShader failed to generate an object id");
            return Err(GlError::Shader(
                "CreateShader returned an object id of 0".into(),
            ));
        }

        // Compile sauce
        unsafe {
            // Copy shader source to object referenced by id.
            // Dropping the CString that stores the source is entirely safe.
            // Length: ShaderSource doesn't need the actual length because CStrings are null terminated.
            // https://docs.gl/gl4/glShaderSource
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

    /// Return OpenGL object id.
    pub(super) fn id(&self) -> GLuint {
        // This function is mostly for [ShaderProgram] so I don't have to make the struct member pub(super)
        self.id
    }

    /// Type of shader
    pub fn kind(&self) -> ShaderKind {
        self.kind
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        // Signals that a shader may be deleted but does not delete if attached to a program.
        // Shaders do not need to stay attached to a program after linking.
        // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteShader.xhtml
        unsafe { self.gl.DeleteShader(self.id) }
    }
}
