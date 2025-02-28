#[cfg(feature = "py")]
use pyo3::prelude::*;

use crate::core::environment::EnvId;

#[cfg_attr(feature = "py", pyclass(eq, eq_int))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Vtype {
    Real,
    Integer,
    Binary,
    Spin,
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub lower: Option<f64>,
    pub upper: Option<f64>,
}

impl Bounds {
    pub fn new(lower: Option<f64>, upper: Option<f64>) -> Self {
        Self { lower, upper }
    }
}

impl Bounds {
    pub fn default(vtype: &Vtype) -> Self {
        match vtype {
            Vtype::Real => Self::new(None, None),
            Vtype::Integer => Self::new(None, None),
            Vtype::Binary => Self::new(Some(0.0), Some(1.0)),
            Vtype::Spin => Self::new(Some(-1.0), Some(1.0)),
        }
    }
}

impl Vtype {
    pub fn default() -> Self {
        Vtype::Binary
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
    pub fn new(name: String, vtype: Option<&Vtype>, bounds: Option<Bounds>, env_id: EnvId) -> Self {
        let vtype = vtype.map_or(Vtype::default(), |e| *e);
        let bounds = bounds.map_or(Bounds::default(&vtype), |e| e);
        Self {
            bounds,
            name,
            vtype,
            env_id,
        }
    }
}
