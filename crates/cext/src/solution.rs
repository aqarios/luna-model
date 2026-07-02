//! `LmSolution` handle operations and solution deserialization.

use crate::utils::{drop_from_c, handle_to_c, out_from_c, slice_from_c};
use luna_model::core::Solution;
use luna_model::serializer::prelude::*;

use crate::error::{CapiError, guard};
use crate::{LmSol, LmStatus};

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

/// Frees a solution handle. Passing null is a no-op.
///
/// # Safety
/// `solution` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_free(solution: *mut LmSol) {
    unsafe { drop_from_c(solution as *mut Solution) }
}
