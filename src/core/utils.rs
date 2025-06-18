use hashbrown::HashSet;
use std::hash::Hash;

use crate::errors::{EvaluationErr, VariableOcc};

// pub fn equal_elements<T: Eq + Hash>(a: &[T], b: &[T]) -> bool {
//     // might benefit from a lazy approach, terminating early if a mismatch is found.
//     let a: HashSet<_> = a.iter().collect();
//     let b: HashSet<_> = b.iter().collect();
//     a == b
// }

// /// Compute the elements of `a` that are not present in `b` and vice versa.
// pub fn diff<'a, T: Eq + Hash>(a: &'a[T], b: &'a[T]) -> (Vec<&'a T>, Vec<&'a T>) {
//     let a: HashSet<_> = a.iter().collect();
//     let b: HashSet<_> = b.iter().collect();
//     let only_in_a = a.difference(&b).map(|e| *e).collect();
//     let only_in_b = b.difference(&a).map(|e| *e).collect();
//     (only_in_a, only_in_b)
// }

/// Compute the elements of `a` that are not present in `b` and vice versa.
pub fn diff<T: Eq + Hash + Clone>(a: &[T], b: &[T]) -> (Vec<T>, Vec<T>) {
    let a: HashSet<_> = a.iter().collect();
    let b: HashSet<_> = b.iter().collect();
    let only_in_a = a.difference(&b).map(|e| *e).cloned().collect();
    let only_in_b = b.difference(&a).map(|e| *e).cloned().collect();
    (only_in_a, only_in_b)
}

pub fn check_variables_sol(vars_sol: &[String], vars_env: &[String]) -> Result<(), EvaluationErr> {
    // First we check the length.
    if vars_sol.len() != vars_env.len() {
        // We can directly return an error.
        return finalize_eval_err_sol(vars_sol, vars_env);
    }

    // The lengths are OK, now we need to check the contents.
    if vars_sol != vars_env {
        return finalize_eval_err_sol(vars_sol, vars_env);
    }

    // The lengths match and the contents match.
    // Everything is OK
    Ok(())
}

pub fn finalize_eval_err_sol(
    vars_sol: &[String],
    vars_env: &[String],
) -> Result<(), EvaluationErr> {
    let (only_in_sol, only_in_env) = diff(vars_sol, vars_env);
    match (only_in_sol.len(), only_in_env.len()) {
        (0, 0) => Ok(()),
        (_, 0) => Err(EvaluationErr::SolutionAndModelVariablesMismatch(
            VariableOcc::new(Some(only_in_sol), None),
        )),
        (0, _) => Err(EvaluationErr::SolutionAndModelVariablesMismatch(
            VariableOcc::new(None, Some(only_in_env)),
        )),
        (_, _) => Err(EvaluationErr::SolutionAndModelVariablesMismatch(
            VariableOcc::new(Some(only_in_sol), Some(only_in_env)),
        )),
    }
}

pub fn check_variables_sample(
    vars_sol: &[String],
    vars_env: &[String],
) -> Result<(), EvaluationErr> {
    // First we check the length.
    if vars_sol.len() != vars_env.len() {
        // We can directly return an error.
        return finalize_eval_err_sample(vars_sol, vars_env);
    }

    // The lengths are OK, now we need to check the contents.
    if vars_sol != vars_env {
        return finalize_eval_err_sample(vars_sol, vars_env);
    }

    // The lengths match and the contents match.
    // Everything is OK
    Ok(())
}

pub fn finalize_eval_err_sample(
    vars_sol: &[String],
    vars_env: &[String],
) -> Result<(), EvaluationErr> {
    let (only_in_sol, only_in_env) = diff(vars_sol, vars_env);
    match (only_in_sol.len(), only_in_env.len()) {
        (0, 0) => Ok(()),
        (_, 0) => Err(EvaluationErr::SampleAndModelVariablesMismatch(
            VariableOcc::new(Some(only_in_sol), None),
        )),
        (0, _) => Err(EvaluationErr::SampleAndModelVariablesMismatch(
            VariableOcc::new(None, Some(only_in_env)),
        )),
        (_, _) => Err(EvaluationErr::SampleAndModelVariablesMismatch(
            VariableOcc::new(Some(only_in_sol), Some(only_in_env)),
        )),
    }
}

pub fn filter_by_mask<T: Clone>(items: &Vec<T>, mask: &Vec<bool>) -> Vec<T> {
    items
        .iter()
        .zip(mask)
        .filter_map(|(x, flag)| flag.then_some(x.clone()))
        .collect()
}
