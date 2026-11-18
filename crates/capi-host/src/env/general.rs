use lunamodel_core::prelude::ContentEquality;

use crate::{
    LmEnv, LmStatus,
    error::guard,
    utils::{out_from_c, storage_from_c, storage_out_from_c},
};

/// Clones an environment into a new, independent handle written to `out`.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `LmEnv *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_clone(env: *const LmEnv, out: *mut *mut LmEnv) -> LmStatus {
    guard(|| {
        let env = unsafe { storage_from_c(env, "env") }?;
        unsafe { storage_out_from_c(out, env.clone()) }
    })
}

/// Deep clones an environment into a new, independent handle written to `out`.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `LmEnv *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_deep_clone(env: *const LmEnv, out: *mut *mut LmEnv) -> LmStatus {
    guard(|| {
        let env = unsafe { storage_from_c(env, "env") }?;
        unsafe { storage_out_from_c(out, env.deep_clone()) }
    })
}

/// Compares complete environment values, including identity and allocator state.
///
/// # Safety
/// `env` and `other` must be valid handles and `out` writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_eq(
    env: *const LmEnv,
    other: *const LmEnv,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let env = unsafe { storage_from_c(env, "env") }?;
        let other = unsafe { storage_from_c(other, "other") }?;
        *unsafe { out_from_c(out) }? = *env.env.read() == *other.env.read();
        Ok(())
    })
}

/// Compares two environments for content equality.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `other` must be a valid environment handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_equal_contents(
    env: *const LmEnv,
    other: *const LmEnv,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let env = unsafe { storage_from_c(env, "env") }?;
        let other = unsafe { storage_from_c(other, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = env.equal_contents(other);
        Ok(())
    })
}
