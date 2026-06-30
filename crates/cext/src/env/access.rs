use std::ffi::{CStr, c_char};

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
/// `env` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_len(env: *const LmEnv, out: *mut usize) -> LmStatus {
    guard(|| {
        let env = unsafe { (env as *const ArcEnv).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("env"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = env.len();
        Ok(())
    })
}

/// Returns `true` when the environment contains no variables.
///
/// # Safety
/// `env` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_is_empty(env: *const LmEnv, out: *mut bool) -> LmStatus {
    guard(|| {
        let env = unsafe { (env as *const ArcEnv).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("env"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = env.is_empty();
        Ok(())
    })
}

/// Returns all variables as environment-bound references.
///
/// # Safety
/// `env` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_vars(env: *const LmEnv, out: *mut LmVrefBuffer) -> LmStatus {
    guard(|| {
        let env = unsafe { (env as *const ArcEnv).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("env"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let vars = env
            .vars()
            .into_iter()
            .map(|v| Box::into_raw(Box::new(v)) as *mut LmVref)
            .collect::<Vec<*mut LmVref>>();
        *out = vref_buffer_from_vec(vars);
        Ok(())
    })
}

/// Returns the unique id of the environment.
///
/// # Safety
/// `env` must be a valid handle and `out` must point to writable `LmEnvId` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_id(env: *const LmEnv, out: *mut LmEnvId) -> LmStatus {
    guard(|| {
        let env = unsafe { (env as *const ArcEnv).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("env"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
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
/// `out` must point to writable `LmVref` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_lookup(
    env: *const LmEnv,
    name: *const c_char,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let env = unsafe { (env as *const ArcEnv).as_ref() }
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
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let var = env
            .lookup(&name)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        *out = Box::into_raw(Box::new(var)) as *mut LmVref;
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
/// `vid` must point to a valid vref id.
/// `out` must point to writable `LmVref` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_get(
    env: *const LmEnv,
    vid: u32,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let env = unsafe { (env as *const ArcEnv).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("env"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let var = env.get(vid);
        *out = Box::into_raw(Box::new(var)) as *mut LmVref;
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
        let env = unsafe { (env as *const ArcEnv).as_ref() }
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
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = env.contains(&name);
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
        let env = unsafe { (env as *const ArcEnv).as_ref() }
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
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let vt = env
            .vtype_of(&name)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        *out = vtype_to_c(vt);
        Ok(())
    })
}
