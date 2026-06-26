use luna_model::core::ArcEnv;

use crate::{LmEnvironment, error::CapiError};

pub(crate) unsafe fn environment_from_c<'a>(
    env: *const LmEnvironment,
) -> Result<&'a ArcEnv, CapiError> {
    unsafe { (env as *const ArcEnv).as_ref() }.ok_or_else(|| CapiError::null_pointer("env"))
}
