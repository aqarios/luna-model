use crate::utils::{handle_to_c, out_from_c, ref_from_c, shared_from_c, slice_from_c};
use luna_model::core::Expression;
use luna_model::serializer::prelude::*;

use crate::buffer::buffer_from_vec;
use crate::error::{CapiError, guard};
use crate::{LmBuffer, LmEnv, LmExpr, LmStatus};

/// Serializes an expression to bytes, writing an owned buffer to `out`.
///
/// The buffer must be released with `lm_buffer_free`.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_serialize(expr: *const LmExpr, out: *mut LmBuffer) -> LmStatus {
    guard(|| {
        let expr = unsafe { ref_from_c(expr as *const Expression, "expr") }?;
        let out = unsafe { out_from_c(out) }?;
        let bytes = expr
            .encode(Some(true), None)
            .map_err(CapiError::serialization)?;
        *out = buffer_from_vec(bytes);
        Ok(())
    })
}

/// Deserializes an expression from `len` bytes at `data`, writing a new handle to `out`.
///
/// The new handle must be released with [`lm_expr_free`].
///
/// # Safety
/// `data` must point to `len` readable bytes.
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_deserialize(
    data: *const u8,
    len: usize,
    env: *const LmEnv,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        let bytes = unsafe { slice_from_c(data, len, "data") }?;
        let env = unsafe { shared_from_c(env, "env")? }.read_arc();
        let expr: Expression = bytes
            .unversionize()
            .decompress()
            .map_err(CapiError::deserialization)?
            .decode(env.clone())
            .map_err(CapiError::deserialization)?;
        *out = handle_to_c(expr);
        Ok(())
    })
}
