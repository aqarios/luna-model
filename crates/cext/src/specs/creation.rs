use luna_model::types::Specs;

use crate::{LmCtypeBuffer, LmSense, LmSpecs, LmStatus, LmVtypeBuffer};

/// Frees a specs handle. Passing null is a no-op.
///
/// # Safety
/// `specs` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_specs_free(specs: *mut LmSpecs) {
    if !specs.is_null() {
        drop(unsafe { Box::from_raw(specs as *mut Specs) });
    }
}

/// Creates a fully specified capability summary.
///
/// # Safety
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_specs_new(
    sense: *const LmSense,
    vtypes: *const LmVtypeBuffer,
    ctypes: *const LmCtypeBuffer,
    max_degree: usize,
    max_constraint_degree: usize,
    max_num_variables: usize,
    out: *mut *mut LmSpecs,
) -> LmStatus {
    todo!()
}
