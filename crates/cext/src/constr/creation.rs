use crate::utils::{drop_from_c, handle_to_c, opt_str_from_c, out_from_c, ref_from_c};
use std::ffi::c_char;

use luna_model::core::{Expression, prelude::Constraint};

use crate::{
    LmComparator, LmConstraint, LmExpr, LmStatus,
    error::{CapiError, guard},
    utils::cmp_from_c,
};

/// Frees a constraint handle. Passing null is a no-op.
///
/// # Safety
/// `constr` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_free(constr: *mut LmConstraint) {
    unsafe { drop_from_c(constr as *mut Constraint) }
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
        let lhs = unsafe { ref_from_c(lhs as *const Expression, "lhs") }?;
        let out = unsafe { out_from_c(out) }?;
        let cmp = cmp_from_c(comparator);

        let name = unsafe { opt_str_from_c(name, "name") }?.map(str::to_owned);

        let constr = Constraint::new(lhs.clone(), rhs, cmp, name).map_err(CapiError::errored)?;
        *out = handle_to_c(constr);

        Ok(())
    })
}
