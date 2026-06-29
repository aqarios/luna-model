use std::ffi::{CStr, c_char};

use luna_model::core::{
    ArcEnv,
    prelude::{LazyBounds, VarRef},
};

use crate::{
    LmEnv, LmLazyBounds, LmStatus, LmVref, LmVtype,
    error::{CapiError, guard},
    utils::vtype_from_c,
};

/// Inserts a new variable and returns a reference tied to this environment.
///
/// # Safety
/// `env` must be a valid mutable environment handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `vtype` must point to a valid `LmVtype`.
/// `bounds`, when non-null, must be a valid lazy-bounds handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_insert(
    env: *mut LmEnv,
    name: *const c_char,
    vtype: *const LmVtype,
    bounds: *const LmLazyBounds,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let env = unsafe { (env as *mut ArcEnv).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("env"))?;
        if name.is_null() {
            return Err(CapiError::null_pointer("name"));
        }
        let name = unsafe { CStr::from_ptr(name) }.to_str().map_err(|e| {
            CapiError::new(
                LmStatus::InvalidArgument,
                format!("`name` must be valid UTF-8: {e}"),
            )
        })?;
        let vtype = vtype_from_c(
            *unsafe { vtype.as_ref() }.ok_or_else(|| CapiError::null_pointer("vtype"))?,
        );
        let bounds = if bounds.is_null() {
            None
        } else {
            let bounds = unsafe { (bounds as *const LazyBounds).as_ref() }
                .ok_or_else(|| CapiError::null_pointer("bounds"))?;
            Some((*bounds).into())
        };
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let var = env
            .insert(name, vtype, bounds)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        *out = Box::into_raw(Box::new(var)) as *mut LmVref;
        Ok(())
    })
}

/// Inserts a new variable, deriving a fallback name when necessary and returns a reference tied to this environment.
///
/// # Safety
/// `env` must be a valid mutable environment handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `vtype` must point to a valid `LmVtype`.
/// `bounds`, when non-null, must be a valid lazy-bounds handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_insert_with_fallback(
    env: *mut LmEnv,
    name: *const c_char,
    vtype: *const LmVtype,
    bounds: *const LmLazyBounds,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let env = unsafe { (env as *mut ArcEnv).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("env"))?;
        if name.is_null() {
            return Err(CapiError::null_pointer("name"));
        }
        let name = unsafe { CStr::from_ptr(name) }.to_str().map_err(|e| {
            CapiError::new(
                LmStatus::InvalidArgument,
                format!("`name` must be valid UTF-8: {e}"),
            )
        })?;
        let vtype = vtype_from_c(
            *unsafe { vtype.as_ref() }.ok_or_else(|| CapiError::null_pointer("vtype"))?,
        );
        let bounds = if bounds.is_null() {
            None
        } else {
            let bounds = unsafe { (bounds as *const LazyBounds).as_ref() }
                .ok_or_else(|| CapiError::null_pointer("bounds"))?;
            Some((*bounds).into())
        };
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let var = env
            .insert_with_fallback(name, vtype, bounds, None)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        *out = Box::into_raw(Box::new(var)) as *mut LmVref;
        Ok(())
    })
}

/// Inserts the inverted companion of a binary variable.
///
/// # Safety
/// `env` must be a valid mutable environment handle created by this library.
/// `vref` must point to a valid `LmVref`.
/// `out` must point to writable `LmVref` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_insert_inverted(
    env: *mut LmEnv,
    vref: *const LmVref,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let env = unsafe { (env as *mut ArcEnv).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("env"))?;
        let vref = unsafe { (vref as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("vref"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let var = env
            .insert_inverted(vref)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        *out = Box::into_raw(Box::new(var)) as *mut LmVref;
        Ok(())
    })
}

/// Removes the referenced variable from the environment.
///
/// # Safety
/// `env` must be a valid mutable environment handle created by this library.
/// `vref` must point to a valid `LmVref`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_remove(env: *mut LmEnv, vref: *const LmVref) -> LmStatus {
    guard(|| {
        let env = unsafe { (env as *mut ArcEnv).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("env"))?;
        let vref = unsafe { (vref as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("vref"))?;
        env.remove(vref);
        Ok(())
    })
}
