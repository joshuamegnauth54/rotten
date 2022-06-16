#![allow(non_upper_case_globals)]

use crate::context::gl::{self, types::GLenum};
use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    #[repr(C)]
    pub struct GetError: GLenum {
        const NoError = gl::NO_ERROR;
        const InvalidEnum = gl::INVALID_ENUM;
        const InvalidValue = gl::INVALID_VALUE;
        const InvalidOperation = gl::INVALID_OPERATION;
        const InvalidFramebufferOperation = gl::INVALID_FRAMEBUFFER_OPERATION;
        const OutOfMemory = gl::OUT_OF_MEMORY;
        const StackUnderflow = gl::STACK_UNDERFLOW;
        const StackOverflow = gl::STACK_OVERFLOW;
    }
}
