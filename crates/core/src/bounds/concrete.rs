use lunamodel_types::{Bound, Vtype};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub(crate) lower: Bound,
    pub(crate) upper: Bound,
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
}

impl Bounds {
    #[inline]
    fn binary() -> Self {
        Self {
            lower: Bound::Bounded(0.0),
            upper: Bound::Bounded(1.0),
        }
    }

    #[inline]
    fn spin() -> Self {
        Self {
            lower: Bound::Bounded(-1.0),
            upper: Bound::Bounded(1.0),
        }
    }

    #[inline]
    fn integer() -> Self {
        Self {
            lower: Bound::Bounded(0.0),
            upper: Bound::Unbounded,
        }
    }

    #[inline]
    fn real() -> Self {
        Self {
            lower: Bound::Unbounded,
            upper: Bound::Unbounded,
        }
    }
}
