use std::ffi::c_char;

use luna_model::core::prelude::{LazyBounds, VarRef};

use crate::{
    LmEnv, LmLazyBounds, LmStatus, LmVref, LmVtype,
    error::{CapiError, guard},
    utils::vtype_from_c,
    utils::{handle_to_c, out_from_c, ref_from_c, slice_from_c, storage_from_c, str_from_c},
};

/// Inserts a new variable and returns a reference tied to this environment.
///
/// # Safety
/// `env` must be a valid mutable environment handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `bounds`, when non-null, must be a valid lazy-bounds handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_insert(
    env: *mut LmEnv,
    name: *const c_char,
    vtype: LmVtype,
    bounds: *const LmLazyBounds,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let mut env = unsafe { storage_from_c(env, "env") }?.clone();
        let name = unsafe { str_from_c(name, "name") }?;
        let vtype = vtype_from_c(vtype);
        let bounds = if bounds.is_null() {
            None
        } else {
            let bounds = unsafe { ref_from_c(bounds as *const LazyBounds, "bounds") }?;
            Some(*bounds)
        };
        let out = unsafe { out_from_c(out) }?;
        let var = env
            .insert(name, vtype, bounds)
            .map_err(CapiError::errored)?;
        *out = handle_to_c(var);
        Ok(())
    })
}

/// Inserts a new variable, deriving a fallback name when necessary and returns a reference tied to this environment.
///
/// # Safety
/// `env` must be a valid mutable environment handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `bounds`, when non-null, must be a valid lazy-bounds handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_insert_with_fallback(
    env: *mut LmEnv,
    name: *const c_char,
    vtype: LmVtype,
    bounds: *const LmLazyBounds,
    enc: *const u64,
    enc_len: usize,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let mut env = unsafe { storage_from_c(env, "env") }?.clone();
        let name = unsafe { str_from_c(name, "name") }?;
        let vtype = vtype_from_c(vtype);
        let bounds = if bounds.is_null() {
            None
        } else {
            let bounds = unsafe { ref_from_c(bounds as *const LazyBounds, "bounds") }?;
            Some(*bounds)
        };
        let enc = if enc.is_null() {
            None
        } else {
            Some(unsafe { slice_from_c(enc, enc_len, "enc") }?)
        };
        let out = unsafe { out_from_c(out) }?;
        let var = env
            .insert_with_fallback(name, vtype, bounds, enc)
            .map_err(CapiError::errored)?;
        *out = handle_to_c(var);
        Ok(())
    })
}

/// Updates the public `Variable::inverted` field through an owned-environment
/// mutation proxy.
///
/// # Safety
/// `env` must be a valid mutable environment handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_set_inverted(
    env: *mut LmEnv,
    index: u32,
    present: bool,
    inverted: u32,
) -> LmStatus {
    guard(|| {
        let env = unsafe { storage_from_c(env, "env") }?;
        env.env
            .write()
            .get_mut(index)
            .map_err(CapiError::errored)?
            .inverted = present.then_some(inverted);
        Ok(())
    })
}

/// Inserts the inverted companion of a binary variable.
///
/// # Safety
/// `env` must be a valid mutable environment handle created by this library.
/// `vref` must point to a valid `LmVref`.
/// `out` must point to writable `LmVref *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_insert_inverted(
    env: *mut LmEnv,
    vref: *const LmVref,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let mut env = unsafe { storage_from_c(env, "env") }?.clone();
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let out = unsafe { out_from_c(out) }?;
        let var = env.insert_inverted(vref).map_err(CapiError::errored)?;
        *out = handle_to_c(var);
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
        let mut env = unsafe { storage_from_c(env, "env") }?.clone();
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        env.remove(vref);
        Ok(())
    })
}
