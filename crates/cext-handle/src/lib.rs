//! Internal synchronized handle storage for the LunaModel C API.
//!
//! Nothing in this crate crosses the external ABI by value. Consumers only see
//! opaque `Lm*` pointers declared by `lunamodel-cext-vtable`.

use std::sync::Arc;

use lunamodel_cext_vtable::{LmEnv, LmModel, LmSol};
use lunamodel_core::{ArcEnv, Model, Solution};
use parking_lot::RwLock;

/// Associates an opaque C handle type with its host-side Rust value.
///
/// # Safety
///
/// Each opaque handle type must have exactly one `Value` mapping in the process.
pub unsafe trait HandleKind: Sized + 'static {
    type Value: 'static;
}

unsafe impl HandleKind for LmModel {
    type Value = Model;
}

unsafe impl HandleKind for LmSol {
    type Value = Solution;
}

unsafe impl HandleKind for LmEnv {
    type Value = ArcEnv;
}

/// Shared synchronized storage behind an opaque C handle.
pub type Shared<H> = Arc<RwLock<<H as HandleKind>::Value>>;

/// Creates synchronized storage and returns an owned opaque C handle.
pub fn new_handle<H: HandleKind>(value: H::Value) -> *mut H {
    handle_from_shared::<H>(Arc::new(RwLock::new(value)))
}

/// Creates an owned opaque C handle retaining `shared`.
pub fn handle_from_shared<H: HandleKind>(shared: Shared<H>) -> *mut H {
    Box::into_raw(Box::new(shared)).cast()
}

/// Borrows the shared storage behind an opaque handle.
///
/// # Safety
///
/// `handle` must be null or a live handle of type `H` created by this crate.
pub unsafe fn handle_ref<'a, H: HandleKind>(handle: *const H) -> Option<&'a Shared<H>> {
    unsafe { handle.cast::<Shared<H>>().as_ref() }
}

/// Consumes an opaque handle and returns its shared storage.
///
/// # Safety
///
/// `handle` must be an owned handle of type `H` created by this crate. It must
/// not be accessed or freed after this call.
pub unsafe fn handle_into_shared<H: HandleKind>(handle: *mut H) -> Option<Shared<H>> {
    if handle.is_null() {
        return None;
    }

    Some(*unsafe { Box::from_raw(handle.cast::<Shared<H>>()) })
}

/// Releases an owned opaque handle.
///
/// Passing null is a no-op.
///
/// # Safety
///
/// `handle` must be null or an owned handle of type `H` created by this crate.
/// It must not be freed twice.
pub unsafe fn drop_handle<H: HandleKind>(handle: *mut H) {
    if !handle.is_null() {
        drop(unsafe { Box::from_raw(handle.cast::<Shared<H>>()) })
    }
}
