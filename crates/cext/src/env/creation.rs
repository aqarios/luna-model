use crate::utils::{drop_from_c, handle_to_c};
use luna_model::core::ArcEnv;

use crate::LmEnv;

/// Frees an environment handle. Passing null is a no-op.
///
/// # Safety
/// `env` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_free(env: *mut LmEnv) {
    unsafe { drop_from_c(env as *mut ArcEnv) }
}

/// Creates a new, empty environment.
///
/// Returns a non-null handle that must be released with [`lm_env_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_env_default() -> *mut LmEnv {
    handle_to_c(ArcEnv::default())
}
