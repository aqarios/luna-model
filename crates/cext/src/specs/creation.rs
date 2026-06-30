use luna_model::types::{Ctype, EnumSetFromVec, Specs, Vtype};

use crate::{
    LmCtypeBuffer, LmSense, LmSpecs, LmStatus, LmVtypeBuffer,
    error::{CapiError, guard},
    utils::{ctype_from_c, sense_from_c, vtype_from_c},
};

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
/// `vtypes` and `ctypes` are read but not consumed: the caller keeps ownership
/// and must still release them with `lm_vtype_buffer_free` / `lm_ctype_buffer_free`.
///
/// # Safety
/// `sense` must point to a readable `LmSense`.
/// `vtypes` must point to a readable `LmVtypeBuffer` created by this library.
/// `ctypes` must point to a readable `LmCtypeBuffer` created by this library.
/// `out` must point to writable `*mut LmSpecs` storage.
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
    guard(|| {
        let sense = unsafe { sense.as_ref() }.ok_or_else(|| CapiError::null_pointer("sense"))?;
        let vtypes = unsafe { vtypes.as_ref() }.ok_or_else(|| CapiError::null_pointer("vtypes"))?;
        let ctypes = unsafe { ctypes.as_ref() }.ok_or_else(|| CapiError::null_pointer("ctypes"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        // `from_raw_parts` requires a non-null pointer, so treat empty/null as no entries.
        let vtype_set = if vtypes.data.is_null() || vtypes.len == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(vtypes.data, vtypes.len) }
                .iter()
                .copied()
                .map(vtype_from_c)
                .collect::<Vec<Vtype>>()
        }
        .to_enumset();

        let ctype_set = if ctypes.data.is_null() || ctypes.len == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(ctypes.data, ctypes.len) }
                .iter()
                .copied()
                .map(ctype_from_c)
                .collect::<Vec<Ctype>>()
        }
        .to_enumset();

        let specs = Specs::new(
            sense_from_c(*sense),
            vtype_set,
            ctype_set,
            max_degree,
            max_constraint_degree,
            max_num_variables,
        );

        *out = Box::into_raw(Box::new(specs)) as *mut LmSpecs;
        Ok(())
    })
}
