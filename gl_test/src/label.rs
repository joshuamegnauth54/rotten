use std::hash::Hash;

pub trait Label {
    type Output: AsRef<str> + Hash;

    fn label(&self) -> Self::Output;
}
