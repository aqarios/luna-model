use super::Vtype;
use std::fmt::{Debug, Display, Formatter};

#[cfg(feature = "py")]
use pyo3::prelude::pyclass;

const fn unwrap_failed() -> ! {
    panic!("called `Bound::unwrap()` on an `Unbounded` value")
}

#[cfg_attr(feature = "py", pyclass(eq, name = "Bound", module = "aqmodels"))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bound {
    /// Explicitly unbounded value.
    Unbounded(),
    /// Represents an explicit value.
    Some(f64),
}

impl Bound {
    pub fn is_bounded(&self) -> bool {
        *self != Self::Unbounded()
    }

    pub fn unwrap(self) -> f64 {
        match self {
            Self::Some(val) => val,
            Self::Unbounded() => unwrap_failed(),
        }
    }
}

// #[cfg_attr(feature = "py", pyclass(eq, name = "LazyBound", module = "aqmodels"))]
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum LazyBound {
//     Default(),
//     Bound(Bound),
// }

/// The bounds on a variable.
#[derive(Clone, Copy, PartialEq)]
pub struct Bounds {
    // The lower bound.
    pub lower: Bound,
    // The upper bound.
    pub upper: Bound,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LazyBounds {
    pub lower: Option<Bound>,
    pub upper: Option<Bound>,
}

impl LazyBounds {
    pub fn new(lower: Option<Bound>, upper: Option<Bound>) -> Self {
        Self { lower, upper }
    }
}

impl Into<LazyBounds> for Bounds {
    fn into(self) -> LazyBounds {
        LazyBounds::new(Some(self.lower), Some(self.upper))
    }
}

impl Bounds {
    // /// Create a new instance of bounds based on the lower and upper bounds.
    // /// None indicate no lower or upper bound.
    pub fn new(lower: Bound, upper: Bound) -> Self {
        Self { lower, upper }
    }

    pub fn lazy(lower: Option<Bound>, upper: Option<Bound>) -> LazyBounds {
        LazyBounds::new(lower, upper)
    }

    /// Create a new instance of bounds based on the given vtype.
    pub fn default(vtype: &Vtype) -> Self {
        match vtype {
            Vtype::Real => Self::real(),
            Vtype::Integer => Self::integer(),
            Vtype::Binary => Self::binary(),
            Vtype::Spin => Self::spin(),
        }
    }

    /// The bounds of a binary variable.
    pub fn binary() -> Self {
        Self::new(Bound::Some(0.0), Bound::Some(1.0))
    }
    /// The bounds of a spin variable.
    pub fn spin() -> Self {
        Self::new(Bound::Some(-1.0), Bound::Some(1.0))
    }
    /// The default bounds of an integer variable.
    pub fn integer() -> Self {
        Self::new(Bound::Some(0.0), Bound::Unbounded())
    }
    /// The default bounds of a real variable.
    pub fn real() -> Self {
        Self::new(Bound::Some(0.0), Bound::Unbounded())
    }

    pub fn evaluate<Elem: PartialEq<f64> + PartialOrd<f64>>(&self, value: Elem) -> bool {
        use Bound::{Some, Unbounded};
        match (self.lower, self.upper) {
            (Some(l), Some(u)) => value >= l && value <= u,
            (Some(l), Unbounded()) => value >= l,
            (Unbounded(), Some(u)) => value <= u,
            (Unbounded(), Unbounded()) => true,
        }
    }
}

impl Debug for Bounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lower = display_bound(&self.lower);
        let upper = display_bound(&self.upper);
        f.debug_struct("Bounds")
            .field("lower", &format_args!("{lower}"))
            .field("upper", &format_args!("{upper}"))
            .finish()
    }
}

impl Display for Bounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lower = display_bound(&self.lower);
        let upper = display_bound(&self.upper);
        write!(f, "{{ lower: {lower}, upper: {upper} }}")
    }
}

pub fn display_bound(bound: &Bound) -> String {
    match bound {
        Bound::Unbounded() => String::from("unlimited"),
        Bound::Some(val) => val.to_string(),
    }
}

impl Display for LazyBounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lower = display_lazy_bound(&self.lower);
        let upper = display_lazy_bound(&self.upper);
        write!(f, "{{ lower: {lower}, upper: {upper} }}")
    }
}

pub fn display_lazy_bound(bound: &Option<Bound>) -> String {
    match bound {
        None => String::from("None"),
        Some(Bound::Unbounded()) => String::from("unlimited"),
        Some(Bound::Some(val)) => val.to_string(),
    }
}
