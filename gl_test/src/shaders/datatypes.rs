use crate::memory::{GpuData, Layout};
use std::mem::size_of;

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Vertex<const P: usize, const C: usize> {
    position: [f32; P],
    color: [f32; C],
}

pub type Vertex3 = Vertex<3, 3>;

impl<const P: usize, const C: usize> Vertex<P, C> {
    pub fn new(position: [f32; P], color: [f32; C]) -> Self {
        Self { position, color }
    }

    /// Size of the position component
    #[inline]
    pub const fn size_position() -> usize {
        size_of::<f32>() * P
    }

    /// Size of the color component
    #[inline]
    pub const fn size_color() -> usize {
        size_of::<f32>() * C
    }

    /// Stride value of all components
    #[inline]
    pub const fn stride() -> usize {
        size_of::<f32>() * (P + C)
    }

    #[inline]
    pub fn position(&self) -> &[f32; P] {
        &self.position
    }

    #[inline]
    pub fn color(&self) -> &[f32; C] {
        &self.color
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

impl GpuData<2> for Triangle {
    fn as_ptr<Vertex3>(&self) -> *const Vertex3 {
        self.vertices.as_ptr() as _
    }

    fn stride(&self) -> usize {
        Vertex3::stride()
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
