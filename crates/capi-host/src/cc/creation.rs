use std::ffi::c_char;

use indexmap::IndexMap;
use lunamodel_core::ConstraintCollection;

use super::utils::{collection_drop_from_c, collection_to_c};
use crate::{
    LmCC, LmConstraint, LmStatus,
    error::guard,
    utils::{out_from_c, shared_from_c, slice_from_c, str_from_c},
};

/// Frees a constraint-collection handle. Passing null is a no-op.
///
/// # Safety
/// `cc` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_free(cc: *mut LmCC) {
    unsafe { collection_drop_from_c(cc) }
}

/// Creates a new empty constraint-collection.
///
/// Returns a non-null handle that must be released with [`lm_cc_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_cc_default() -> *mut LmCC {
    collection_to_c(ConstraintCollection::default())
}

/// Creates a new empty constraint-collection.
///
/// Returns a non-null handle that must be released with [`lm_cc_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_cc_new() -> *mut LmCC {
    collection_to_c(ConstraintCollection::new())
}

/// Builds a constraint-collection directly from `len` `(name, constraint)` pairs, mirroring the
/// native `From<IndexMap<String, Constraint>>` conversion. Unlike [`lm_cc_add_constraint`], this
/// does not synthesize fallback names or rename the inserted constraints to match their key —
/// callers are responsible for keeping `names` and each constraint's own name in sync. As with a
/// plain `IndexMap`, a repeated name keeps its first position but takes the last matching
/// constraint's value.
///
/// Returns a non-null handle in `out` that must be released with [`lm_cc_free`].
///
/// # Safety
/// `names` must point to `len` valid NUL-terminated C strings.
/// `constrs` must point to `len` valid constraint handles created by this library.
/// `out` must point to writable `LmCC *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_from_map(
    names: *const *const c_char,
    constrs: *const *const LmConstraint,
    len: usize,
    out: *mut *mut LmCC,
) -> LmStatus {
    guard(|| {
        let names = unsafe { slice_from_c(names, len, "names") }?
            .iter()
            .map(|&n| unsafe { str_from_c(n, "name") }.map(str::to_owned))
            .collect::<Result<Vec<_>, _>>()?;
        let constrs = unsafe { slice_from_c(constrs, len, "constrs") }?
            .iter()
            .map(|&c| unsafe { shared_from_c(c, "constr") }.map(|c| c.read_arc().clone()))
            .collect::<Result<Vec<_>, _>>()?;
        let map: IndexMap<String, _> = names.into_iter().zip(constrs).collect();
        let out = unsafe { out_from_c(out) }?;
        *out = collection_to_c(ConstraintCollection::from(map));
        Ok(())
    })
}
