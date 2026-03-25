use std::collections::HashMap;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;

use super::Model;
use crate::{Solution, ops::make_lookup};

impl Model {
    pub fn evaluate_solution(&self, sol: &Solution) -> LunaModelResult<Solution> {
        check_alignment(
            &self.vars().map(|v| v.name().unwrap()).collect::<Vec<_>>(),
            &sol.variable_names(),
        )?;

        let mut newsol = Solution::default();
        newsol.samples = sol.samples.clone();
        newsol.counts = sol.counts.clone();
        newsol.raw_energies = sol.raw_energies.clone();
        newsol.timing = sol.timing.clone();
        newsol.sense = sol.sense.clone();

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
            make_lookup(&self.environment.read_arc(), &sample, &mut lu)?;
            obj_vals.push(self.objective.evaluate_sample_quick(&lu)?);
            let mut all_constr_ok = true;
            for (cname, val) in self.constraints.evaluate_sample_quick(&lu)? {
                constrs.get_mut(&cname).unwrap().push(val);
                all_constr_ok = all_constr_ok && val;
            }
            let mut all_vars_ok = true;
            for name in sol.variable_names() {
                let bs = vbounds.get_mut(&name).unwrap();
                let v = self.environment.lookup(&name)?;
                let vok = v.evaluate(sample[&name])?;
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

fn check_alignment(expr_vars: &[String], sample_vars: &[String]) -> LunaModelResult<()> {
    // Removed checks to allow solutions with more variables than the model.
    // if expr_vars.len() != sample_vars.len() {
    //     return Err(LunaModelError::Evaluation(
    //         "number of variables does not match".into(),
    //     ));
    // }
    for ev in expr_vars {
        if !sample_vars.contains(ev) {
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
