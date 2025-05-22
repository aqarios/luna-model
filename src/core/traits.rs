pub trait ValueByIndex<Idx>
where
    Idx: ?Sized,
{
    type Output: ?Sized;

    // Required method
    fn value_by_index(&self, index: Idx) -> Self::Output;
}
