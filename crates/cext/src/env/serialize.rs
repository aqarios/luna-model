use luna_model::core::{ArcEnv, Environment};
use luna_model::serializer::prelude::*;

use crate::buffer::buffer_from_vec;
use crate::error::{CapiError, guard};
use crate::{LmBuffer, LmEnv, LmStatus};

/// Serializes an environment to bytes, writing an owned buffer to `out`.
///
/// The buffer must be released with [`crate::buffer::lm_buffer_free`].
///
/// # Safety
/// `env` must be a valid handle and `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_serialize(env: *const LmEnv, out: *mut LmBuffer) -> LmStatus {
    guard(|| {
        let env = unsafe { (env as *const ArcEnv).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("env"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let bytes = env
            .read_arc()
            .encode(Some(true), None)
            .map_err(|e| CapiError::new(LmStatus::Serialization, e.to_string()))?;
        *out = buffer_from_vec(bytes);
        Ok(())
    })
}

/// Deserializes a env from `len` bytes at `data`, writing a new handle to `out`.
///
/// The new handle must be released with [`lm_env_free`].
///
/// # Safety
/// `data` must point to `len` readable bytes
/// `out` must point to writable storage
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_deserialize(
    data: *const u8,
    len: usize,
    out: *mut *mut LmEnv,
) -> LmStatus {
    guard(|| {
        if data.is_null() {
            return Err(CapiError::null_pointer("data"));
        }
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let bytes = unsafe { std::slice::from_raw_parts(data, len) };
        let env: Environment = bytes
            .unversionize()
            .decompress()
            .map_err(|e| CapiError::new(LmStatus::Deserialization, e.to_string()))?
            .decode(())
            .map_err(|e| CapiError::new(LmStatus::Deserialization, e.to_string()))?;
        let env = ArcEnv::from(env);
        *out = Box::into_raw(Box::new(env)) as *mut LmEnv;
        Ok(())
    })
}
