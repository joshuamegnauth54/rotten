//! Enumerations for [glGetString and glGetStringi](https://docs.gl/gl4/glGetString).

#![allow(non_upper_case_globals)]

use crate::context::gl::{self, types::GLenum};
use bitflags::bitflags;

bitflags! {
    #[repr(C)]
    pub struct GetString: GLenum {
        const Vendor = gl::VENDOR;
        const Renderer = gl::RENDERER;
        const Version = gl::VERSION;
        const GlslVersion = gl::SHADING_LANGUAGE_VERSION;
        const Extensions = gl::EXTENSIONS;
    }
}
