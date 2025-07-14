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

impl<T: ContentEquality + ?Sized> ContentEquality for &T {
    fn is_equal_contents(&self, other: &&T) -> bool {
        (*self).is_equal_contents(*other)
    }
}

pub trait FilterByMask<T> {
    fn filter_by_mask(&self, mask: &Vec<bool>) -> Vec<T>;
}

impl<T: Clone> FilterByMask<T> for Vec<T> {
    fn filter_by_mask(&self, mask: &Vec<bool>) -> Vec<T> {
        self.iter()
            .zip(mask)
            .filter_map(|(x, flag)| flag.then_some(x.clone()))
            .collect()
    }
}

pub trait PushOrCreate<T> {
    fn push_or_create(&mut self, item: T);
}

impl<T> PushOrCreate<T> for Option<Vec<T>> {
    fn push_or_create(&mut self, item: T) {
        if let Some(ref mut data) = self {
            data.push(item);
        } else {
            self.insert(vec![item]);
        }
    }
}
