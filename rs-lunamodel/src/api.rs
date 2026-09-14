//! Access to the LunaModel C API dispatch table.
//!
//! Like the `example_rsext` extension, this crate does not link the LunaModel
//! library directly. Instead it holds a pointer to the [`LunaModelCApi`] table
//! whose function pointers dispatch into the host's single in-process instance.
//! The host provides that table once (typically read from the
//! `luna_model._lm._C_API` PyCapsule) via [`init`].

use std::ffi::CStr;
use std::fmt;
use std::sync::OnceLock;

use lunamodel_capi::{CAPI_ABI_VERSION, LunaModelCApi};

/// Wraps the raw table pointer so it can live in a `static`.
///
/// The table is immutable and lives for the host process's lifetime, so sharing
/// the pointer across threads is sound.
struct ApiHandle(*const LunaModelCApi);
unsafe impl Send for ApiHandle {}
unsafe impl Sync for ApiHandle {}

static API: OnceLock<ApiHandle> = OnceLock::new();

/// Failure while installing the C API table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitError {
    /// The provided table pointer was null.
    NullTable,
    /// The table was already installed.
    AlreadyInitialized,
    /// The table's ABI version does not match the one this crate was built with.
    AbiMismatch {
        /// ABI version this crate expects.
        expected: u32,
        /// ABI version reported by the provided table.
        found: u32,
    },
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InitError::NullTable => write!(f, "C API table pointer is null"),
            InitError::AlreadyInitialized => write!(f, "C API table already initialized"),
            InitError::AbiMismatch { expected, found } => write!(
                f,
                "C API ABI mismatch: built against {expected}, runtime {found}"
            ),
        }
    }
}

impl std::error::Error for InitError {}

/// Installs the C API dispatch table for this crate to use.
///
/// Call once during startup with the table obtained from the host — for a Python
/// extension, the pointer stored in the `luna_model._lm._C_API` capsule.
///
/// # Safety
/// `table` must point to a valid [`LunaModelCApi`] that stays alive for the rest
/// of the process (the capsule's backing allocation satisfies this).
pub unsafe fn init(table: *const LunaModelCApi) -> Result<(), InitError> {
    if table.is_null() {
        return Err(InitError::NullTable);
    }
    let found = unsafe { (*table).abi_version };
    if found != CAPI_ABI_VERSION {
        return Err(InitError::AbiMismatch {
            expected: CAPI_ABI_VERSION,
            found,
        });
    }
    API.set(ApiHandle(table))
        .map_err(|_| InitError::AlreadyInitialized)
}

/// Returns whether the table has been installed.
pub fn is_initialized() -> bool {
    API.get().is_some()
}

/// Returns the LunaModel host version backing the installed dispatch table.
pub fn version() -> crate::Result<String> {
    let ptr = (api().version)();
    if ptr.is_null() {
        return Err(crate::Error {
            status: crate::Status::NullPointer,
            message: "the LunaModel host returned a null version string".to_owned(),
            kind_tag: None,
        });
    }
    Ok(unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned())
}

/// Returns the ABI version of the installed table.
pub fn abi_version() -> u32 {
    api().abi_version
}

/// Borrows the installed C API table.
///
/// # Panics
/// Panics if [`init`] has not been called.
pub(crate) fn api() -> &'static LunaModelCApi {
    let handle = API
        .get()
        .expect("rs-lunamodel: C API table not installed; call `rs_lunamodel::init` first");
    // SAFETY: `init` validated the pointer and the host keeps it alive for the
    // process lifetime.
    unsafe { &*handle.0 }
}
