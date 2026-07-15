use itertools::Either;
use luna_model::core::{ArcEnv, Model, Solution};
use lunamodel_cext_handle::{drop_handle, new_handle};

use crate::{
    LmEnv, LmModel, LmSense, LmSol, LmStatus,
    error::{CapiError, guard},
    utils::{new_handle_out_from_c, opt_from_c, sense_from_c, shared_from_c},
};

/// Frees a solution handle. Passing null is a no-op.
///
/// # Safety
/// `solution` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_free(solution: *mut LmSol) {
    unsafe { drop_handle(solution) }
}

/// Creates a new, empty solution.
///
/// Returns a non-null handle that must be released with [`lm_sol_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_sol_default() -> *mut LmSol {
    new_handle(Solution::default())
}

/// Creates a new solution for a sense.
#[unsafe(no_mangle)]
pub extern "C" fn lm_sol_with_sense(sense: LmSense) -> *mut LmSol {
    new_handle(Solution::with_sense(sense_from_c(sense)))
}

/// Creates a new solution with capacity.
#[unsafe(no_mangle)]
pub extern "C" fn lm_sol_with_capacity(capacity: usize) -> *mut LmSol {
    new_handle(Solution::with_capacity(capacity))
}

/// Creates a new solution from an environment.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `LmSol *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_from_env(env: *const LmEnv, out: *mut *mut LmSol) -> LmStatus {
    guard(|| {
        let env = unsafe { shared_from_c(env, "env") }?.read_arc();
        let sol: Solution = env.clone().try_into().map_err(CapiError::errored)?;
        unsafe { new_handle_out_from_c(out, sol) }
    })
}

/// Creates a new random solution.
///
/// # Safety
/// `out` must point to writable `LmSol *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_from_random(
    n_samples: usize,
    seed: *const u64,
    context_a: *const LmEnv,
    context_b: *const LmModel,
    sense: *const LmSense,
    tol: *const f64,
    out: *mut *mut LmSol,
) -> LmStatus {
    guard(|| {
        if context_a.is_null() == context_b.is_null() {
            return Err(CapiError::invalid_argument(
                "exactly one of `context_a` or `context_b` must be non-null",
            ));
        }

        let context: Either<ArcEnv, Model> = if !context_a.is_null() {
            let env = unsafe { shared_from_c(context_a, "context_a") }?.read_arc();
            Either::Left(env.clone())
        } else if !context_b.is_null() {
            let model = unsafe { shared_from_c(context_b, "context_b") }?.read_arc();
            Either::Right(model.clone())
        } else {
            return Err(CapiError::null_pointer("context"));
        };

        let seed = unsafe { opt_from_c(seed) };
        let sense = unsafe { opt_from_c(sense).map(sense_from_c) };
        let tol = unsafe { opt_from_c(tol) };
        let sol = Solution::from_random(n_samples, seed, context, sense, tol)
            .map_err(CapiError::errored)?;
        unsafe { new_handle_out_from_c(out, sol) }
    })
}
