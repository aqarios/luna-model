use crate::utils::{handle_to_c, out_from_c, ref_from_c};
use luna_model::core::{ArcEnv, prelude::ContentEquality};

use crate::{LmEnv, LmStatus, error::guard};

/// Clones an environment into a new, independent handle written to `out`.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `LmEnv *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_clone(env: *const LmEnv, out: *mut *mut LmEnv) -> LmStatus {
    guard(|| {
        let env = unsafe { ref_from_c(env as *const ArcEnv, "env") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(env.clone());
        Ok(())
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
        let env = unsafe { ref_from_c(env as *const ArcEnv, "env") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(env.deep_clone());
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
        let env = unsafe { ref_from_c(env as *const ArcEnv, "env") }?;
        let other = unsafe { ref_from_c(other as *const ArcEnv, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = env.equal_contents(other);
        Ok(())
    })
}
