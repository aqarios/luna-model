//! `LmModel` handle operations and the model serialization bridge.

use std::ffi::{CStr, c_char};

use luna_model::core::Model;
use luna_model::serializer::prelude::*;

use crate::buffer::buffer_from_vec;
use crate::error::{CapiError, guard};
use crate::utils::sense_from_c;
use crate::{LmBuffer, LmModel, LmSense, LmStatus};

/// Creates a new, empty (unnamed, minimization) model.
///
/// Returns a non-null handle that must be released with [`lm_model_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_model_default() -> *mut LmModel {
    Box::into_raw(Box::new(Model::default())) as *mut LmModel
}

/// Creates a new model.
///
/// `name == NULL` means use the default model name.
/// `out` receives an owned model handle that must be freed with `lm_model_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_new(
    name: *const c_char,
    sense: LmSense,
    out: *mut *mut LmModel,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let name = if name.is_null() {
            None
        } else {
            let cstr = unsafe { CStr::from_ptr(name) };
            Some(
                cstr.to_str()
                    .map_err(|e| {
                        CapiError::new(
                            LmStatus::InvalidArgument,
                            format!("`name` must be valid UTF-8: {e}"),
                        )
                    })?
                    .to_owned(),
            )
        };

        let model = Model::new(name, Some(sense_from_c(sense)));
        *out = Box::into_raw(Box::new(model)) as *mut LmModel;

        Ok(())
    })
}

/// Returns a copy of the model name as an owned UTF-8 byte buffer.
///
/// The returned buffer is written to `out` and must be released with
/// `lm_buffer_free`.
///
/// The buffer is not guaranteed to be NUL-terminated. C callers should use
/// `buffer.data` together with `buffer.len`.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_name(model: *const LmModel, out: *mut LmBuffer) -> LmStatus {
    guard(|| {
        let model = unsafe { (model as *const Model).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        *out = buffer_from_vec(model.name.as_bytes().to_vec());

        Ok(())
    })
}

/// Clones a model into a new, independent handle written to `out`.
///
/// # Safety
/// `model` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_clone(model: *const LmModel, out: *mut *mut LmModel) -> LmStatus {
    guard(|| {
        let model = unsafe { (model as *const Model).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(model.clone())) as *mut LmModel;
        Ok(())
    })
}

/// Frees a model handle. Passing null is a no-op.
///
/// # Safety
/// `model` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_free(model: *mut LmModel) {
    if !model.is_null() {
        drop(unsafe { Box::from_raw(model as *mut Model) });
    }
}

/// Serializes a model to bytes, writing an owned buffer to `out`.
///
/// The buffer must be released with [`crate::buffer::lm_buffer_free`].
///
/// # Safety
/// `model` must be a valid handle and `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_serialize(model: *const LmModel, out: *mut LmBuffer) -> LmStatus {
    guard(|| {
        let model = unsafe { (model as *const Model).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let bytes = model
            .encode(Some(true), None)
            .map_err(|e| CapiError::new(LmStatus::Serialization, e.to_string()))?;
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
        if data.is_null() {
            return Err(CapiError::null_pointer("data"));
        }
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let bytes = unsafe { std::slice::from_raw_parts(data, len) };
        let model: Model = bytes
            .unversionize()
            .decompress()
            .map_err(|e| CapiError::new(LmStatus::Deserialization, e.to_string()))?
            .decode(())
            .map_err(|e| CapiError::new(LmStatus::Deserialization, e.to_string()))?;
        *out = Box::into_raw(Box::new(model)) as *mut LmModel;
        Ok(())
    })
}
