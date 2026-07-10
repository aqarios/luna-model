//! Owning wrapper over a LunaModel sample_view handle.
//!
//! Mirrors the Rust `SampleView`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Sample`]; the C ABI can always report a status, so methods
//! that are infallible on the native `SampleView` still return `Sample` here.

use lunamodel_cext_vtable::LmSampleView;
use rs_lunamodel_derive::RawHandle;

/// Owning handle to a LunaModel sample_view (frees on drop).
#[derive(RawHandle)]
pub struct SampleView {
    handle: *mut LmSampleView,
}

impl Drop for SampleView {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, sample_view_free)
        }
    }
}

