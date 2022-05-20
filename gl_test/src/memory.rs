//! Wrappers around OpenGL's memory management API.

mod classicbuf;
mod classicvao;
mod gpudata;
mod layout;

pub use classicbuf::ClassicBuffer;
pub use classicvao::ClassicVao;
pub use gpudata::GpuData;
pub use layout::Layout;
