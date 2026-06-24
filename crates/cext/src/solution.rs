//! `LmSolution` handle operations and solution deserialization.

use luna_model::core::Solution;
use luna_model::serializer::prelude::*;

use crate::error::{CapiError, guard};
use crate::{LmSolution, LmStatus};

/// Deserializes a solution from `len` bytes at `data`, writing a new handle to `out`.
///
/// The new handle must be released with [`lm_solution_free`].
///
/// # Safety
/// `data` must point to `len` readable bytes and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_solution_deserialize(
    data: *const u8,
    len: usize,
    out: *mut *mut LmSolution,
) -> LmStatus {
    guard(|| {
        if data.is_null() {
            return Err(CapiError::null_pointer("data"));
        }
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let bytes = unsafe { std::slice::from_raw_parts(data, len) };
        let solution: Solution = bytes
            .unversionize()
            .decompress()
            .map_err(|e| CapiError::new(LmStatus::Deserialization, e.to_string()))?
            .decode(())
            .map_err(|e| CapiError::new(LmStatus::Deserialization, e.to_string()))?;
        *out = Box::into_raw(Box::new(solution)) as *mut LmSolution;
        Ok(())
    })
}

/// Frees a solution handle. Passing null is a no-op.
///
/// # Safety
/// `solution` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_solution_free(solution: *mut LmSolution) {
    if !solution.is_null() {
        drop(unsafe { Box::from_raw(solution as *mut Solution) });
    }
}
