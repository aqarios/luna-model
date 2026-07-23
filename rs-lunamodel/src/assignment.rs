//! Owning wrapper over a LunaModel assignment handle.
//!
//! Mirrors the Rust `Assignment`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_capi::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Assignment` still return `Result` here.

use lunamodel_capi::{LmAssignment, LmVtype};
pub use lunamodel_types::Assignment;
use lunamodel_types::Vtype;

use crate::{
    Error, Result, Status,
    traits::{LmFrom, LmTryFrom},
};

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

impl LmTryFrom<LmAssignment> for Assignment {
    fn lm_try_from(a: LmAssignment) -> Result<Self> {
        let vtype = match a.vtype {
            LmVtype::Binary => Vtype::Binary,
            LmVtype::Spin => Vtype::Spin,
            LmVtype::Integer => Vtype::Integer,
            LmVtype::Real => Vtype::Real,
            LmVtype::InvertedBinary => {
                return Err(Error {
                    status: Status::InvalidArgument,
                    message: "cannot create an assignment for a plain inverted binary".into(),
                });
            }
        };
        Assignment::try_from_value(vtype, a.value).ok_or_else(|| Error {
            status: Status::InvalidArgument,
            message: "assignment value is incompatible with its variable type".into(),
        })
    }
}
