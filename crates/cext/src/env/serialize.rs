use crate::utils::{handle_to_c, out_from_c, ref_from_c, slice_from_c};
use luna_model::core::{ArcEnv, Environment};
use luna_model::serializer::prelude::*;

use crate::buffer::buffer_from_vec;
use crate::error::{CapiError, guard};
use crate::{LmBuffer, LmEnv, LmStatus};

/// Serializes an environment to bytes, writing an owned buffer to `out`.
///
/// The buffer must be released with `lm_buffer_free`.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_serialize(env: *const LmEnv, out: *mut LmBuffer) -> LmStatus {
    guard(|| {
        let env = unsafe { ref_from_c(env as *const ArcEnv, "env") }?;
        let out = unsafe { out_from_c(out) }?;
        let bytes = env
            .read_arc()
            .encode(Some(true), None)
            .map_err(CapiError::serialization)?;
        *out = buffer_from_vec(bytes);
        Ok(())
    })
}

/// Deserializes an environment from `len` bytes at `data`, writing a new handle to `out`.
///
/// The new handle must be released with [`lm_env_free`].
///
/// # Safety
/// `data` must point to `len` readable bytes.
/// `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_deserialize(
    data: *const u8,
    len: usize,
    out: *mut *mut LmEnv,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        let bytes = unsafe { slice_from_c(data, len, "data") }?;
        let env: Environment = bytes
            .unversionize()
            .decompress()
            .map_err(CapiError::deserialization)?
            .decode(())
            .map_err(CapiError::deserialization)?;
        let env = ArcEnv::from(env);
        *out = handle_to_c(env);
        Ok(())
    })
}
