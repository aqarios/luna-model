//! Owning wrapper over a LunaModel assignment handle.
//!
//! Mirrors the Rust `Assignment`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Assignment` still return `Result` here.

use lunamodel_cext_vtable::{LmAssignment, LmVtype};
use lunamodel_types::{BinaryAssignment, IntegerAssignment, RealAssignment, SpinAssignment};

use crate::{
    Error, Result, Status,
    traits::{LmFrom, LmTryFrom, LmTryInto},
};

/// Type-erased single assignment value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Assignment {
    Binary(BinaryAssignment),
    Spin(SpinAssignment),
    Integer(IntegerAssignment),
    Real(RealAssignment),
}

impl LmFrom<Assignment> for LmAssignment {
    fn lm_from(assignment: Assignment) -> Self {
        match assignment {
            Assignment::Binary(value) => Self {
                vtype: LmVtype::Binary,
                value: value as f64,
            },
            Assignment::Spin(value) => Self {
                vtype: LmVtype::Spin,
                value: value as f64,
            },
            Assignment::Integer(value) => Self {
                vtype: LmVtype::Integer,
                value: value as f64,
            },
            Assignment::Real(value) => Self {
                vtype: LmVtype::Real,
                value,
            },
        }
    }
}

impl Assignment {
    pub fn value(self) -> f64 {
        match self {
            Self::Binary(value) => value as f64,
            Self::Spin(value) => value as f64,
            Self::Integer(value) => value as f64,
            Self::Real(value) => value,
        }
    }
}

impl From<u8> for Assignment {
    fn from(value: u8) -> Self {
        Self::Binary(value)
    }
}

impl From<i8> for Assignment {
    fn from(value: i8) -> Self {
        Self::Spin(value)
    }
}

impl From<i64> for Assignment {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Assignment {
    fn from(value: f64) -> Self {
        Self::Real(value)
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(value) => write!(f, "{value}"),
            Self::Spin(value) => write!(f, "{value}"),
            Self::Integer(value) => write!(f, "{value}"),
            Self::Real(value) => {
                let value = format!("{value:.}");
                if value.contains('.') {
                    write!(f, "{value}")
                } else {
                    write!(f, "{value}.0")
                }
            }
        }
    }
}

impl LmTryFrom<LmAssignment> for Assignment {
    fn lm_try_from(a: LmAssignment) -> Result<Self> {
        match a.vtype {
            LmVtype::Binary => Ok(Assignment::Binary(a.value.lm_try_into()?)),
            LmVtype::Spin => Ok(Assignment::Spin(a.value.lm_try_into()?)),
            LmVtype::Integer => Ok(Assignment::Integer(a.value.lm_try_into()?)),
            LmVtype::Real => Ok(Assignment::Real(a.value)),
            LmVtype::InvertedBinary => Err(Error {
                status: Status::InvalidArgument,
                message: "cannot create an assignment for a plain inverted binary".into(),
            }),
        }
    }
}
