use std::mem::size_of;

use crate::{
    memory::{GpuData, GpuDataIndices, GpuDataVerts, Layout},
    shaders::datatypes::Vertex3,
};

/// Indices for element array buffers.
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct RectangleIndices {
    indices: [u32; 6],
}

impl GpuData for RectangleIndices {
    type Data = [u32; 6];

    fn size_total(&self) -> usize {
        size_of::<Self::Data>()
    }

    fn as_ptr(&self) -> *const Self::Data {
        std::ptr::addr_of!(self.indices)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Rectangle {
    vertices: [Vertex3; 4],
}

impl Rectangle {
    pub fn new(vertices: [Vertex3; 4]) -> Self {
        Self { vertices }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            vertices: [
                // Top right
                Vertex3::new([0.5, 0.5, 0.0], [1.0, 0.0, 0.0]),
                // Bottom right (hypotenuse)
                Vertex3::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),
                // Top left (hypotenuse)
                Vertex3::new([-0.5, 0.5, 0.0], [0.0, 0.0, 1.0]),
                // Bottom left
                Vertex3::new([-0.5, -0.5, 0.0], [0.25, 0.25, 0.25]),
            ],
        }
    }
}

impl GpuData for Rectangle {
    type Data = [Vertex3; 4];

    fn as_ptr(&self) -> *const Self::Data {
        std::ptr::addr_of!(self.vertices)
    }

    fn size_total(&self) -> usize {
        size_of::<Self::Data>()
    }
}

impl GpuDataVerts<2> for Rectangle {
    fn stride(&self) -> usize {
        self.vertices[0].stride()
    }

    fn memory_layout(&self) -> [Layout; 2] {
        [
            Layout {
                index: 0,
                size: 4,
                stride: self.stride(),
                start: 0,
            },
            Layout {
                index: 1,
                size: 4,
                stride: self.stride(),
                start: Vertex3::size_position(),
            },
        ]
    }
}

impl GpuDataIndices<RectangleIndices, 2> for Rectangle {
    fn indices(&self) -> RectangleIndices {
        RectangleIndices {
            indices: [0, 1, 2, 1, 2, 3],
        }
    }
}
