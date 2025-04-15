pub trait IndexByValue<Idx>
where
    Idx: ?Sized,
{
    type Output: ?Sized;

    // Required method
    fn index_by_value(&self, index: Idx) -> Self::Output;
}
