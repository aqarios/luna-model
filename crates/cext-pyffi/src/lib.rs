//! Generic PyCapsule transport for owned LunaModel C handles.
//!
//! A capsule contains one consumable handle slot. If the receiver does not
//! consume the handle, the capsule releases it through the supplied C-API
//! destructor when Python destroys the capsule.

use std::{
    ffi::{CStr, c_void},
    marker::PhantomData,
    mem, ptr,
};

use lunamodel_cext_vtable::{LmModel, LmSol};
use pyo3::{
    Bound, PyResult, Python,
    exceptions::PyValueError,
    types::{PyCapsule, PyCapsuleMethods},
};

/// Name of the capsule exporting the process-wide LunaModel C API table.
pub const C_API_CAPSULE_NAME: &CStr = c"luna_model._lm._C_API";

/// Associates an opaque C handle with its versioned PyCapsule name.
pub trait CapsuleKind: Sized + 'static {
    const CAPSULE_NAME: &'static CStr;
}

impl CapsuleKind for LmModel {
    const CAPSULE_NAME: &'static CStr = c"luna_model.capi.Model.v1";
}

impl CapsuleKind for LmSol {
    const CAPSULE_NAME: &'static CStr = c"luna_model.capi.Solution.v1";
}

#[repr(C)]
struct HandleSlot {
    handle: *mut c_void,
}

#[repr(C)]
struct CapsuleOwner<H: CapsuleKind> {
    // This must stay first: PyCapsule exposes a pointer to the stored value.
    slot: HandleSlot,
    release: unsafe fn(*mut H),
    _kind: PhantomData<H>,
}

// The handle lifecycle is synchronized by the host-side handle implementation.
unsafe impl<H: CapsuleKind> Send for CapsuleOwner<H> {}

impl<H: CapsuleKind> Drop for CapsuleOwner<H> {
    fn drop(&mut self) {
        let handle = mem::replace(&mut self.slot.handle, ptr::null_mut()).cast::<H>();
        if !handle.is_null() {
            unsafe { (self.release)(handle) };
        }
    }
}

/// Creates a named capsule which owns `handle` until it is consumed.
pub fn new_capsule<H: CapsuleKind>(
    py: Python<'_>,
    handle: *mut H,
    release: unsafe fn(*mut H),
) -> PyResult<Bound<'_, PyCapsule>> {
    PyCapsule::new_with_value(
        py,
        CapsuleOwner {
            slot: HandleSlot {
                handle: handle.cast(),
            },
            release,
            _kind: PhantomData,
        },
        H::CAPSULE_NAME,
    )
}

/// Consumes and returns the handle stored in a named capsule.
///
/// A capsule can be consumed exactly once.
pub fn take_handle<H: CapsuleKind>(capsule: &Bound<'_, PyCapsule>) -> PyResult<*mut H> {
    let pointer = capsule.pointer_checked(Some(H::CAPSULE_NAME))?;
    let slot = unsafe { pointer.cast::<HandleSlot>().as_mut() };
    let handle = mem::replace(&mut slot.handle, ptr::null_mut()).cast::<H>();

    if handle.is_null() {
        Err(PyValueError::new_err(format!(
            "the {} capsule was already consumed",
            H::CAPSULE_NAME.to_string_lossy()
        )))
    } else {
        Ok(handle)
    }
}
