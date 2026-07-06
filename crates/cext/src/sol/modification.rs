use std::ffi::c_char;

use luna_model::core::Solution;

use crate::{
    LmSol, LmStatus, LmVtype,
    error::{CapiError, guard},
    utils::{mut_from_c, opt_from_c, slice_from_c, str_from_c, vtype_from_c},
};

/// Adds an empty binary column for a variable name.
///
/// # Safety
/// `sol` must be a valid mutable solution handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_add_empty_binary(sol: *mut LmSol, name: *const c_char) -> LmStatus {
    guard(|| {
        let sol = unsafe { mut_from_c(sol as *mut Solution, "sol") }?;
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
        let sol = unsafe { mut_from_c(sol as *mut Solution, "sol") }?;
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
        let sol = unsafe { mut_from_c(sol as *mut Solution, "sol") }?;
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
        let sol = unsafe { mut_from_c(sol as *mut Solution, "sol") }?;
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
        let sol = unsafe { mut_from_c(sol as *mut Solution, "sol") }?;
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
        let sol = unsafe { mut_from_c(sol as *mut Solution, "sol") }?;
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
        let sol = unsafe { mut_from_c(sol as *mut Solution, "sol") }?;
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
        let sol = unsafe { mut_from_c(sol as *mut Solution, "sol") }?;
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
        let sol = unsafe { mut_from_c(sol as *mut Solution, "sol") }?;
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
pub unsafe extern "C" fn lm_sol_remove_col(sol: *mut LmSol, name: *const c_char) -> LmStatus {
    guard(|| {
        let sol = unsafe { mut_from_c(sol as *mut Solution, "sol") }?;
        let name = unsafe { str_from_c(name, "name") }?;
        sol.remove_col(name);
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
) -> LmStatus {
    guard(|| {
        let sol = unsafe { mut_from_c(sol as *mut Solution, "sol") }?;
        let names = unsafe { slice_from_c(names, len, "names") }?;
        let names: Vec<String> = names
            .iter()
            .map(|&name| unsafe { str_from_c(name, "names") }.map(str::to_owned))
            .collect::<Result<_, _>>()?;
        sol.remove_cols(&names);
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
        let sol = unsafe { mut_from_c(sol as *mut Solution, "sol") }?;
        sol.aggregate().map_err(CapiError::errored)?;
        Ok(())
    })
}
