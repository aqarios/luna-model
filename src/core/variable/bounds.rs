use super::Vtype;
use std::fmt::{Debug, Display, Formatter};

/// The bounds on a variable.
#[derive(Clone, Copy, PartialEq)]
pub struct Bounds {
    // The lower bound.
    pub lower: Option<f64>,
    // The upper bound.
    pub upper: Option<f64>,
}

impl Bounds {
    /// Create a new instance of bounds based on the lower and upper bounds.
    /// None indicate no lower or upper bound.
    pub fn new(lower: Option<f64>, upper: Option<f64>) -> Self {
        Self { lower, upper }
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
        Self::new(Some(0.0), Some(1.0))
    }
    /// The bounds of a spin variable.
    pub fn spin() -> Self {
        Self::new(Some(-1.0), Some(1.0))
    }
    /// The default bounds of an integer variable.
    pub fn integer() -> Self {
        Self::new(None, None)
    }
    /// The default bounds of a real variable.
    pub fn real() -> Self {
        Self::new(None, None)
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

pub fn display_bound(bound: &Option<f64>) -> String {
    match bound {
        None => String::from("unlimited"),
        Some(val) => val.to_string(),
    }
}
