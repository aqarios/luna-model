/// Trait implementing a deep clone. Designed to copy the contents of elements
pub trait DeepClone {
    fn deep_clone(&self) -> Self;
}
