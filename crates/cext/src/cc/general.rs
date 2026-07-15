use crate::utils::{handle_to_c, opt_from_c, out_from_c, ref_from_c, slice_from_c};
use luna_model::core::{ConstraintCollection, prelude::ContentEquality};

use crate::{
    LmBoolMapBuffer, LmCC, LmEnv, LmStatus,
    buffer::bool_map_buffer_from_vec,
    env::utils::environment_from_c,
    error::{CapiError, guard},
};

/// Clones a constraint-collection into a new, independent handle written to `out`.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle created by this library.
/// `out` must point to writable `LmCC *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_clone(cc: *const LmCC, out: *mut *mut LmCC) -> LmStatus {
    guard(|| {
        let cc = unsafe { ref_from_c(cc as *const ConstraintCollection, "cc") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(cc.clone());
        Ok(())
    })
}

/// Deep clones a constraint-collection into a new, independent handle written to `out`.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle created by this library.
/// `out` must point to writable `LmCC *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_deep_clone(
    cc: *const LmCC,
    env: *const LmEnv,
    out: *mut *mut LmCC,
) -> LmStatus {
    guard(|| {
        let cc = unsafe { ref_from_c(cc as *const ConstraintCollection, "cc") }?;
        let env = unsafe { environment_from_c(env)? };
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(cc.deep_clone(env.clone()));
        Ok(())
    })
}

/// Compares semantic contents while ignoring environment identity.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle.
/// `other` must be a valid constraint-collection handle.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_eq(cc: *const LmCC, other: *const LmCC, out: *mut bool) -> LmStatus {
    guard(|| {
        let cc = unsafe { ref_from_c(cc as *const ConstraintCollection, "cc") }?;
        let other = unsafe { ref_from_c(other as *const ConstraintCollection, "other") }?;

        let out = unsafe { out_from_c(out) }?;

        *out = cc.eq(other);
        Ok(())
    })
}

/// Compares semantic contents while ignoring environment identity.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle.
/// `other` must be a valid constraint-collection handle.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_equal_contents(
    cc: *const LmCC,
    other: *const LmCC,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let cc = unsafe { ref_from_c(cc as *const ConstraintCollection, "cc") }?;
        let other = unsafe { ref_from_c(other as *const ConstraintCollection, "other") }?;

        let out = unsafe { out_from_c(out) }?;

        *out = cc.equal_contents(other);
        Ok(())
    })
}

/// Evaluates every constraint against an index-aligned dense lookup.
///
/// # Safety
/// `cc` must be a valid collection handle, `lookup` must reference `len`
/// readable doubles, `tol` may be null or readable, and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_evaluate_sample_quick(
    cc: *const LmCC,
    lookup: *const f64,
    len: usize,
    tol: *const f64,
    out: *mut LmBoolMapBuffer,
) -> LmStatus {
    guard(|| {
        let cc = unsafe { ref_from_c(cc as *const ConstraintCollection, "cc") }?;
        let lookup = unsafe { slice_from_c(lookup, len, "lookup") }?;
        let tol = unsafe { opt_from_c(tol) };
        let out = unsafe { out_from_c(out) }?;
        let values = cc
            .evaluate_sample_quick(lookup, tol)
            .map_err(CapiError::errored)?
            .into_iter()
            .map(|(name, value)| (name, vec![value]))
            .collect();
        *out = bool_map_buffer_from_vec(values);
        Ok(())
    })
}
