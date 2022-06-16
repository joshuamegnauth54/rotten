#![allow(non_upper_case_globals)]

use crate::context::gl::{self, types::GLenum};
use bitflags::bitflags;

bitflags! {
    #[repr(C)]
    pub struct ContextProfile: GLenum {
        const Core = gl::CONTEXT_CORE_PROFILE_BIT;
        const Compatibility = gl::CONTEXT_COMPATIBILITY_PROFILE_BIT;
    }
}
