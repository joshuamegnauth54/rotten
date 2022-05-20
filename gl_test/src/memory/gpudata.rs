use super::Layout;

pub trait GpuData<const N: usize> {
    fn as_ptr<D>(&self) -> *const D;
    fn size_total(&self) -> usize;
    fn stride(&self) -> usize;
    fn memory_layout(&self) -> [Layout; N];
}
