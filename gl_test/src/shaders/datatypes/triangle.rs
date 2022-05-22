use super::Vertex3;
use crate::memory::{GpuData, Layout};
use std::mem::size_of;

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

impl GpuData<2> for Triangle {
    type Data = Vertex3;

    fn as_ptr(&self) -> *const Self::Data {
        self.vertices.as_ptr()
    }

    fn stride(&self) -> usize {
        self.vertices[0].stride()
    }

    fn size_total(&self) -> usize {
        size_of::<Vertex3>() * 3
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
