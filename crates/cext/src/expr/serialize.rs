use luna_model::core::Expression;
use luna_model::serializer::prelude::*;

use crate::buffer::buffer_from_vec;
use crate::env::utils::environment_from_c;
use crate::error::{CapiError, guard};
use crate::{LmBuffer, LmEnv, LmExpr, LmStatus};

/// Serializes an expression to bytes, writing an owned buffer to `out`.
///
/// The buffer must be released with [`crate::buffer::lm_buffer_free`].
///
/// # Safety
/// `expr` must be a valid handle and `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_serialize(expr: *const LmExpr, out: *mut LmBuffer) -> LmStatus {
    guard(|| {
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let bytes = expr
            .encode(Some(true), None)
            .map_err(|e| CapiError::new(LmStatus::Serialization, e.to_string()))?;
        *out = buffer_from_vec(bytes);
        Ok(())
    })
}

/// Deserializes a expr from `len` bytes at `data`, writing a new handle to `out`.
///
/// The new handle must be released with [`lm_expr_free`].
///
/// # Safety
/// `data` must point to `len` readable bytes
/// `env` must point to a valid environment handle
/// `out` must point to writable storage
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_deserialize(
    data: *const u8,
    len: usize,
    env: *const LmEnv,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        if data.is_null() {
            return Err(CapiError::null_pointer("data"));
        }
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let bytes = unsafe { std::slice::from_raw_parts(data, len) };
        let env = unsafe { environment_from_c(env)? };
        let expr: Expression = bytes
            .unversionize()
            .decompress()
            .map_err(|e| CapiError::new(LmStatus::Deserialization, e.to_string()))?
            .decode(env.clone())
            .map_err(|e| CapiError::new(LmStatus::Deserialization, e.to_string()))?;
        *out = Box::into_raw(Box::new(expr)) as *mut LmExpr;
        Ok(())
    })
}
