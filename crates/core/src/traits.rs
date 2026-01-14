use std::error::Error;

pub trait ContentEquality {
    fn equal_contents(&self, other: &Self) -> bool;
}

impl<T> ContentEquality for Vec<T>
where
    T: ContentEquality,
{
    fn equal_contents(&self, other: &Self) -> bool {
        self.iter().zip(other).all(|(l, r)| l.equal_contents(r))
    }
}

impl<T: ContentEquality + ?Sized> ContentEquality for &T {
    fn equal_contents(&self, other: &&T) -> bool {
        (*self).equal_contents(*other)
    }
}

pub trait Editable {
    fn maybe_edit<F, E>(mut self, f: F) -> Result<Self, E>
    where
        Self: Sized,
        E: Error,
        F: FnOnce(&mut Self) -> Result<(), E>,
    {
        f(&mut self)?;
        Ok(self)
    }

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
    fn maybe_with<F, E>(f: F) -> Result<Self, E>
    where
        Self: Sized,
        E: Error,
        F: FnOnce(&mut Self) -> Result<(), E>,
    {
        Self::default().maybe_edit(f)
    }

    fn with<F>(f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        Self::default().edit(f)
    }
}
impl<T: Editable + Default> DefaultEditable for T {}

pub trait FilterByMask<T> {
    fn filter_by_mask(&self, mask: &[bool]) -> Vec<T>;
}

impl<T: Clone> FilterByMask<T> for Vec<T> {
    fn filter_by_mask(&self, mask: &[bool]) -> Vec<T> {
        self.iter()
            .zip(mask)
            .filter_map(|(x, flag)| flag.then_some(x.clone()))
            .collect()
    }
}
