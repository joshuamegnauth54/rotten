pub mod datatypes;
mod shader;
mod shaderprogram;

pub(super) use shader::Shader;
pub use shader::{ShaderDescriptor, ShaderFrom, ShaderKind};
pub use shaderprogram::ShaderProgram;
