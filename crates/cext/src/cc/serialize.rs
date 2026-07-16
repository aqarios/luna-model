use luna_model::core::ConstraintCollection;
use luna_model::serializer::prelude::*;

use super::utils::{collection_from_c, collection_out_from_c};
use crate::{
    LmBuffer, LmCC, LmEnv, LmStatus,
    buffer::buffer_from_vec,
    error::{CapiError, guard},
    utils::{out_from_c, slice_from_c, storage_from_c},
};

/// Serializes a constraint-collection to bytes, writing an owned buffer to `out`.
///
/// The buffer must be released with `lm_buffer_free`.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle created by this library.
/// `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_serialize(cc: *const LmCC, out: *mut LmBuffer) -> LmStatus {
    guard(|| {
        let cc = unsafe { collection_from_c(cc, "cc") }?.read();
        let out = unsafe { out_from_c(out) }?;
        let bytes = cc
            .encode(Some(true), None)
            .map_err(CapiError::serialization)?;
        *out = buffer_from_vec(bytes);
        Ok(())
    })
}

/// Deserializes a constraint-collection from `len` bytes at `data`, writing a new handle to `out`.
///
/// The new handle must be released with [`lm_cc_free`].
///
/// # Safety
/// `data` must point to `len` readable bytes.
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_deserialize(
    data: *const u8,
    len: usize,
    env: *const LmEnv,
    out: *mut *mut LmCC,
) -> LmStatus {
    guard(|| {
        let bytes = unsafe { slice_from_c(data, len, "data") }?;
        let env = unsafe { storage_from_c(env, "env")? };
        let model: ConstraintCollection = bytes
            .unversionize()
            .decompress()
            .map_err(CapiError::deserialization)?
            .decode(env.clone())
            .map_err(CapiError::deserialization)?;
        unsafe { collection_out_from_c(out, model) }
    })
}
