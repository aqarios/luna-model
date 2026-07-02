use luna_model::core::ArcEnv;

use crate::{LmEnv, error::CapiError, utils::ref_from_c};

/// Borrows the shared environment behind a C handle.
///
/// # Safety
/// `env` must be null or a valid environment handle created by this library.
pub(crate) unsafe fn environment_from_c<'a>(env: *const LmEnv) -> Result<&'a ArcEnv, CapiError> {
    unsafe { ref_from_c(env as *const ArcEnv, "env") }
}
