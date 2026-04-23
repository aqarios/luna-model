use std::hash::Hash;

use std::collections::HashSet;

struct UniqueIter<T, I>
where
    T: Eq + Hash + Copy,
    I: Iterator<Item = T>,
{
    state: HashSet<T>,
    iterator: I,
}

impl<T, I> UniqueIter<T, I>
where
    T: Eq + Hash + Copy,
    I: Iterator<Item = T>,
{
    fn new(iterator: I) -> Self {
        Self {
            state: HashSet::new(),
            iterator,
        }
    }
}

impl<T, I> Iterator for UniqueIter<T, I>
where
    T: Eq + Hash + Copy,
    I: Iterator<Item = T>,
{
    type Item = Option<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.iterator.next() {
            if self.state.insert(e) {
                Some(Some(e))
            } else {
                Some(None)
            }
        } else {
            None
        }
    }
}

struct UniqueIterMap<T, A, I, F>
where
    A: Eq + Hash + Copy,
    I: Iterator<Item = T>,
    F: Fn(&T) -> A,
{
    state: HashSet<A>,
    iterator: I,
    f: F,
}

impl<T, A, I, F> UniqueIterMap<T, A, I, F>
where
    A: Eq + Hash + Copy,
    I: Iterator<Item = T>,
    F: Fn(&T) -> A,
{
    fn new(iterator: I, f: F) -> Self {
        Self {
            state: HashSet::new(),
            iterator,
            f,
        }
    }
}

impl<T, A, I, F> Iterator for UniqueIterMap<T, A, I, F>
where
    A: Eq + Hash + Copy,
    I: Iterator<Item = T>,
    F: Fn(&T) -> A,
{
    type Item = Option<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.iterator.next() {
            if self.state.insert((self.f)(&e)) {
                Some(Some(e))
            } else {
                Some(None)
            }
        } else {
            None
        }
    }
}

pub fn unique<T: Eq + Hash + Copy, I: Iterator<Item = T>>(iterator: I) -> impl Iterator<Item = T> {
    UniqueIter::<T, I>::new(iterator)
        .flatten()
}

pub fn unique_by<T, A: Eq + Hash + Copy, F: Fn(&T) -> A, I: Iterator<Item = T>>(
    iterator: I,
    f: F,
) -> impl Iterator<Item = T> {
    UniqueIterMap::<T, A, I, F>::new(iterator, f)
        .flatten()
}
