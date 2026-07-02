use crate::utils::{out_from_c, ref_from_c};
use luna_model::types::Specs;

use crate::{
    LmCtype, LmCtypeBuffer, LmSense, LmSpecs, LmStatus, LmVtype, LmVtypeBuffer,
    buffer::{ctype_buffer_from_vec, vtype_buffer_from_vec},
    error::{CapiError, guard},
    utils::{ctype_to_c, sense_to_c, vtype_to_c},
};

/// Returns the specs's sense, if one is set.
///
/// On success (`LmStatus::Ok`), `*out` is set to the specs's sense. If the specs
/// has no sense, returns `LmStatus::Absent` and leaves `*out` unchanged.
///
/// # Safety
/// `specs` must be a valid specs handle created by this library.
/// `out` must point to writable `LmSense` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_specs_sense(specs: *const LmSpecs, out: *mut LmSense) -> LmStatus {
    guard(|| {
        let specs = unsafe { ref_from_c(specs as *const Specs, "specs") }?;
        let out = unsafe { out_from_c(out) }?;
        match specs.sense {
            Some(s) => {
                *out = sense_to_c(s);
                Ok(())
            }
            None => Err(CapiError::new(LmStatus::Absent, "specs has no sense")),
        }
    })
}

/// Returns the specs's vtypes, if set.
///
/// On success (`LmStatus::Ok`), `*out` is set to the specs's vtypes. If the specs
/// has no vtypes, returns `LmStatus::Absent` and leaves `*out` unchanged.
///
/// # Safety
/// `specs` must be a valid specs handle created by this library.
/// `out` must point to writable `LmVtypeBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_specs_vtypes(
    specs: *const LmSpecs,
    out: *mut LmVtypeBuffer,
) -> LmStatus {
    guard(|| {
        let specs = unsafe { ref_from_c(specs as *const Specs, "specs") }?;
        let out = unsafe { out_from_c(out) }?;
        match specs.vtypes {
            Some(vs) => {
                *out =
                    vtype_buffer_from_vec(vs.into_iter().map(vtype_to_c).collect::<Vec<LmVtype>>());
                Ok(())
            }
            None => Err(CapiError::new(LmStatus::Absent, "specs has no vtypes")),
        }
    })
}

/// Returns the specs's ctypes, if set.
///
/// On success (`LmStatus::Ok`), `*out` is set to the specs's ctypes. If the specs
/// has no ctypes, returns `LmStatus::Absent` and leaves `*out` unchanged.
///
/// # Safety
/// `specs` must be a valid specs handle created by this library.
/// `out` must point to writable `LmCtypeBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_specs_ctypes(
    specs: *const LmSpecs,
    out: *mut LmCtypeBuffer,
) -> LmStatus {
    guard(|| {
        let specs = unsafe { ref_from_c(specs as *const Specs, "specs") }?;
        let out = unsafe { out_from_c(out) }?;
        match specs.constraints {
            Some(cs) => {
                *out =
                    ctype_buffer_from_vec(cs.into_iter().map(ctype_to_c).collect::<Vec<LmCtype>>());
                Ok(())
            }
            None => Err(CapiError::new(LmStatus::Absent, "specs has no ctypes")),
        }
    })
}

/// Returns the specs's max degree, if set.
///
/// On success (`LmStatus::Ok`), `*out` is set to the specs's max degree. If the specs
/// has no max degree, returns `LmStatus::Absent` and leaves `*out` unchanged.
///
/// # Safety
/// `specs` must be a valid specs handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_specs_max_degree(specs: *const LmSpecs, out: *mut usize) -> LmStatus {
    guard(|| {
        let specs = unsafe { ref_from_c(specs as *const Specs, "specs") }?;
        let out = unsafe { out_from_c(out) }?;
        match specs.max_degree {
            Some(d) => {
                *out = d;
                Ok(())
            }
            None => Err(CapiError::new(LmStatus::Absent, "specs has no max_degree")),
        }
    })
}

/// Returns the specs's max constraint degree, if set.
///
/// On success (`LmStatus::Ok`), `*out` is set to the specs's max degree. If the specs
/// has no max degree, returns `LmStatus::Absent` and leaves `*out` unchanged.
///
/// # Safety
/// `specs` must be a valid specs handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_specs_max_constraint_degree(
    specs: *const LmSpecs,
    out: *mut usize,
) -> LmStatus {
    guard(|| {
        let specs = unsafe { ref_from_c(specs as *const Specs, "specs") }?;
        let out = unsafe { out_from_c(out) }?;
        match specs.max_constraint_degree {
            Some(d) => {
                *out = d;
                Ok(())
            }
            None => Err(CapiError::new(
                LmStatus::Absent,
                "specs has no max_constraint_degree",
            )),
        }
    })
}

/// Returns the specs's max num variables, if set.
///
/// On success (`LmStatus::Ok`), `*out` is set to the specs's max num variables.
/// If the specs has no max num variables, returns `LmStatus::Absent` and leaves `*out` unchanged.
///
/// # Safety
/// `specs` must be a valid specs handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_specs_max_num_variables(
    specs: *const LmSpecs,
    out: *mut usize,
) -> LmStatus {
    guard(|| {
        let specs = unsafe { ref_from_c(specs as *const Specs, "specs") }?;
        let out = unsafe { out_from_c(out) }?;
        match specs.max_num_variables {
            Some(n) => {
                *out = n;
                Ok(())
            }
            None => Err(CapiError::new(
                LmStatus::Absent,
                "specs has no max_num_variables",
            )),
        }
    })
}
