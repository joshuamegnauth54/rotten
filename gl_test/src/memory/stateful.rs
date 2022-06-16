//! Stateful buffers and other objects that modify global state.

mod buffer;
mod vao;

pub use buffer::ClassicBuffer as Buffer;
pub use vao::VertexArray;
