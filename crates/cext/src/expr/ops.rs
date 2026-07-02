use luna_model::core::{Expression, ops::LmAddAssign, prelude::VarRef};

use crate::{
    LmExpr, LmStatus, LmVref,
    error::{CapiError, guard},
    utils::{mut_from_c, ref_from_c},
};

/// Adds a variable reference to the expression in place (Rust `expr += &vref`).
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `vref` must be a valid vref handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_add_vref(expr: *mut LmExpr, vref: *const LmVref) -> LmStatus {
    guard(|| {
        let expr = unsafe { mut_from_c(expr as *mut Expression, "expr") }?;
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        expr.add_assign(vref).map_err(CapiError::errored)?;
        Ok(())
    })
}
