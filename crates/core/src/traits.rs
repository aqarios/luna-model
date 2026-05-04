//! Shared helper traits used across core data structures.

use std::error::Error;

/// Semantic equality that can ignore identity details.
///
/// `PartialEq` in LunaModel often remains strict about environment identity or
/// names because those details matter for some workflows. `ContentEquality`
/// exists for the cases where callers want to know whether two values describe
/// the same mathematical content instead.
pub trait ContentEquality {
    /// Returns whether `self` and `other` are semantically equivalent.
    fn equal_contents(&self, other: &Self) -> bool;
}

impl<T> ContentEquality for Vec<T>
where
    T: ContentEquality,
{
    /// Compares vectors elementwise using semantic equality.
    fn equal_contents(&self, other: &Self) -> bool {
        self.iter().zip(other).all(|(l, r)| l.equal_contents(r))
    }
}

impl<T: ContentEquality + ?Sized> ContentEquality for &T {
    /// Delegates semantic equality through shared references.
    fn equal_contents(&self, other: &&T) -> bool {
        (*self).equal_contents(*other)
    }
}

/// Builder-style editing helpers for move-oriented APIs.
///
/// Many LunaModel types are cheap enough to construct and then modify in a
/// closure. These helpers keep that pattern concise while still supporting both
/// infallible and fallible edits.
pub trait Editable {
    /// Applies a fallible edit closure and returns the edited value.
    fn maybe_edit<F, E>(mut self, f: F) -> Result<Self, E>
    where
        Self: Sized,
        E: Error,
        F: FnOnce(&mut Self) -> Result<(), E>,
    {
        f(&mut self)?;
        Ok(self)
    }

    /// Applies an infallible edit closure and returns the edited value.
    fn edit<F>(mut self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(&mut Self),
    {
        f(&mut self);
        self
    }
}

/// Convenience extension for types that are both [`Default`] and [`Editable`].
pub trait DefaultEditable: Default + Editable {
    /// Constructs the default value, applies a fallible edit closure, and returns it.
    fn maybe_with<F, E>(f: F) -> Result<Self, E>
    where
        Self: Sized,
        E: Error,
        F: FnOnce(&mut Self) -> Result<(), E>,
    {
        Self::default().maybe_edit(f)
    }

    /// Constructs the default value, applies an infallible edit closure, and returns it.
    fn with<F>(f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        Self::default().edit(f)
    }
}
impl<T: Editable + Default> DefaultEditable for T {}

/// Helper trait for filtering vector-like data with a boolean keep-mask.
pub trait FilterByMask<T> {
    /// Returns all elements whose corresponding mask entry is `true`.
    fn filter_by_mask(&self, mask: &[bool]) -> Vec<T>;
}

impl<T: Clone> FilterByMask<T> for Vec<T> {
    /// Filters a vector by cloning the retained elements.
    fn filter_by_mask(&self, mask: &[bool]) -> Vec<T> {
        self.iter()
            .zip(mask)
            .filter_map(|(x, flag)| flag.then_some(x.clone()))
            .collect()
    }
}

/// Fallible indexing abstraction used by expression and constraint evaluation.
///
/// This lets evaluation code work with plain maps, solution views, and other
/// sample containers without committing to a single concrete collection type.
pub trait TryIndex<I> {
    type Err;
    type Output;

    /// Returns a reference to the indexed value or an indexing error.
    fn try_index(&self, index: I) -> Result<&Self::Output, Self::Err>;
}
