use std::ffi::{CString, c_char};

use crate::{
    Error, Result, Status,
    traits::{TryAsOptPtr, TryAsPtr},
};

impl TryAsPtr<c_char> for String {
    fn try_as_ptr(self) -> Result<*const c_char> {
        let name_c = CString::new(self).map_err(|_| Error {
            status: Status::InvalidArgument,
            message: "string contains an interior NUL byte".to_string(),
        })?;
        let name_ptr: *const c_char = name_c.as_ptr();
        Ok(name_ptr)
    }
}

impl TryAsPtr<c_char> for &str {
    fn try_as_ptr(self) -> Result<*const c_char> {
        let name_c = CString::new(self).map_err(|_| Error {
            status: Status::InvalidArgument,
            message: "string contains an interior NUL byte".to_string(),
        })?;
        let name_ptr: *const c_char = name_c.as_ptr();
        Ok(name_ptr)
    }
}

impl TryAsOptPtr<c_char> for Option<String> {
    fn try_as_opt_ptr(self) -> Result<*const c_char> {
        match self {
            Some(s) => Ok(s.try_as_ptr()?),
            None => Ok(std::ptr::null()),
        }
    }
}
