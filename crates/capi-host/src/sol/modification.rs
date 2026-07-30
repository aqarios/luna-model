use std::ffi::c_char;

use lunamodel_core::Timing;

use crate::{
    LmAssignment, LmColumn, LmColumnBuffer, LmSense, LmSol, LmStatus, LmTiming, LmVtype,
    buffer::column_buffer_from_vec,
    error::{CapiError, guard},
    utils::{
        assignment_from_c, handle_to_c, new_handle_out_from_c, opt_from_c, opt_slice_from_c,
        out_from_c, ref_from_c, sense_from_c, shared_from_c, slice_from_c, str_from_c,
        vtype_from_c,
    },
};

/// Adds an empty binary column for a variable name.
///
/// # Safety
/// `sol` must be a valid mutable solution handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_add_empty_binary(sol: *mut LmSol, name: *const c_char) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        let name = unsafe { str_from_c(name, "name") }?;
        sol.add_empty_binary(name.to_owned());
        Ok(())
    })
}

/// Adds an empty spin column for a variable name.
///
/// # Safety
/// `sol` must be a valid mutable solution handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_add_empty_spin(sol: *mut LmSol, name: *const c_char) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        let name = unsafe { str_from_c(name, "name") }?;
        sol.add_empty_spin(name.to_owned());
        Ok(())
    })
}

/// Adds an empty integer column for a variable name.
///
/// # Safety
/// `sol` must be a valid mutable solution handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_add_empty_integer(
    sol: *mut LmSol,
    name: *const c_char,
) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        let name = unsafe { str_from_c(name, "name") }?;
        sol.add_empty_integer(name.to_owned());
        Ok(())
    })
}

/// Adds an empty real column for a variable name.
///
/// # Safety
/// `sol` must be a valid mutable solution handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_add_empty_real(sol: *mut LmSol, name: *const c_char) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        let name = unsafe { str_from_c(name, "name") }?;
        sol.add_empty_real(name.to_owned());
        Ok(())
    })
}

/// Adds a binary column for a variable name.
///
/// Values are cast to binary using `tol` as the near-integral tolerance.
///
/// # Safety
/// `sol` must be a valid mutable solution handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `data` must be null or point to `len` readable `double` values.
/// `tol` may be null; if non-null, it must point to a readable `double`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_add_binary(
    sol: *mut LmSol,
    name: *const c_char,
    data: *const f64,
    len: usize,
    tol: *const f64,
) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        let name = unsafe { str_from_c(name, "name") }?;
        let data = unsafe { slice_from_c(data, len, "data") }?.to_vec();
        let tol = unsafe { opt_from_c(tol) };
        sol.add_binary(name.to_owned(), data, tol)
            .map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Adds a spin column for a variable name.
///
/// Values are cast to binary using `tol` as the near-integral tolerance.
///
/// # Safety
/// `sol` must be a valid mutable solution handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `data` must be null or point to `len` readable `double` values.
/// `tol` may be null; if non-null, it must point to a readable `double`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_add_spin(
    sol: *mut LmSol,
    name: *const c_char,
    data: *const f64,
    len: usize,
    tol: *const f64,
) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        let name = unsafe { str_from_c(name, "name") }?;
        let data = unsafe { slice_from_c(data, len, "data") }?.to_vec();
        let tol = unsafe { opt_from_c(tol) };
        sol.add_spin(name.to_owned(), data, tol)
            .map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Adds an integer column for a variable name.
///
/// Values are cast to binary using `tol` as the near-integral tolerance.
///
/// # Safety
/// `sol` must be a valid mutable solution handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `data` must be null or point to `len` readable `double` values.
/// `tol` may be null; if non-null, it must point to a readable `double`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_add_integer(
    sol: *mut LmSol,
    name: *const c_char,
    data: *const f64,
    len: usize,
    tol: *const f64,
) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        let name = unsafe { str_from_c(name, "name") }?;
        let data = unsafe { slice_from_c(data, len, "data") }?.to_vec();
        let tol = unsafe { opt_from_c(tol) };
        sol.add_integer(name.to_owned(), data, tol)
            .map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Adds a real column for a variable name.
///
/// # Safety
/// `sol` must be a valid mutable solution handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `data` must be null or point to `len` readable `double` values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_add_real(
    sol: *mut LmSol,
    name: *const c_char,
    data: *const f64,
    len: usize,
) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        let name = unsafe { str_from_c(name, "name") }?;
        let data = unsafe { slice_from_c(data, len, "data") }?.to_vec();
        sol.add_real(name.to_owned(), data);
        Ok(())
    })
}

/// Dispatches to the appropriate typed column insertion helper.
///
/// # Safety
/// `sol` must be a valid mutable solution handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `data` must be null or point to `len` readable `double` values.
/// `tol` may be null; if non-null, it must point to a readable `double`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_add_col(
    sol: *mut LmSol,
    vtype: LmVtype,
    name: *const c_char,
    data: *const f64,
    len: usize,
    tol: *const f64,
) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        let name = unsafe { str_from_c(name, "name") }?;
        let data = unsafe { slice_from_c(data, len, "data") }?.to_vec();
        let tol = unsafe { opt_from_c(tol) };
        sol.add_col(vtype_from_c(vtype), name.to_owned(), data, tol)
            .map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Removes a column for a variable name.
///
/// # Safety
/// `sol` must be a valid mutable solution handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_remove_col(
    sol: *mut LmSol,
    name: *const c_char,
    out: *mut *mut LmColumn,
) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        let name = unsafe { str_from_c(name, "name") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = sol
            .remove_col(name)
            .map(handle_to_c)
            .unwrap_or(std::ptr::null_mut());
        Ok(())
    })
}

/// Removes columns for many variable names.
///
/// # Safety
/// `sol` must be a valid mutable solution handle created by this library.
/// `names` must point to `len` readable `char *` values.
/// Each entry in `names` must be non-null and point to a valid NUL-terminated UTF-8 C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_remove_cols(
    sol: *mut LmSol,
    names: *const *const c_char,
    len: usize,
    out: *mut LmColumnBuffer,
) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        let names = unsafe { slice_from_c(names, len, "names") }?;
        let names: Vec<String> = names
            .iter()
            .map(|&name| unsafe { str_from_c(name, "names") }.map(str::to_owned))
            .collect::<Result<_, _>>()?;
        let removed = sol
            .remove_cols(&names)
            .into_iter()
            .map(|column| column.map(handle_to_c).unwrap_or(std::ptr::null_mut()))
            .collect();
        *unsafe { out_from_c(out) }? = column_buffer_from_vec(removed);
        Ok(())
    })
}

/// Combines duplicate sample rows into a single row and sums their counts.
///
/// Duplicate detection is currently based on the stringified sample values in
/// column order. That keeps the implementation simple, but it also means the
/// exact behavior for floating-point heavy solutions depends on how those
/// values stringify.
///
/// # Safety
/// `sol` must be a valid mutable solution handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_aggregate(sol: *mut LmSol) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        sol.aggregate().map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Filters solution rows by a boolean keep-mask.
///
/// # Safety
/// `sol` must be valid, `mask` must contain `len` values, and `out` writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_filter_by_mask(
    sol: *const LmSol,
    mask: *const bool,
    len: usize,
    out: *mut *mut LmSol,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let mask = unsafe { slice_from_c(mask, len, "mask") }?;
        let filtered = sol.filter_by_mask(mask).map_err(CapiError::errored)?;
        unsafe { new_handle_out_from_c(out, filtered) }
    })
}

/// Appends a named assignment row.
///
/// # Safety
/// `sol` must be valid; `names` and `assignments` must each contain `len`
/// valid elements; optional `energy` must be readable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_push(
    sol: *mut LmSol,
    names: *const *const c_char,
    assignments: *const LmAssignment,
    len: usize,
    counts: usize,
    energy: *const f64,
) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        let names = unsafe { slice_from_c(names, len, "names") }?;
        let assignments = unsafe { slice_from_c(assignments, len, "assignments") }?;
        let sample = names
            .iter()
            .zip(assignments)
            .map(|(&name, assignment)| {
                let name = unsafe { str_from_c(name, "name") }?.to_owned();
                let assignment = assignment_from_c(*assignment)?;
                Ok((name, assignment))
            })
            .collect::<Result<_, CapiError>>()?;
        sol.push(sample, counts, unsafe { opt_from_c(energy) })
            .map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Replaces the per-row sample counts.
///
/// # Safety
/// `sol` must be a valid mutable solution handle. `counts` must be null when
/// `len` is zero or point to `len` readable values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_set_counts(
    sol: *mut LmSol,
    counts: *const usize,
    len: usize,
) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        sol.counts = unsafe { slice_from_c(counts, len, "counts") }?.to_vec();
        Ok(())
    })
}

/// Replaces the optional solution timing metadata.
///
/// A null `timing` clears the timing value.
///
/// # Safety
/// `sol` must be a valid mutable solution handle. When non-null, `timing` must
/// be a valid timing handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_set_timing(sol: *mut LmSol, timing: *const LmTiming) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        sol.timing = if timing.is_null() {
            None
        } else {
            Some(*unsafe { ref_from_c(timing as *const Timing, "timing") }?)
        };
        Ok(())
    })
}

/// Sets the solution optimization sense.
///
/// # Safety
/// `sol` must be a valid mutable solution handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_set_sense(sol: *mut LmSol, sense: LmSense) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        sol.sense = sense_from_c(sense);
        Ok(())
    })
}

/// Replaces the optional per-sample raw energies. A null `data` clears the value.
///
/// # Safety
/// `sol` must be a valid mutable solution handle. `data` must be null or point
/// to `len` readable values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_set_raw_energies(
    sol: *mut LmSol,
    data: *const f64,
    len: usize,
) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        sol.raw_energies = unsafe { opt_slice_from_c(data, len) }.map(|s| s.to_vec());
        Ok(())
    })
}

/// Replaces the optional per-sample objective values. A null `data` clears the value.
///
/// # Safety
/// `sol` must be a valid mutable solution handle. `data` must be null or point
/// to `len` readable values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_set_obj_values(
    sol: *mut LmSol,
    data: *const f64,
    len: usize,
) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        sol.obj_values = unsafe { opt_slice_from_c(data, len) }.map(|s| s.to_vec());
        Ok(())
    })
}

/// Replaces the optional per-sample feasibility flags. A null `data` clears the value.
///
/// # Safety
/// `sol` must be a valid mutable solution handle. `data` must be null or point
/// to `len` readable values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_set_feasible(
    sol: *mut LmSol,
    data: *const bool,
    len: usize,
) -> LmStatus {
    guard(|| {
        let mut sol = unsafe { shared_from_c(sol, "sol") }?.write_arc();
        sol.feasible = unsafe { opt_slice_from_c(data, len) }.map(|s| s.to_vec());
        Ok(())
    })
}
