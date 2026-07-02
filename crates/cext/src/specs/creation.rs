use crate::utils::{drop_from_c, handle_to_c, opt_from_c, opt_slice_from_c, out_from_c};
use luna_model::types::{Ctype, EnumSetFromVec, Specs, Vtype};

use crate::{
    LmCtype, LmSense, LmSpecs, LmStatus, LmVtype,
    error::guard,
    utils::{ctype_from_c, sense_from_c, vtype_from_c},
};

/// Frees a specs handle. Passing null is a no-op.
///
/// # Safety
/// `specs` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_specs_free(specs: *mut LmSpecs) {
    unsafe { drop_from_c(specs as *mut Specs) }
}

/// Creates a capability summary.
///
/// Every property is optional, mirroring the Rust `Specs` struct: a null
/// pointer means "unspecified / do not care" (`None`). For `vtypes` and
/// `ctypes` a non-null pointer with `len == 0` is an explicit empty set,
/// which is different from an unspecified (null) one. The arrays are read
/// but not consumed: the caller keeps ownership.
///
/// # Safety
/// `sense` must be null or point to a readable `LmSense`.
/// `vtypes` must be null or point to `vtypes_len` readable `LmVtype` values.
/// `ctypes` must be null or point to `ctypes_len` readable `LmCtype` values.
/// `max_degree`, `max_constraint_degree` and `max_num_variables` must each be
/// null or point to a readable `uintptr_t`.
/// `out` must point to writable `LmSpecs *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_specs_new(
    sense: *const LmSense,
    vtypes: *const LmVtype,
    vtypes_len: usize,
    ctypes: *const LmCtype,
    ctypes_len: usize,
    max_degree: *const usize,
    max_constraint_degree: *const usize,
    max_num_variables: *const usize,
    out: *mut *mut LmSpecs,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;

        let vtype_set = unsafe { opt_slice_from_c(vtypes, vtypes_len) }.map(|s| {
            s.iter()
                .copied()
                .map(vtype_from_c)
                .collect::<Vec<Vtype>>()
                .to_enumset()
        });

        let ctype_set = unsafe { opt_slice_from_c(ctypes, ctypes_len) }.map(|s| {
            s.iter()
                .copied()
                .map(ctype_from_c)
                .collect::<Vec<Ctype>>()
                .to_enumset()
        });

        let specs = Specs {
            sense: unsafe { opt_from_c(sense) }.map(sense_from_c),
            vtypes: vtype_set,
            constraints: ctype_set,
            max_degree: unsafe { opt_from_c(max_degree) },
            max_constraint_degree: unsafe { opt_from_c(max_constraint_degree) },
            max_num_variables: unsafe { opt_from_c(max_num_variables) },
        };

        *out = handle_to_c(specs);
        Ok(())
    })
}
