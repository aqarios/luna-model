use lunamodel_error::{LunaModelError, LunaModelResult};
use std::collections::{HashMap, HashSet};

use super::Model;
use crate::ops::SolutionLookup;
use crate::{Solution, solution::Column};

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

        let num_samples = sol.len();

        let mut newsol = Solution {
            samples: sol.samples.clone(),
            counts: sol.counts.clone(),
            raw_energies: sol.raw_energies.clone(),
            timing: sol.timing,
            sense: sol.sense,
            ..Default::default()
        };

        let mut obj_vals = Vec::with_capacity(num_samples);
        let mut feasible = Vec::with_capacity(num_samples);

        let mut constrs_results: Vec<Vec<bool>> = self
            .constraints
            .iter()
            .map(|_| Vec::with_capacity(num_samples))
            .collect();

        let cols: Vec<&Column> = sol.samples.values().collect();

        // Direct column-by-column variable bounds evaluation
        let sol_var_names = sol.variable_names();
        let mut vbounds: HashMap<String, Vec<bool>> = HashMap::with_capacity(sol_var_names.len());
        let mut all_vars_ok: Vec<bool> = vec![true; num_samples];

        for name in &sol_var_names {
            if let Some(col_idx) = sol.samples.get_index_of(name) {
                let v = self.environment.lookup(name)?;
                let b = v.bounds()?;
                let col = cols[col_idx];
                let bool_vec = (0..num_samples)
                    .map(|s_idx| {
                        let vok = b.evaluate(col[s_idx], tol)?;
                        if !vok {
                            all_vars_ok[s_idx] = false;
                        }
                        Ok(vok)
                    })
                    .collect::<LunaModelResult<Vec<bool>>>()?;
                vbounds.insert(name.clone(), bool_vec);
            }
        }

        let mut lookup = SolutionLookup::new(&self.environment.read_arc(), sol)?;

        for (sample, v_ok) in sol.samples().zip(all_vars_ok) {
            // Lookup update
            lookup.update(&sample)?;

            // Objective evaluation
            let obj_val = self.objective.evaluate_sample_quick(&lookup.lu)?;
            obj_vals.push(obj_val);

            // Constraint evaluation
            let mut all_constr_ok = true;
            for (val_res, c_res) in self
                .constraints
                .iter()
                .map(|(_, c)| c.evaluate_sample_quick(&lookup.lu, tol))
                .zip(constrs_results.iter_mut())
            {
                let val = val_res?;
                c_res.push(val);
                all_constr_ok = all_constr_ok && val;
            }
            feasible.push(v_ok && all_constr_ok);
        }

        newsol.obj_values = Some(obj_vals);
        newsol.feasible = Some(feasible);
        newsol.constraints = self
            .constraints
            .iter()
            .map(|(n, _)| n.clone())
            .zip(constrs_results)
            .collect();
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
    for ev in expr_vars {
        if !sample_set.contains(ev.as_str()) {
            return Err(LunaModelError::Evaluation(
                format!("variable '{ev}' is not contained in sample").into(),
            ));
        }
    }
    Ok(())
}
