//! Small iterator utilities shared across crates.
//!
//! The helpers in this crate stay intentionally minimal. They avoid pulling in a
//! heavier iterator utility dependency for the few deduplication behaviors that
//! LunaModel uses repeatedly when walking variables, types, and constraints.
use std::hash::Hash;

use std::collections::HashSet;

/// Iterator adaptor that yields only the first occurrence of each copied value.
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
    /// Creates a new uniqueness-filtering iterator.
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
    /// Returns `Some(Some(value))` for unseen values and `Some(None)` for duplicates.
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

/// Iterator adaptor that deduplicates values by a projected key.
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
    /// Creates a new key-based uniqueness-filtering iterator.
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
    /// Returns `Some(Some(value))` when the projected key has not been seen before.
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

/// Returns an iterator over the first occurrence of each copied value.
pub fn unique<T: Eq + Hash + Copy, I: Iterator<Item = T>>(iterator: I) -> impl Iterator<Item = T> {
    UniqueIter::<T, I>::new(iterator).flatten()
}

/// Returns an iterator over the first occurrence of each projected key.
pub fn unique_by<T, A: Eq + Hash + Copy, F: Fn(&T) -> A, I: Iterator<Item = T>>(
    iterator: I,
    f: F,
) -> impl Iterator<Item = T> {
    UniqueIterMap::<T, A, I, F>::new(iterator, f).flatten()
}
