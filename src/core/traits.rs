pub trait IndexCopy<Idx>
where
    Idx: ?Sized,
{
    type Output: ?Sized;

    // Required method
    fn index_copy(&self, index: Idx) -> Self::Output;
}
