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

impl<T: ContentEquality + ?Sized> ContentEquality for &T {
    fn is_equal_contents(&self, other: &&T) -> bool {
        (*self).is_equal_contents(*other)
    }
}

pub trait Editable {
    fn edit<F>(mut self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(&mut Self),
    {
        f(&mut self);
        self
    }
}

pub trait DefaultEditable: Default + Editable {
    fn with<F>(f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        Self::default().edit(f)
    }
}
impl<T: Editable + Default> DefaultEditable for T {}
