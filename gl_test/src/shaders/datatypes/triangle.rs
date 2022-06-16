use super::Vertex3;
use crate::memory::{GpuData, GpuDataIndices, GpuDataVerts, Layout};
use std::mem::size_of;

/// Indices for element array buffer
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct TriangleIndices {
    indices: [u32; 3],
}

impl GpuData for TriangleIndices {
    type Data = [u32; 3];

    fn as_ptr(&self) -> *const Self::Data {
        std::ptr::addr_of!(self.indices)
    }

    fn size_total(&self) -> usize {
        size_of::<Self::Data>()
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Triangle {
    vertices: [Vertex3; 3],
}

impl Triangle {
    pub fn new(vertices: [Vertex3; 3]) -> Self {
        Self { vertices }
    }
}

impl GpuData for Triangle {
    type Data = [Vertex3; 3];

    fn as_ptr(&self) -> *const Self::Data {
        self.vertices.as_ptr() as _
    }

    fn size_total(&self) -> usize {
        size_of::<Vertex3>() * 3
    }
}

impl GpuDataVerts<2> for Triangle {
    fn stride(&self) -> usize {
        self.vertices[0].stride()
    }

    fn memory_layout(&self) -> [Layout; 2] {
        // Two Layout structs: one for position and another for color
        // Size = 3 for Vertex3 (three f32)
        [
            Layout {
                index: 0,
                size: 3,
                stride: self.stride(),
                start: 0,
            },
            Layout {
                index: 1,
                size: 3,
                stride: self.stride(),
                start: Vertex3::size_position(),
            },
        ]
    }
}

impl GpuDataIndices<TriangleIndices, 2> for Triangle {
    fn indices(&self) -> TriangleIndices {
        TriangleIndices { indices: [0, 1, 2] }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            vertices: [
                Vertex3::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),
                Vertex3::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),
                Vertex3::new([0.0, 0.5, 0.0], [0.0, 0.0, 1.0]),
            ],
        }
    }
}
