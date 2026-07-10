use std::ffi::{CString, c_char};

use lunamodel_cext_vtable::{LmEnvId, LmSol, LmVref};
use num::ToPrimitive;

use crate::{
    Error, Result, Solution, Status, VarRef,
    traits::{LmTryFrom, PtrList, ToPtrList, TryToCString, TryToOptCString},
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

impl LmTryFrom<f64> for u8 {
    fn lm_try_from(value: f64) -> Result<Self> {
        if value.fract() != 0.0 {
            return Err(Error {
                status: Status::InvalidArgument,
                message: format!("{value} is not an integer"),
            });
        }
        value.to_u8().ok_or_else(|| Error {
            status: Status::InvalidArgument,
            message: format!("{value} is out of range for u8"),
        })
    }
}

impl LmTryFrom<f64> for i8 {
    fn lm_try_from(value: f64) -> Result<Self> {
        if value.fract() != 0.0 {
            return Err(Error {
                status: Status::InvalidArgument,
                message: format!("{value} is not an integer"),
            });
        }
        value.to_i8().ok_or_else(|| Error {
            status: Status::InvalidArgument,
            message: format!("{value} is out of range for i8"),
        })
    }
}

impl LmTryFrom<f64> for i64 {
    fn lm_try_from(value: f64) -> Result<Self> {
        if value.fract() != 0.0 {
            return Err(Error {
                status: Status::InvalidArgument,
                message: format!("{value} is not an integer"),
            });
        }
        value.to_i64().ok_or_else(|| Error {
            status: Status::InvalidArgument,
            message: format!("{value} is out of range for i64"),
        })
    }
}
