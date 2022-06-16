#![allow(non_upper_case_globals)]

use crate::context::gl::{self, types::GLenum};
use bitflags::bitflags;

bitflags! {
    #[repr(C)]
    pub struct ContextFlags: GLenum {
        const ForwardCompatible = gl::CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT;
        const Debug = gl::CONTEXT_FLAG_DEBUG_BIT;
        const RobustAccess = gl::CONTEXT_FLAG_ROBUST_ACCESS_BIT;
        const NoError = gl::CONTEXT_FLAG_NO_ERROR_BIT;
    }
}
