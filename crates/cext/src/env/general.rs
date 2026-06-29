use luna_model::core::{ArcEnv, prelude::ContentEquality};

use crate::{
    LmEnv, LmStatus,
    error::{CapiError, guard},
};

/// Clones an environment into a new, independent handle written to `out`.
///
/// # Safety
/// `env` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_clone(env: *const LmEnv, out: *mut *mut LmEnv) -> LmStatus {
    guard(|| {
        let env = unsafe { (env as *const ArcEnv).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("env"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(env.clone())) as *mut LmEnv;
        Ok(())
    })
}

/// Deep clones an environment into a new, independent handle written to `out`.
///
/// # Safety
/// `env` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_deep_clone(env: *const LmEnv, out: *mut *mut LmEnv) -> LmStatus {
    guard(|| {
        let env = unsafe { (env as *const ArcEnv).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("env"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(env.deep_clone())) as *mut LmEnv;
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
        let env = unsafe { (env as *const ArcEnv).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("env"))?;
        let other = unsafe { (other as *const ArcEnv).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = env.equal_contents(other);
        Ok(())
    })
}
