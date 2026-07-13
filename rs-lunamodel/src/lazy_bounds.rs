//! Owning wrapper over a LunaModel lazy bounds handle.
//!
//! Mirrors the Rust `LazyBounds`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `LazyBounds` still return `Result` here.

use lunamodel_cext_vtable::LmLazyBounds;
use rs_lunamodel_derive::RawHandle;

/// Owning handle to a LunaModel lazy bounds (frees on drop).
#[derive(RawHandle)]
pub struct LazyBounds {
    handle: *mut LmLazyBounds,
}

impl Drop for LazyBounds {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, lazy_bounds_free)
        }
    }
}

// pub lazy_bounds_free: unsafe extern "C" fn(*mut LmLazyBounds),
// pub lazy_bounds_default: extern "C" fn() -> *mut LmLazyBounds,
// pub lazy_bounds_new: unsafe extern "C" fn(*const LmBound, *const LmBound, *mut *mut LmLazyBounds) -> LmStatus,
// pub lazy_bounds_clone: unsafe extern "C" fn(*const LmLazyBounds, *mut *mut LmLazyBounds) -> LmStatus,
// pub lazy_bounds_lower: unsafe extern "C" fn(*const LmLazyBounds, *mut LmBound) -> LmStatus,
// pub lazy_bounds_upper: unsafe extern "C" fn(*const LmLazyBounds, *mut LmBound) -> LmStatus,
// pub lazy_bounds_concretize: unsafe extern "C" fn(*const LmLazyBounds, LmVtype, *mut *mut LmBounds) -> LmStatus,
