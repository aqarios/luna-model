// use std::ffi::c_char;
// 
// use luna_model::core::Solution;
// 
// use crate::{
//     LmSol, LmStatus, LmVtype,
//     error::guard,
//     utils::{mut_from_c, opt_from_c, slice_from_c, str_from_c, vtype_from_c},
// };
// 
// /// Adds an empty binary column for a variable name.
// ///
// /// # Safety
// /// `sol` must be a valid mutable solution handle created by this library.
// /// `name` must point to a valid NUL-terminated C string.
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn lm_sol_n_samples(sol: *const LmSol, out: *const usize) -> LmStatus {
//     guard(|| {
//         let sol = unsafe { mut_from_c(sol as *mut Solution, "sol") }?;
//         let name = unsafe { str_from_c(name, "name") }?;
//         sol.add_empty_binary(name.to_owned());
//         Ok(())
//     })
// }
// 
// 
