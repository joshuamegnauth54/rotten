//! Enumeration for [glClear](https://docs.gl/gl4/glClear)

#![allow(non_upper_case_globals)]

use crate::context::gl::{self, types::GLenum};
use bitflags::bitflags;

bitflags! {
    #[repr(C)]
    pub struct ClearKind: GLenum {
        const ColorBuffer = gl::COLOR_BUFFER_BIT;
        const DepthBuffer = gl::DEPTH_BUFFER_BIT;
        const StencilBuffer = gl::STENCIL_BUFFER_BIT;
    }
}
