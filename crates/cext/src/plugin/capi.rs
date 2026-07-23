//! #[no_mangle]` C entry points for plugin registration.

use std::ffi::{c_char, c_void};

use lunamodel_capi::{
    LmStatus,
    transpiler::{LmBackwardFn, LmPassKind},
};

use crate::{
    error::{CapiError, guard},
    utils::str_from_c,
};

/// Registers a reversal handler for a plugin pass, keyed by its stable id.
///
/// # Safety
/// `id` must be a valid NUL-terminated UTF-8 C string. `func`/`user_data` must stay valid for as
/// long as any record by the pass may be reversed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_register_plugin_backward(
    kind: LmPassKind,
    id: *const c_char,
    func: LmBackwardFn,
    user_data: *mut c_void,
) -> LmStatus {
    guard(|| {
        lunamodel_cext_plugin::ensure_registered();
        let id = unsafe { str_from_c(id, "id") }?.to_owned();
        match kind {
            LmPassKind::Transformation => {
                lunamodel_cext_plugin::register_transform(id, func, user_data);
                Ok(())
            }
            _ => Err(CapiError::new(
                LmStatus::InvalidArgument,
                "only Transformation plugin backward handlers are supported yet",
            )),
        }
    })
}
