use std::ffi::c_char;

use luna_model::core::prelude::ContentEquality;

use super::utils::{collection_from_c, collection_out_from_c};
use crate::{
    LmBoolMapBuffer, LmCC, LmEnv, LmStatus,
    buffer::bool_map_buffer_from_vec,
    error::{CapiError, guard},
    eval::NameSample,
    utils::{opt_from_c, out_from_c, slice_from_c, storage_from_c},
};

/// Evaluates every constraint against a name-keyed sample.
///
/// # Safety
/// `cc` must be valid, `names` and `values` must reference `len` readable
/// elements, `tol` may be null or readable, and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_evaluate_sample(
    cc: *const LmCC,
    names: *const *const c_char,
    values: *const f64,
    len: usize,
    tol: *const f64,
    out: *mut LmBoolMapBuffer,
) -> LmStatus {
    guard(|| {
        let cc = unsafe { collection_from_c(cc, "cc") }?.read();
        let sample = unsafe { NameSample::from_c(names, values, len) }?;
        let values = cc
            .evaluate_sample(&sample, unsafe { opt_from_c(tol) })
            .map_err(CapiError::errored)?
            .into_iter()
            .map(|(name, value)| (name, vec![value]))
            .collect();
        *unsafe { out_from_c(out) }? = bool_map_buffer_from_vec(values);
        Ok(())
    })
}

/// Clones a constraint-collection into a new, independent handle written to `out`.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle created by this library.
/// `out` must point to writable `LmCC *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_clone(cc: *const LmCC, out: *mut *mut LmCC) -> LmStatus {
    guard(|| {
        let cc = unsafe { collection_from_c(cc, "cc") }?.read().clone();
        unsafe { collection_out_from_c(out, cc) }
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
        let cc = unsafe { collection_from_c(cc, "cc") }?.read();
        let env = unsafe { storage_from_c(env, "env")? };
        unsafe { collection_out_from_c(out, cc.deep_clone(env.clone())) }
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
        let cc = unsafe { collection_from_c(cc, "cc") }?.read();
        let other = unsafe { collection_from_c(other, "other") }?.read();

        let out = unsafe { out_from_c(out) }?;

        *out = cc.eq(&other);
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
        let cc = unsafe { collection_from_c(cc, "cc") }?.read();
        let other = unsafe { collection_from_c(other, "other") }?.read();

        let out = unsafe { out_from_c(out) }?;

        *out = cc.equal_contents(&other);
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
        let cc = unsafe { collection_from_c(cc, "cc") }?.read();
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
