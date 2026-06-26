use luna_model::core::ArcEnv;

use crate::LmEnvironment;

/// Frees an environment handle. Passing null is a no-op.
///
/// # Safety
/// `model` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_environment_free(env: *mut LmEnvironment) {
    if !env.is_null() {
        drop(unsafe { Box::from_raw(env as *mut ArcEnv) });
    }
}

/// Creates a new, empty environment.
///
/// Returns a non-null handle that must be released with [`lm_environment_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_environment_default() -> *mut LmEnvironment {
    Box::into_raw(Box::new(ArcEnv::default())) as *mut LmEnvironment
}
