use std::ffi::c_char;

use lunamodel_capi_handle::drop_handle;
use lunamodel_core::prelude::Constraint;

use crate::{
    LmComparator, LmConstraint, LmExpr, LmStatus,
    error::{CapiError, guard},
    expr::utils::expression_from_c,
    utils::cmp_from_c,
    utils::{new_handle_out_from_c, opt_str_from_c},
};

/// Frees a constraint handle. Passing null is a no-op.
///
/// # Safety
/// `constr` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_free(constr: *mut LmConstraint) {
    unsafe { drop_handle(constr) }
}

/// Creates a new constraint and normalizes its left-hand side.
///
/// If `lhs` contains a constant offset, that constant is subtracted from the
/// expression and folded into `rhs`. The resulting constraint is therefore
/// easier to compare, serialize, and translate consistently.
///
/// `lhs` is cloned, not consumed: the caller keeps ownership of the handle.
///
/// # Safety
/// `lhs` must be a valid expression handle created by this library.
/// `name` may be null (the constraint is then auto-named, e.g. `c0`);
///        if non-null it must point to a NUL-terminated, valid UTF-8 C string.
/// `out` must point to writable `LmConstraint *` storage.
///        On `LmStatus::Ok` it receives an owned handle the caller must release with `lm_constr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_new(
    lhs: *const LmExpr,
    rhs: f64,
    comparator: LmComparator,
    name: *const c_char,
    out: *mut *mut LmConstraint,
) -> LmStatus {
    guard(|| {
        let lhs = unsafe { expression_from_c(lhs, "lhs") }?.read().clone();
        let cmp = cmp_from_c(comparator);

        let name = unsafe { opt_str_from_c(name, "name") }?.map(str::to_owned);

        let constr = Constraint::new(lhs, rhs, cmp, name).map_err(CapiError::from)?;
        unsafe { new_handle_out_from_c(out, constr) }
    })
}
