use luna_model::core::prelude::VarRef;

use crate::{
    LmEnv, LmStatus, LmVref,
    env::utils::environment_from_c,
    error::{CapiError, guard},
};

/// Frees a vref handle. Passing null is a no-op.
///
/// # Safety
/// `vref` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_free(vref: *mut LmVref) {
    if !vref.is_null() {
        drop(unsafe { Box::from_raw(vref as *mut VarRef) });
    }
}

/// Creates a new vref for an id and an environment.
///
/// `out` receives an owned vref handle that must be freed with `lm_vref_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_new(
    id: u32,
    env: *const LmEnv,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let env = unsafe { environment_from_c(env) }?;
        let vref = VarRef::new(id, env.clone());
        *out = Box::into_raw(Box::new(vref)) as *mut LmVref;
        Ok(())
    })
}
