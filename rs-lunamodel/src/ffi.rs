//! Explicit ownership transfers for opaque handles crossing an FFI adapter.
//!
//! Normal consumers should use the facade types directly. These operations are
//! for bindings which temporarily transfer an owned handle through another ABI,
//! such as the PyCapsule bridge used by `pyo3-lunamodel`.

/// An owning facade wrapper around one opaque C handle.
///
/// # Safety
///
/// Implementations must store exactly one owned handle, transfer it without
/// running `Drop`, and release it through their normal `Drop` implementation.
pub unsafe trait OwnedHandle: Sized {
    type Raw;

    /// Adopts a non-null owned handle without validating its provenance.
    ///
    /// # Safety
    /// `handle` must be a live, uniquely owned handle of the correct type from
    /// the currently installed LunaModel C API.
    unsafe fn from_raw_owned_unchecked(handle: *mut Self::Raw) -> Self;

    /// Transfers this wrapper's owned handle to the caller.
    fn into_raw_owned(self) -> *mut Self::Raw;
}

/// Adopts an owned handle, rejecting null.
///
/// # Safety
/// A non-null `handle` must satisfy [`OwnedHandle::from_raw_owned_unchecked`].
pub unsafe fn adopt<H: OwnedHandle>(handle: *mut H::Raw) -> crate::Result<H> {
    if handle.is_null() {
        return Err(crate::Error {
            status: crate::Status::NullPointer,
            message: "cannot adopt a null LunaModel handle".to_owned(),
            kind: None,
        });
    }

    Ok(unsafe { H::from_raw_owned_unchecked(handle) })
}

/// Releases an owned raw handle through its facade wrapper.
///
/// Passing null is a no-op.
///
/// # Safety
/// A non-null `handle` must satisfy [`OwnedHandle::from_raw_owned_unchecked`]
/// and must not be used or released again.
pub unsafe fn release<H: OwnedHandle>(handle: *mut H::Raw) {
    if !handle.is_null() {
        drop(unsafe { H::from_raw_owned_unchecked(handle) });
    }
}
