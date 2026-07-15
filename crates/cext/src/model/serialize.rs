use crate::utils::{new_handle_out_from_c, out_from_c, shared_from_c, slice_from_c};
use luna_model::core::Model;
use luna_model::serializer::prelude::*;

use crate::buffer::buffer_from_vec;
use crate::error::{CapiError, guard};
use crate::{LmBuffer, LmModel, LmStatus};

/// Serializes a model to bytes, writing an owned buffer to `out`.
///
/// The buffer must be released with `lm_buffer_free`.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_serialize(model: *const LmModel, out: *mut LmBuffer) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        let bytes = model
            .encode(Some(true), None)
            .map_err(CapiError::serialization)?;
        *out = buffer_from_vec(bytes);
        Ok(())
    })
}

/// Deserializes a model from `len` bytes at `data`, writing a new handle to `out`.
///
/// The new handle must be released with [`lm_model_free`].
///
/// # Safety
/// `data` must point to `len` readable bytes and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_deserialize(
    data: *const u8,
    len: usize,
    out: *mut *mut LmModel,
) -> LmStatus {
    guard(|| {
        let bytes = unsafe { slice_from_c(data, len, "data") }?;
        let model: Model = bytes
            .unversionize()
            .decompress()
            .map_err(CapiError::deserialization)?
            .decode(())
            .map_err(CapiError::deserialization)?;
        unsafe { new_handle_out_from_c(out, model) }
    })
}
