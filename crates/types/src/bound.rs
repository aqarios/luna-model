//! Numeric bound endpoint types.

use std::fmt::Display;

/// One side of a variable bound interval.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bound {
    /// A finite inclusive bound value.
    Bounded(f64),
    /// No bound on this side of the interval.
    Unbounded,
}

impl Bound {
    /// Returns whether the bound is finite.
    pub fn is_bounded(&self) -> bool {
        match self {
            Self::Bounded(_) => true,
            Self::Unbounded => false,
        }
    }
}

impl Display for Bound {
    /// Formats bounded values numerically and unbounded values as `"Unbounded"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Bounded(v) => write!(f, "{}", v),
            Self::Unbounded => write!(f, "Unbounded"),
        }
    }
}
