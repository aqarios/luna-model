//! Model evaluation against solutions and samples.

use itertools::Itertools;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;
use std::collections::{HashMap, HashSet};

use super::Model;
use crate::{Solution, ops::make_lookup};

impl Model {
    /// Evaluates a solution against the model.
    ///
    /// This populates derived solution fields such as `obj_values`,
    /// `constraints`, `variable_bounds`, and `feasible` while preserving the
    /// original column-oriented sample data and any raw solver energies.
    ///
    /// Solutions are aligned by variable name rather than environment index.
    pub fn evaluate_solution(&self, sol: &Solution) -> LunaModelResult<Solution> {
        self.evaluate_solution_with_tol(sol, None)
    }

    /// Evaluates a solution against the model using an optional comparison tolerance.
    ///
    /// `tol` is used when checking constraint comparisons (`==`, `<=`, and
    /// `>=`) so small floating-point drift does not make otherwise feasible
    /// samples fail constraint evaluation. If `tol` is `None`, the default
    /// tolerance is used by the underlying comparator.
    ///
    /// The returned solution has updated objective values, constraint results,
    /// variable-bound results, and feasibility flags.
    pub fn evaluate_solution_with_tol(
        &self,
        sol: &Solution,
        tol: Option<f64>,
    ) -> LunaModelResult<Solution> {
        check_alignment(
            &self.vars().map(|v| v.name().unwrap()).collect::<Vec<_>>(),
            &sol.variable_names(),
        )?;

        let mut newsol = Solution {
            samples: sol.samples.clone(),
            counts: sol.counts.clone(),
            raw_energies: sol.raw_energies.clone(),
            timing: sol.timing,
            sense: sol.sense,
            ..Default::default()
        };

        let mut obj_vals = Vec::new();
        let mut constrs: HashMap<String, Vec<bool>> = self
            .constraints
            .iter()
            .map(|(n, _)| (n.clone(), Vec::default()))
            .collect();
        let mut vbounds: HashMap<String, Vec<bool>> = sol
            .variable_names()
            .into_iter()
            .map(|n| (n, Vec::default()))
            .collect();
        let mut feasible: Vec<bool> = Vec::new();

        let mut lu: Vec<Bias> = vec![0.0; self.environment.read_arc().next_idx as usize];
        for sample in sol.samples() {
            make_lookup(
                &self.environment.read_arc(),
                self.objective
                    .complete_vars()
                    .chain(self.constraints.complete_vars())
                    .map(|v| v.id())
                    .unique(),
                &sample,
                &mut lu,
            )?;
            obj_vals.push(self.objective.evaluate_sample_quick(&lu)?);
            let mut all_constr_ok = true;
            for (cname, val) in self.constraints.evaluate_sample_quick(&lu, tol)? {
                constrs.get_mut(&cname).unwrap().push(val);
                all_constr_ok = all_constr_ok && val;
            }
            let mut all_vars_ok = true;
            for name in sol.variable_names() {
                let bs = vbounds.get_mut(&name).unwrap();
                let v = self.environment.lookup(&name)?;
                let vok = v.evaluate(sample[&name], tol)?;
                bs.push(vok);
                all_vars_ok = all_vars_ok && vok;
            }
            feasible.push(all_vars_ok && all_constr_ok);
        }

        newsol.obj_values = Some(obj_vals);
        newsol.feasible = Some(feasible);
        newsol.constraints = constrs;
        newsol.variable_bounds = vbounds;

        Ok(newsol)
    }
}

/// Verifies that the solution contains every variable needed by the model.
///
/// Extra variables in the solution are currently tolerated; missing variables
/// are not.
fn check_alignment(expr_vars: &[String], sample_vars: &[String]) -> LunaModelResult<()> {
    let sample_set: HashSet<&str> = sample_vars.iter().map(String::as_str).collect();
    // Removed checks to allow solutions with more variables than the model.
    // if expr_vars.len() != sample_vars.len() {
    //     return Err(LunaModelError::Evaluation(
    //         "number of variables does not match".into(),
    //     ));
    // }
    for ev in expr_vars {
        if !sample_set.contains(ev.as_str()) {
            return Err(LunaModelError::Evaluation(
                format!("variable '{ev}' is not contained in sample").into(),
            ));
        }
    }
    // for sv in sample_vars {
    //     if !expr_vars.contains(sv) {
    //         return Err(LunaModelError::Evaluation(
    //             format!("variable '{sv}' is not contained in expression").into(),
    //         ));
    //     }
    // }
    Ok(())
}
