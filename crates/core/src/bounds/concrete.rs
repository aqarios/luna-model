use lunamodel_types::{Bound, Vtype};

use crate::bounds::LazyBounds;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub lower: Bound,
    pub upper: Bound,
}

impl Bounds {
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

    pub fn new(lower: Bound, upper: Bound) -> Self {
        Self { lower, upper }
    }

    pub fn lower(&self) -> Bound {
        self.lower
    }

    pub fn upper(&self) -> Bound {
        self.upper
    }
}

impl Bounds {
    #[inline]
    pub fn binary() -> Self {
        Self {
            lower: Bound::Bounded(0.0),
            upper: Bound::Bounded(1.0),
        }
    }

    #[inline]
    pub fn spin() -> Self {
        Self {
            lower: Bound::Bounded(-1.0),
            upper: Bound::Bounded(1.0),
        }
    }

    #[inline]
    pub fn integer() -> Self {
        Self {
            lower: Bound::Bounded(0.0),
            upper: Bound::Unbounded,
        }
    }

    #[inline]
    pub fn real() -> Self {
        Self {
            lower: Bound::Bounded(0.0),
            upper: Bound::Unbounded,
        }
    }
}

impl From<Bounds> for LazyBounds {
    fn from(val: Bounds) -> Self {
        LazyBounds {
            lower: Some(val.lower),
            upper: Some(val.upper),
        }
    }
}
