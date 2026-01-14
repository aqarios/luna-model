use hashbrown::HashMap;
use lunamodel_error::LunaModelResult;

use super::Model;
use crate::Solution;

impl Model {
    pub fn evaluate_solution(&self, sol: &Solution) -> LunaModelResult<Solution> {
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
        let mut vbounds: HashMap<String, Vec<bool>> = self
            .vars()
            .map(|n| (n.name().unwrap(), Vec::default()))
            .collect();
        let mut feasible: Vec<bool> = Vec::new();

        for sample in sol.samples() {
            obj_vals.push(self.objective.evaluate_sample(&sample)?);
            let mut all_constr_ok = true;
            for (cname, val) in self.constraints.evaluate_sample(&sample)? {
                constrs.get_mut(&cname).unwrap().push(val);
                all_constr_ok = all_constr_ok && val;
            }

            let mut all_vars_ok = true;
            for v in self.vars() {
                let name = v.name()?;
                let bs = vbounds.get_mut(&name).unwrap();
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

    // pub fn evaluate_sample<'a>(&self, sample: &Sample) -> Result<OwnedResult, EvaluationErr> {
    //     let sample_var_names = sample.variable_names();
    //     let env_var_names = &self.environment.variable_names();
    //     check_variables_sample(&sample_var_names, env_var_names)?;

    //     let index_map = make_index_map(sample.varname_to_pos(), &self.environment);

    //     let obj_val = self
    //         .objective
    //         .evaluate_sample(sample, |idx| index_map[&idx]);
    //     let cf: Vec<_> = self
    //         .constraints
    //         .iter()
    //         .map(|(_, constraint)| {
    //             let v = constraint
    //                 .lhs
    //                 .evaluate_sample(sample, |idx| index_map[&idx]);
    //             constraint.comparator.evaluate(v, constraint.rhs)
    //         })
    //         .collect();
    //     let vf: Vec<_> = self
    //         .environment
    //         .access()
    //         .evaluate_bounds(sample, |idx| index_map[&idx]);
    //     let feasible = cf.iter().all(|&b| b) && vf.iter().all(|&b| b);
    //     let owned_sample = SampleOwned::new(
    //         sample_var_names.to_vec(),
    //         sample.iter().collect(),
    //         sample.var_indices(),
    //     );
    //     Ok(OwnedResult::new(owned_sample, obj_val, cf, vf, feasible))
    // }
}
