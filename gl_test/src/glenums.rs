//! Wrapped OpenGL enumerations.
//!
//! This module contains various OpenGL enumerations, such as buffer targets, wrapped up into type
//! safe structs. The new types implement bitwise operators as well as convenience functions.

mod buffers;
mod geterror;

pub use buffers::{BufferTarget, BufferUsage};
pub use geterror::GetError;
