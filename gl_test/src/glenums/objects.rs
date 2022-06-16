//! OpenGL object names for use in functions such as [glObjectLabel](https://docs.gl/gl4/glObjectLabel)

#![allow(non_upper_case_globals)]

use crate::context::gl::{self, types::GLenum};
use bitflags::bitflags;

bitflags! {
    #[repr(C)]
    pub struct ObjectName: GLenum {
        const Buffer = gl::BUFFER;
        const Shader = gl::SHADER;
        const Program = gl::PROGRAM;
        const VertexArray = gl::VERTEX_ARRAY;
        const Query = gl::QUERY;
        const ProgramPipeline = gl::PROGRAM_PIPELINE;
        const TransformFeedback = gl::TRANSFORM_FEEDBACK;
        const Sampler = gl::SAMPLER;
        const Texture = gl::TEXTURE;
        const Renderbuffer = gl::RENDERBUFFER;
        const Framebuffer = gl::FRAMEBUFFER;
    }
}
