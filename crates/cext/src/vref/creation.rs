use luna_model::core::prelude::VarRef;

use crate::{
    LmEnv, LmStatus, LmVref,
    error::guard,
    utils::{drop_from_c, handle_to_c, out_from_c, storage_from_c},
};

/// Frees a vref handle. Passing null is a no-op.
///
/// # Safety
/// `vref` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_free(vref: *mut LmVref) {
    unsafe { drop_from_c(vref as *mut VarRef) }
}

/// Creates a new vref for an id and an environment.
///
/// The id is not validated immediately: metadata access through the returned
/// vref fails if the id does not exist in the environment.
///
/// `out` receives an owned vref handle that must be freed with `lm_vref_free`.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `LmVref *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_new(
    id: u32,
    env: *const LmEnv,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;

        let env = unsafe { storage_from_c(env, "env") }?;
        let vref = VarRef::new(id, env.clone());
        *out = handle_to_c(vref);
        Ok(())
    })
}
