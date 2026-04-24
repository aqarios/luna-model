//! Concrete, fully validated bounds representation.

use lunamodel_types::{Bound, Vtype};

use crate::bounds::LazyBounds;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    /// Inclusive lower bound.
    pub lower: Bound,
    /// Inclusive upper bound.
    pub upper: Bound,
}

impl Bounds {
    /// Returns the default bounds implied by a variable type.
    ///
    /// Binary and inverted-binary variables share the same concrete domain.
    pub fn default_for(vtype: &Vtype) -> Self {
        use Vtype::*;
        match vtype {
            Binary => Self::binary(),
            InvertedBinary => Self::binary(),
            Spin => Self::spin(),
            Integer => Self::integer(),
            Real => Self::real(),
        }
    }

    /// Creates a new concrete bounds object from explicit endpoints.
    pub fn new(lower: Bound, upper: Bound) -> Self {
        Self { lower, upper }
    }

    /// Returns the inclusive lower bound.
    pub fn lower(&self) -> Bound {
        self.lower
    }

    /// Returns the inclusive upper bound.
    pub fn upper(&self) -> Bound {
        self.upper
    }
}

impl Bounds {
    #[inline]
    /// Returns the canonical domain for binary variables.
    pub fn binary() -> Self {
        Self {
            lower: Bound::Bounded(0.0),
            upper: Bound::Bounded(1.0),
        }
    }

    #[inline]
    /// Returns the canonical domain for spin variables.
    pub fn spin() -> Self {
        Self {
            lower: Bound::Bounded(-1.0),
            upper: Bound::Bounded(1.0),
        }
    }

    #[inline]
    /// Returns the default domain for integer variables.
    ///
    /// Integer variables are lower-bounded at zero unless callers override the
    /// bounds explicitly through [`LazyBounds`].
    pub fn integer() -> Self {
        Self {
            lower: Bound::Bounded(0.0),
            upper: Bound::Unbounded,
        }
    }

    #[inline]
    /// Returns the default domain for real-valued variables.
    pub fn real() -> Self {
        Self {
            lower: Bound::Bounded(0.0),
            upper: Bound::Unbounded,
        }
    }
}

impl From<Bounds> for LazyBounds {
    /// Wraps concrete bounds as fully specified lazy bounds.
    fn from(val: Bounds) -> Self {
        LazyBounds {
            lower: Some(val.lower),
            upper: Some(val.upper),
        }
    }
}
