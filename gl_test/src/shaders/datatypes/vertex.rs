use crate::memory::{GpuData, GpuDataVerts, Layout};
use std::mem::size_of;

/// Colored vertex.
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Vertex<const P: usize, const C: usize> {
    position: [f32; P],
    color: [f32; C],
}

/// Convenience type for a 3D vertex with RGB colors.
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

    // This is apparently unsafe because the struct's members are not aligned.
    /*
    #[inline]
    pub fn position(&self) -> &[f32; P] {
        &self.position
    }

    #[inline]
    pub fn color(&self) -> &[f32; C] {
        &self.color
    }*/
}

impl<const P: usize, const C: usize> GpuData for Vertex<P, C> {
    type Data = f32;

    fn as_ptr(&self) -> *const Self::Data {
        std::ptr::addr_of!(*self) as _
    }

    fn size_total(&self) -> usize {
        self.stride()
    }
}

impl<const P: usize, const C: usize> GpuDataVerts<2> for Vertex<P, C> {
    /// Stride value of all components
    #[inline]
    fn stride(&self) -> usize {
        size_of::<f32>() * (P + C)
    }

    fn memory_layout(&self) -> [Layout; 2] {
        // I think stride is ignored here since this is a single vertex.
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
                start: Self::size_position(),
            },
        ]
    }
}
