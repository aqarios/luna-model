use luna_model::core::ArcEnv;

use crate::{
    LmEnv, LmStatus, LmVref, LmVrefBuffer,
    buffer::vref_buffer_from_vec,
    error::{CapiError, guard},
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
