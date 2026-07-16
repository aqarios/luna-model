use std::ffi::c_char;

use crate::{
    LmEnv, LmEnvId, LmStatus, LmStrBuffer, LmVref, LmVrefBuffer, LmVtype,
    buffer::{str_buffer_from_vec, vref_buffer_from_vec},
    error::{CapiError, guard},
    utils::vtype_to_c,
    utils::{handle_to_c, out_from_c, slice_from_c, storage_from_c, str_from_c},
};

/// Sorts names by their variable order in the environment.
///
/// # Safety
/// `env` must be valid, `names` must describe `len` valid C strings, and `out`
/// must be writable buffer storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_sort(
    env: *const LmEnv,
    names: *const *const c_char,
    len: usize,
    out: *mut LmStrBuffer,
) -> LmStatus {
    guard(|| {
        let env = unsafe { storage_from_c(env, "env") }?;
        let names = unsafe { slice_from_c(names, len, "names") }?
            .iter()
            .map(|&name| unsafe { str_from_c(name, "name") }.map(str::to_owned))
            .collect::<Result<Vec<_>, _>>()?;
        let out = unsafe { out_from_c(out) }?;
        *out = str_buffer_from_vec(env.sort(names));
        Ok(())
    })
}

/// Returns the next variable index.
///
/// # Safety
/// `env` must be a valid environment handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_next_idx(env: *const LmEnv, out: *mut u32) -> LmStatus {
    guard(|| {
        let env = unsafe { storage_from_c(env, "env") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = env.env.read().next_idx();
        Ok(())
    })
}

/// Returns the number of variables in the environment.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_len(env: *const LmEnv, out: *mut usize) -> LmStatus {
    guard(|| {
        let env = unsafe { storage_from_c(env, "env") }?;
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
        let env = unsafe { storage_from_c(env, "env") }?;
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
        let env = unsafe { storage_from_c(env, "env") }?;
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
        let env = unsafe { storage_from_c(env, "env") }?;
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
        let env = unsafe { storage_from_c(env, "env") }?;
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
        let env = unsafe { storage_from_c(env, "env") }?;
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
        let env = unsafe { storage_from_c(env, "env") }?;
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
        let env = unsafe { storage_from_c(env, "env") }?;
        let name = unsafe { str_from_c(name, "name") }?;
        let out = unsafe { out_from_c(out) }?;
        let vt = env.vtype_of(name).map_err(CapiError::errored)?;
        *out = vtype_to_c(vt);
        Ok(())
    })
}
