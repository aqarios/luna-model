use crate::utils::{handle_to_c, out_from_c, ref_from_c, str_from_c};
use std::ffi::c_char;

use luna_model::core::ArcEnv;

use crate::{
    LmEnv, LmEnvId, LmStatus, LmVref, LmVrefBuffer, LmVtype,
    buffer::vref_buffer_from_vec,
    error::{CapiError, guard},
    utils::vtype_to_c,
};

/// Returns the number of variables in the environment.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_len(env: *const LmEnv, out: *mut usize) -> LmStatus {
    guard(|| {
        let env = unsafe { ref_from_c(env as *const ArcEnv, "env") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = env.len();
        Ok(())
    })
}

/// Returns `true` when the environment contains no variables.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_is_empty(env: *const LmEnv, out: *mut bool) -> LmStatus {
    guard(|| {
        let env = unsafe { ref_from_c(env as *const ArcEnv, "env") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = env.is_empty();
        Ok(())
    })
}

/// Returns all variables as environment-bound references.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `LmVrefBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_vars(env: *const LmEnv, out: *mut LmVrefBuffer) -> LmStatus {
    guard(|| {
        let env = unsafe { ref_from_c(env as *const ArcEnv, "env") }?;
        let out = unsafe { out_from_c(out) }?;
        let vars = env
            .vars()
            .into_iter()
            .map(handle_to_c)
            .collect::<Vec<*mut LmVref>>();
        *out = vref_buffer_from_vec(vars);
        Ok(())
    })
}

/// Returns the unique id of the environment.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `LmEnvId` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_id(env: *const LmEnv, out: *mut LmEnvId) -> LmStatus {
    guard(|| {
        let env = unsafe { ref_from_c(env as *const ArcEnv, "env") }?;
        let out = unsafe { out_from_c(out) }?;
        let id = env.id();
        *out = LmEnvId {
            hi: (id >> 64) as u64,
            lo: id as u64,
        };
        Ok(())
    })
}

/// Looks up a variable by name and returns a bound variable reference.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `out` must point to writable `LmVref *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_lookup(
    env: *const LmEnv,
    name: *const c_char,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let env = unsafe { ref_from_c(env as *const ArcEnv, "env") }?;
        let name = unsafe { str_from_c(name, "name") }?;
        let out = unsafe { out_from_c(out) }?;
        let var = env.lookup(name).map_err(CapiError::errored)?;
        *out = handle_to_c(var);
        Ok(())
    })
}

/// Returns a variable reference for a known index.
///
/// This does not validate the index immediately. Any later metadata access
/// through the returned `VarRef` will fail if the index does not exist.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `LmVref *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_get(
    env: *const LmEnv,
    vid: u32,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let env = unsafe { ref_from_c(env as *const ArcEnv, "env") }?;
        let out = unsafe { out_from_c(out) }?;
        let var = env.get(vid);
        *out = handle_to_c(var);
        Ok(())
    })
}

/// Returns whether a variable with the given name exists.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_contains(
    env: *const LmEnv,
    name: *const c_char,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let env = unsafe { ref_from_c(env as *const ArcEnv, "env") }?;
        let name = unsafe { str_from_c(name, "name") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = env.contains(name);
        Ok(())
    })
}

/// Returns the variable type for a named variable.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `out` must point to writable `LmVtype` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_vtype_of(
    env: *const LmEnv,
    name: *const c_char,
    out: *mut LmVtype,
) -> LmStatus {
    guard(|| {
        let env = unsafe { ref_from_c(env as *const ArcEnv, "env") }?;
        let name = unsafe { str_from_c(name, "name") }?;
        let out = unsafe { out_from_c(out) }?;
        let vt = env.vtype_of(name).map_err(CapiError::errored)?;
        *out = vtype_to_c(vt);
        Ok(())
    })
}
