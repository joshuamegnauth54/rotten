mod bindings;
mod clear;
pub mod info;
mod rect;

pub use bindings::{gl, Gl};
pub use clear::{Clear, Color};
pub use rect::{Rect, Size};
