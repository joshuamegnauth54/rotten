//! Enumerations for [glEnable](https://docs.gl/gl4/glEnable).

#![allow(non_upper_case_globals)]

use crate::context::gl::{self, types::GLenum};
use bitflags::bitflags;

bitflags! {
    #[repr(C)]
    pub struct Enable: GLenum {

    }
}
