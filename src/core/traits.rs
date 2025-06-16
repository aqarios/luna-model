pub trait ValueByIndex<Idx>
where
    Idx: ?Sized,
{
    type Output: ?Sized;

    // Required method
    fn value_by_index(&self, index: Idx) -> Self::Output;
}

pub trait ContentEquality {
    fn is_equal_contents(&self, other: &Self) -> bool;
}

impl<T> ContentEquality for Vec<T>
where
    T: ContentEquality,
{
    fn is_equal_contents(&self, other: &Self) -> bool {
        self.iter().zip(other).all(|(l, r)| l.is_equal_contents(r))
    }
}
