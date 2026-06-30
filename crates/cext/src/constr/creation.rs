use std::ffi::{CStr, c_char};

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
    if !constr.is_null() {
        drop(unsafe { Box::from_raw(constr as *mut Constraint) });
    }
}

/// Creates a new constraint and normalizes its left-hand side.
///
/// If `lhs` contains a constant offset, that constant is subtracted from the
/// expression and folded into `rhs`. The resulting constraint is therefore
/// easier to compare, serialize, and translate consistently.
///
/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_new(
    lhs: *const LmExpr,
    rhs: f64,
    comparator: LmComparator,
    name: *const c_char,
    out: *mut *mut LmConstraint,
) -> LmStatus {
    guard(|| {
        let lhs = unsafe { (lhs as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("lhs"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let cmp = cmp_from_c(comparator);

        let name = if name.is_null() {
            None
        } else {
            let cstr = unsafe { CStr::from_ptr(name) };
            Some(
                cstr.to_str()
                    .map_err(|e| {
                        CapiError::new(
                            LmStatus::InvalidArgument,
                            format!("`name` must be valid UTF-8: {e}"),
                        )
                    })?
                    .to_owned(),
            )
        };

        let constr = Constraint::new(lhs.clone(), rhs, cmp, name)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        *out = Box::into_raw(Box::new(constr)) as *mut LmConstraint;

        Ok(())
    })
}
