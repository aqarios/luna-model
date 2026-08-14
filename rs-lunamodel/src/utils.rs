use std::ffi::{CString, c_char};

use lunamodel_capi::{LmEnvId, LmSol, LmVref};

use crate::{
    Error, Result, Solution, Status, VarRef,
    traits::{LmTryFrom, PtrList, ToPtrList, TryToCString, TryToOptCString},
};

fn interior_nul() -> Error {
    Error {
        status: Status::InvalidArgument,
        message: "string contains an interior NUL byte".to_string(),
        kind: None,
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

/// Borrows an optional owned array as a nullable data pointer.
///
/// `None` yields null; `Some` yields the vec's data pointer, which is non-null
/// even for an empty vec — the ABI distinguishes an explicit empty set
/// (non-null, len 0) from an unspecified one (null). The `Option<Vec<T>>` must
/// outlive the FFI call.
pub(crate) fn opt_vec_ptr<T>(opt: &Option<Vec<T>>) -> *const T {
    opt.as_ref().map_or(std::ptr::null(), |v| v.as_ptr())
}

/// Length companion to [`opt_vec_ptr`]; `None` yields 0.
pub(crate) fn opt_vec_len<T>(opt: &Option<Vec<T>>) -> usize {
    opt.as_ref().map_or(0, |v| v.len())
}

impl ToPtrList for &[VarRef] {
    type Elem = LmVref;
    fn to_ptr_list(self) -> Result<PtrList<LmVref>> {
        // The handle pointers are owned by the borrowed slice, so no extra backing is needed.
        Ok(PtrList::from_ptrs(
            self.iter().map(|v| v.as_ptr()).collect(),
            Vec::new(),
        ))
    }
}

impl ToPtrList for &[Solution] {
    type Elem = LmSol;
    fn to_ptr_list(self) -> Result<PtrList<LmSol>> {
        // The handle pointers are owned by the borrowed slice, so no extra backing is needed.
        Ok(PtrList::from_ptrs(
            self.iter().map(|v| v.as_ptr()).collect(),
            Vec::new(),
        ))
    }
}

impl ToPtrList for &[String] {
    type Elem = c_char;
    fn to_ptr_list(self) -> Result<PtrList<c_char>> {
        // The CStrings own the bytes; the pointers borrow from them, so they are
        // stored alongside the pointer array and kept alive together.
        let owners: Vec<CString> = self
            .iter()
            .map(|s| s.as_str().try_to_cstring())
            .collect::<Result<_>>()?;
        let ptrs = owners.iter().map(|c| c.as_ptr()).collect();
        Ok(PtrList::from_ptrs(ptrs, owners))
    }
}

impl LmTryFrom<LmEnvId> for u128 {
    fn lm_try_from(value: LmEnvId) -> Result<Self> {
        Ok(((value.hi as u128) << 64) | (value.lo as u128))
    }
}
