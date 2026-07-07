use std::ffi::{CString, c_char};

use lunamodel_cext_vtable::LmEnvId;

use crate::{
    Error, Result, Status,
    traits::{LmTryFrom, TryToCString, TryToOptCString},
};

fn interior_nul() -> Error {
    Error {
        status: Status::InvalidArgument,
        message: "string contains an interior NUL byte".to_string(),
    }
}

impl TryToCString for String {
    fn try_to_cstring(self) -> Result<CString> {
        CString::new(self).map_err(|_| interior_nul())
    }
}

impl TryToCString for &str {
    fn try_to_cstring(self) -> Result<CString> {
        CString::new(self).map_err(|_| interior_nul())
    }
}

impl TryToOptCString for Option<String> {
    fn try_to_opt_cstring(self) -> Result<Option<CString>> {
        self.map(|s| s.try_to_cstring()).transpose()
    }
}

impl TryToOptCString for Option<&str> {
    fn try_to_opt_cstring(self) -> Result<Option<CString>> {
        self.map(|s| s.try_to_cstring()).transpose()
    }
}

/// Borrows an owned optional C string as a nullable `*const c_char`.
///
/// `None` yields null. The `Option<CString>` must outlive the FFI call; the `?!`
/// macro sigil ensures this by keeping it alive as a statement-scoped temporary.
pub(crate) fn opt_cstring_ptr(opt: &Option<CString>) -> *const c_char {
    opt.as_ref().map_or(std::ptr::null(), |c| c.as_ptr())
}

impl LmTryFrom<LmEnvId> for u128 {
    fn lm_try_from(value: LmEnvId) -> Result<Self> {
        Ok(((value.hi as u128) << 64) | (value.lo as u128))
    }
}
