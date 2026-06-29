use luna_model::core::{ConstraintCollection, prelude::ContentEquality};

use crate::{
    LmCC, LmEnv, LmStatus,
    env::utils::environment_from_c,
    error::{CapiError, guard},
};

/// Clones a constraint-collection into a new, independent handle written to `out`.
///
/// # Safety
/// `cc` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_clone(cc: *const LmCC, out: *mut *mut LmCC) -> LmStatus {
    guard(|| {
        let cc = unsafe { (cc as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(cc.clone())) as *mut LmCC;
        Ok(())
    })
}

/// Deep clones a constraint-collection into a new, independent handle written to `out`.
///
/// # Safety
/// `cc` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_deep_clone(
    cc: *const LmCC,
    env: *const LmEnv,
    out: *mut *mut LmCC,
) -> LmStatus {
    guard(|| {
        let cc = unsafe { (cc as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let env = unsafe { environment_from_c(env)? };
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(cc.deep_clone(env.clone()))) as *mut LmCC;
        Ok(())
    })
}

/// Compares semantic contents while ignoring environment identity.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle.
/// `other` must be a valid constraint-collection handle.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_eq(cc: *const LmCC, other: *const LmCC, out: *mut bool) -> LmStatus {
    guard(|| {
        let cc = unsafe { (cc as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let other = unsafe { (other as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        *out = cc.eq(other);
        Ok(())
    })
}

/// Compares semantic contents while ignoring environment identity.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle.
/// `other` must be a valid constraint-collection handle.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_equal_contents(
    cc: *const LmCC,
    other: *const LmCC,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let cc = unsafe { (cc as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let other = unsafe { (other as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        *out = cc.equal_contents(other);
        Ok(())
    })
}
