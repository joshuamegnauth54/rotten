//! Wrappers around OpenGL's memory management API.

mod gpudata;
mod layout;
pub mod stateful;

pub use gpudata::{GpuData, GpuDataIndices, GpuDataVerts};
pub use layout::Layout;
