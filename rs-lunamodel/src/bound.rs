//! Tagged-enum wrapper for a numeric bound endpoint.

use std::fmt::Display;

use lunamodel_cext_vtable::{LmBound, LmBoundTag};

use crate::{
    Result,
    traits::{LmFrom, LmTryFrom},
};

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
        matches!(self, Self::Bounded(_))
    }
}

impl LmFrom<LmBound> for Bound {
    fn lm_from(bound: LmBound) -> Self {
        match bound.tag {
            LmBoundTag::Bounded => Self::Bounded(bound.value),
            LmBoundTag::Unbounded => Self::Unbounded,
        }
    }
}

impl LmFrom<Bound> for LmBound {
    fn lm_from(bound: Bound) -> Self {
        match bound {
            Bound::Bounded(value) => Self {
                tag: LmBoundTag::Bounded,
                value,
            },
            Bound::Unbounded => Self {
                tag: LmBoundTag::Unbounded,
                value: 0.0,
            },
        }
    }
}

impl LmTryFrom<LmBound> for Bound {
    fn lm_try_from(bound: LmBound) -> Result<Self> {
        Ok(Self::lm_from(bound))
    }
}

impl Display for Bound {
    /// Formats bounded values numerically and unbounded values as `"Unbounded"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bounded(value) => write!(f, "{value}"),
            Self::Unbounded => write!(f, "Unbounded"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{LmInto, LmTryInto};

    #[test]
    fn bounded_round_trips_through_abi() {
        let raw: LmBound = Bound::Bounded(-1.25).lm_into();

        assert_eq!(raw.tag, LmBoundTag::Bounded);
        assert_eq!(raw.value, -1.25);
        let round_trip: Bound = raw.lm_try_into().unwrap();
        assert_eq!(round_trip, Bound::Bounded(-1.25));
    }

    #[test]
    fn unbounded_round_trips_through_abi_with_canonical_payload() {
        let raw: LmBound = Bound::Unbounded.lm_into();

        assert_eq!(raw.tag, LmBoundTag::Unbounded);
        assert_eq!(raw.value, 0.0);
        let round_trip: Bound = raw.lm_try_into().unwrap();
        assert_eq!(round_trip, Bound::Unbounded);
    }

    #[test]
    fn unbounded_ignores_the_abi_payload() {
        let raw = LmBound {
            tag: LmBoundTag::Unbounded,
            value: f64::NAN,
        };

        let bound: Bound = raw.lm_try_into().unwrap();
        assert_eq!(bound, Bound::Unbounded);
    }
}
