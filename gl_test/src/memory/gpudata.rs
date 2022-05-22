use super::Layout;

pub trait GpuData<const N: usize> {
    type Data;

    #[must_use]
    fn as_ptr(&self) -> *const Self::Data;
    fn size_total(&self) -> usize;
    fn stride(&self) -> usize;
    fn memory_layout(&self) -> [Layout; N];
}
