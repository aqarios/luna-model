use luna_model::core::prelude::VarRef;

use crate::{
    LmBounds, LmBuffer, LmStatus, LmVref, LmVtype,
    buffer::buffer_from_vec,
    error::{CapiError, guard},
    utils::vtype_to_c,
    utils::{handle_to_c, out_from_c, ref_from_c},
};

/// Returns the id of the vref.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `uint32_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_id(vref: *const LmVref, out: *mut u32) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = vref.id();
        Ok(())
    })
}

/// Writes `true` to `out` when the referenced variable still exists in its
/// environment, `false` otherwise.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_check_living(vref: *const LmVref, out: *mut bool) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = vref.check_living().is_ok();
        Ok(())
    })
}

/// Returns a copy of the vref's name as an owned UTF-8 byte buffer.
///
/// The returned buffer is written to `out` and must be released with
/// `lm_buffer_free`.
///
/// The buffer is not guaranteed to be NUL-terminated. C callers should use
/// `buffer.data` together with `buffer.len`.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_name(vref: *const LmVref, out: *mut LmBuffer) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;

        let out = unsafe { out_from_c(out) }?;

        let name = vref.name().map_err(CapiError::errored)?;
        *out = buffer_from_vec(name.into_bytes());
        Ok(())
    })
}

/// Returns a copy of the vref's vtype.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmVtype` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_vtype(vref: *const LmVref, out: *mut LmVtype) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;

        let out = unsafe { out_from_c(out) }?;

        let vtype = vref.vtype().map_err(CapiError::errored)?;
        *out = vtype_to_c(vtype);

        Ok(())
    })
}

/// Returns the vref's bounds.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmBounds *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_bounds(vref: *const LmVref, out: *mut *mut LmBounds) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;

        let out = unsafe { out_from_c(out) }?;

        let bounds = vref.bounds().map_err(CapiError::errored)?;
        *out = handle_to_c(bounds);

        Ok(())
    })
}

/// Returns the vref's inverted companion if one exists.
///
/// On success, `*out` is set to a newly allocated `LmVref` handle for the
/// inverted companion, or to NULL if the variable has no inverted companion.
/// A non-null handle is owned by the caller and must be freed like any other
/// vref handle.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmVref *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_inverted(vref: *const LmVref, out: *mut *mut LmVref) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;

        let out = unsafe { out_from_c(out) }?;

        let companion = vref.inverted().map_err(CapiError::errored)?;

        *out = companion.map(handle_to_c).unwrap_or(std::ptr::null_mut());

        Ok(())
    })
}

/// Returns a stable hash derived from the variable name.
///
/// The hash intentionally uses the resolved name rather than the raw index so
/// it remains meaningful across equivalent environments.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `uint64_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_hash(vref: *const LmVref, out: *mut u64) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;

        let out = unsafe { out_from_c(out) }?;

        *out = vref.hash().map_err(CapiError::errored)?;

        Ok(())
    })
}
