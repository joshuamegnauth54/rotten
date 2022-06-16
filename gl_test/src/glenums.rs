//! Wrapped OpenGL enumerations.
//!
//! This module contains various OpenGL enumerations, such as buffer targets, wrapped up into type
//! safe structs. The new types implement bitwise operators as well as convenience functions.

mod buffers;
mod clearkind;
mod contextflags;
mod contextprofile;
mod debug;
mod drawmode;
mod enable;
mod geterror;
mod getstring;
mod objects;

pub use buffers::{BufferTarget, BufferUsage};
pub use clearkind::ClearKind;
pub use contextflags::ContextFlags;
pub use contextprofile::ContextProfile;
pub use debug::{DebugSeverity, DebugSource, DebugType};
pub use drawmode::DrawMode;
pub use enable::Enable;
pub use geterror::GetError;
pub use getstring::GetString;
pub use objects::ObjectName;
