use luna_model::core::prelude::VarRef;

use crate::{
    LmBounds, LmBuffer, LmStatus, LmVref, LmVtype,
    buffer::buffer_from_vec,
    error::{CapiError, guard},
    utils::vtype_to_c,
};

/// Returns the id of the vref.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `u32` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_id(vref: *const LmVref, out: *mut u32) -> LmStatus {
    guard(|| {
        let vref = unsafe { (vref as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("vref"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = vref.id();
        Ok(())
    })
}

/// Returns LmStatus::Errored the if the vref is not alive, else it returns OK.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_check_living(vref: *const LmVref, out: *mut bool) -> LmStatus {
    guard(|| {
        let vref = unsafe { (vref as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("vref"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
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
        let vref = unsafe { (vref as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("vref"))?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let name = vref
            .name()
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        *out = buffer_from_vec(name.as_bytes().to_vec());
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
        let vref = unsafe { (vref as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("vref"))?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let vtype = vref
            .vtype()
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        *out = vtype_to_c(vtype);

        Ok(())
    })
}

/// Returns the vref's bounds.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmBounds` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_bounds(vref: *const LmVref, out: *mut *mut LmBounds) -> LmStatus {
    guard(|| {
        let vref = unsafe { (vref as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("vref"))?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let bounds = vref
            .bounds()
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        *out = Box::into_raw(Box::new(bounds)) as *mut LmBounds;

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
/// `out` must point to writable `*mut LmVref` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_inverted(vref: *const LmVref, out: *mut *mut LmVref) -> LmStatus {
    guard(|| {
        let vref = unsafe { (vref as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("vref"))?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let companion = vref
            .inverted()
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;

        *out = companion
            .map(|c| Box::into_raw(Box::new(c)) as *mut LmVref)
            .unwrap_or(std::ptr::null_mut());

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
/// `out` must point to writable `u64` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_hash(vref: *const LmVref, out: *mut u64) -> LmStatus {
    guard(|| {
        let vref = unsafe { (vref as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("vref"))?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        *out = vref
            .hash()
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;

        Ok(())
    })
}
