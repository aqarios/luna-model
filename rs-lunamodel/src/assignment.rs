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
    traits::{LmTryFrom, LmTryInto},
};

/// Owning handle to a LunaModel assignment (frees on drop).
pub enum Assignment {
    Binary(BinaryAssignment),
    Spin(SpinAssignment),
    Integer(IntegerAssignment),
    Real(RealAssignment),
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
