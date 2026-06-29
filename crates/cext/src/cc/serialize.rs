use luna_model::core::ConstraintCollection;
use luna_model::serializer::prelude::*;

use crate::buffer::buffer_from_vec;
use crate::env::utils::environment_from_c;
use crate::error::{CapiError, guard};
use crate::{LmBuffer, LmCC, LmEnv, LmStatus};

/// Serializes a constraint-collection to bytes, writing an owned buffer to `out`.
///
/// The buffer must be released with [`crate::buffer::lm_buffer_free`].
///
/// # Safety
/// `cc` must be a valid handle and `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_serialize(cc: *const LmCC, out: *mut LmBuffer) -> LmStatus {
    guard(|| {
        let cc = unsafe { (cc as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let bytes = cc
            .encode(Some(true), None)
            .map_err(|e| CapiError::new(LmStatus::Serialization, e.to_string()))?;
        *out = buffer_from_vec(bytes);
        Ok(())
    })
}

/// Deserializes a constraint-collection from `len` bytes at `data`, writing a new handle to `out`.
///
/// The new handle must be released with [`lm_cc_free`].
///
/// # Safety
/// `data` must point to `len` readable bytes
/// `env` must point to a valid environment handle
/// `out` must point to writable storage
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_deserialize(
    data: *const u8,
    len: usize,
    env: *const LmEnv,
    out: *mut *mut LmCC,
) -> LmStatus {
    guard(|| {
        if data.is_null() {
            return Err(CapiError::null_pointer("data"));
        }
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let bytes = unsafe { std::slice::from_raw_parts(data, len) };
        let env = unsafe { environment_from_c(env)? };
        let model: ConstraintCollection = bytes
            .unversionize()
            .decompress()
            .map_err(|e| CapiError::new(LmStatus::Deserialization, e.to_string()))?
            .decode(env.clone())
            .map_err(|e| CapiError::new(LmStatus::Deserialization, e.to_string()))?;
        *out = Box::into_raw(Box::new(model)) as *mut LmCC;
        Ok(())
    })
}
