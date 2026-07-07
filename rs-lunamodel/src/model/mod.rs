//! Owning wrapper over a LunaModel model handle.
//!
//! Mirrors the Rust `Model`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Model` still return `Result` here.

mod access;
mod creation;

use lunamodel_cext_vtable::LmModel;
use rs_lunamodel_derive::RawHandle;

use crate::{ContentEquality, Serialization, TryClone, TryDeepClone, api::api};

/// Owning handle to a LunaModel model (frees on drop).
#[derive(RawHandle)]
pub struct Model {
    handle: *mut LmModel,
}

impl Drop for Model {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, model_free)
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::from_raw((api().model_default)())
    }
}

impl PartialEq for Model {
    /// Structural equality including environment identity.
    ///
    /// An FFI failure is treated as "not equal".
    fn eq(&self, other: &Self) -> bool {
        out_or!(model_eq(@self, @other), false)
    }
}

impl TryClone for Model {
    fn try_clone(&self) -> crate::Result<Self> {
        out!(model_clone(@self))
    }
}

impl TryDeepClone<'_> for Model {
    type Target = ();

    fn try_deep_clone(&self, _: Self::Target) -> crate::Result<Self> {
        out!(model_deep_clone(@self))
    }
}

impl ContentEquality for Model {
    fn equal_contents(&self, other: &Self) -> crate::Result<bool> {
        out!(model_equal_contents(@self, @other))
    }
}

impl Serialization<'_> for Model {
    type Payload = ();

    fn serialize(&self) -> crate::Result<Vec<u8>> {
        out!(model_serialize(@self))
    }

    fn deserialize(data: &[u8], _: Self::Payload) -> crate::Result<Self> {
        out!(model_deserialize(@data, data.len()))
    }
}

// pub model_set_name: unsafe extern "C" fn(*mut LmModel, *const c_char) -> LmStatus,
// pub model_set_sense: unsafe extern "C" fn(*mut LmModel, LmSense) -> LmStatus,
// pub model_set_objective: unsafe extern "C" fn(*mut LmModel, *const LmExpr) -> LmStatus,
// pub model_set_constraints: unsafe extern "C" fn(*mut LmModel, *const LmCC) -> LmStatus,
// pub model_add_var: unsafe extern "C" fn(
//     *mut LmModel,
//     *const c_char,
//     LmVtype,
//     *const LmBounds,
//     *mut *mut LmVref,
// ) -> LmStatus,
// pub model_add_var_with_fallback: unsafe extern "C" fn(
//     *mut LmModel,
//     *const c_char,
//     LmVtype,
//     *const LmBounds,
//     *mut *mut LmVref,
// ) -> LmStatus,
// pub model_satisfies: unsafe extern "C" fn(*const LmModel, *const LmSpecs, *mut bool) -> LmStatus,
//
// pub model_evaluate_solution: unsafe extern "C" fn(*const LmModel, *const LmSol, *mut *mut LmSol) -> LmStatus,
// pub model_evaluate_solution_with_tol: unsafe extern "C" fn(*const LmModel, *const LmSol, *const f64, *mut *mut LmSol) -> LmStatus,
// pub model_substitute: unsafe extern "C" fn(*mut LmModel, *const LmVref, *const LmExpr) -> LmStatus,
