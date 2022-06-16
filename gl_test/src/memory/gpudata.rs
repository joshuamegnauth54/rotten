use super::Layout;

// Trait for a bytes array that can be copied into a GPU buffer.
//
// [glBufferData](https://docs.gl/gl4/glBufferData)
pub trait GpuData {
    type Data;

    #[must_use]
    fn as_ptr(&self) -> *const Self::Data;
    fn size_total(&self) -> usize;
}

// Trait for a bytes array containing vertices.
pub trait GpuDataVerts<const LN: usize>: GpuData {
    fn stride(&self) -> usize;
    fn memory_layout(&self) -> [Layout; LN];
}

// Trait for returning indices to use for an index buffer.
pub trait GpuDataIndices<D, const LN: usize>: GpuDataVerts<LN>
where
    D: GpuData,
{
    fn indices(&self) -> D;
}
