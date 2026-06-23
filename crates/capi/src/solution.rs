//! `LmSolution` opaque handle and solution deserialization.
//!
//! `inner` is only written for now (read once getters are added); the allow keeps
//! incremental builds clean and is removed during the cleanup step.
#![allow(dead_code)]

use luna_model::core::Solution;
use luna_model::serializer::prelude::*;

use crate::error::{CapiError, LmStatus, guard};

/// Opaque handle to a LunaModel solution.
///
/// Solutions are produced elsewhere (a solver or the Python API) and loaded here
/// from serialized bytes with [`lm_solution_deserialize`]. Release with
/// [`lm_solution_free`].
pub struct LmSolution {
    pub(crate) inner: Solution,
}

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
        *out = Box::into_raw(Box::new(LmSolution { inner: solution }));
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
        drop(unsafe { Box::from_raw(solution) });
    }
}
