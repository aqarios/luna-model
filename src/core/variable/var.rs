#[cfg(feature = "py")]
use pyo3::prelude::*;
use std::fmt::{Debug, Display, Formatter};

use crate::core::environment::EnvId;

#[cfg_attr(feature = "py", pyclass(eq, eq_int))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Vtype {
    Real,
    Integer,
    Binary,
    Spin,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Bounds {
    pub lower: Option<f64>,
    pub upper: Option<f64>,
}

impl Bounds {
    pub fn new(lower: Option<f64>, upper: Option<f64>) -> Self {
        Self { lower, upper }
    }

    pub fn default(vtype: &Vtype) -> Self {
        match vtype {
            Vtype::Real => Self::new(None, None),
            Vtype::Integer => Self::new(None, None),
            Vtype::Binary => Self::new(Some(0.0), Some(1.0)),
            Vtype::Spin => Self::new(Some(-1.0), Some(1.0)),
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

pub fn display_bound(bound: &Option<f64>) -> String {
    match bound {
        None => String::from("unlimited"),
        Some(val) => val.to_string(),
    }
}

impl Vtype {
    pub fn default() -> Self {
        Vtype::Binary
    }
}

impl Display for Vtype {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Vtype::Real => "real",
            Vtype::Integer => "int",
            Vtype::Binary => "binary",
            Vtype::Spin => "spin",
        };
        write!(f, "{out}")
    }
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub vtype: Vtype,
    pub bounds: Bounds,
    pub env_id: EnvId,
}

impl Variable {
    pub fn new(
        name: String,
        vtype: Option<&Vtype>,
        bounds: Option<&Bounds>,
        env_id: EnvId,
    ) -> Self {
        let vtype = vtype.map_or(Vtype::default(), |e| *e);
        let bounds = bounds.map_or(Bounds::default(&vtype), |e| *e);
        Self {
            bounds,
            name,
            vtype,
            env_id,
        }
    }
    fn format_bounds(&self) -> String {
        let mut out = String::new();
        if matches!(self.vtype, Vtype::Integer | Vtype::Real) {
            let default = Bounds::default(&self.vtype);
            let has_lower = self.bounds.lower != default.lower;
            let has_upper = self.bounds.upper != default.upper;
            if has_lower || has_upper {
                let mut bounds = vec![];
                if has_lower {
                    bounds.push(format!("lower: {}", display_bound(&self.bounds.lower)));
                }
                if has_upper {
                    bounds.push(format!("upper: {}", display_bound(&self.bounds.upper)));
                }
                out += &format!(" {{ {} }}", bounds.join(", "));
            }
        }
        out
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}{}", self.name, self.vtype, self.format_bounds())
    }
}
