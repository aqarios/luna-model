use crate::{
    buffer::buffer_from_vec,
    utils::{handle_to_c, out_from_c, ref_from_c, slice_from_c},
};
use luna_model::core::Solution;
use luna_model::serializer::prelude::*;

use crate::{
    LmBuffer, LmSol, LmStatus,
    error::{CapiError, guard},
};

/// Serializes a solution to bytes, writing an owned buffer to `out`.
///
/// The buffer must be released with `lm_buffer_free`.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_serialize(sol: *const LmSol, out: *mut LmBuffer) -> LmStatus {
    guard(|| {
        let sol = unsafe { ref_from_c(sol as *const Solution, "sol") }?;
        let out = unsafe { out_from_c(out) }?;
        let bytes = sol
            .encode(Some(true), None)
            .map_err(CapiError::serialization)?;
        *out = buffer_from_vec(bytes);
        Ok(())
    })
}

/// Deserializes a solution from `len` bytes at `data`, writing a new handle to `out`.
///
/// The new handle must be released with [`lm_sol_free`].
///
/// # Safety
/// `data` must point to `len` readable bytes and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_deserialize(
    data: *const u8,
    len: usize,
    out: *mut *mut LmSol,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        let bytes = unsafe { slice_from_c(data, len, "data") }?;
        let solution: Solution = bytes
            .unversionize()
            .decompress()
            .map_err(CapiError::deserialization)?
            .decode(())
            .map_err(CapiError::deserialization)?;
        *out = handle_to_c(solution);
        Ok(())
    })
}
