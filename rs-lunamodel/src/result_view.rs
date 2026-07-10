//! Owning wrapper over a LunaModel result_view handle.
//!
//! Mirrors the Rust `ResultView`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `ResultView` still return `Result` here.

use lunamodel_cext_vtable::LmResultView;
use rs_lunamodel_derive::RawHandle;

/// Owning handle to a LunaModel result_view (frees on drop).
#[derive(RawHandle)]
pub struct ResultView {
    handle: *mut LmResultView,
}

impl Drop for ResultView {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, result_view_free)
        }
    }
}
