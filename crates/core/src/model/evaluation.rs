use hashbrown::HashMap;
use lunamodel_error::LunaModelResult;

use crate::Solution;

use super::Model;

impl Model {
    pub fn evaluate_solution(&self, sol: &Solution) -> LunaModelResult<Solution> {
        // let vars_sol = &sol.variable_names();
        // let vars_env = &self.environment.variable_names();
        // check_variables_sol(vars_sol, vars_env)?;
        //
        // let index_map = make_index_map(sol.varname_to_pos(), &self.environment);
        // let mut obj_values = Vec::with_capacity(sol.n_samples);
        // let mut constr = Vec::with_capacity(sol.n_samples);
        // let mut vb = Vec::with_capacity(sol.n_samples);
        let mut newsol = Solution::default();
        newsol.samples = sol.samples.clone();
        newsol.counts = sol.counts.clone();
        newsol.raw_energies = sol.raw_energies.clone();
        newsol.timing = sol.timing.clone();
        newsol.n_samples = sol.n_samples.clone();
        newsol.sense = sol.sense.clone();

        let mut obj_vals = Vec::new();
        let mut vbounds = self
            .vars()
            .map(|n| (n.name().unwrap(), Vec::default()))
            .collect::<HashMap<String, Vec<bool>>>();

        for sample in sol.samples() {
            obj_vals.push(self.objective.evaluate_sample(&sample)?);
            // TODO: need to compute the variable bounds and constraints + feasible
        }


        newsol.obj_values = Some(obj_vals);
        newsol.variable_bounds = vbounds;

        Ok(newsol)

        // for sample in sol.iter_samples() {
        //     let obj_val = self
        //         .objective
        //         .evaluate_sample(&sample, |var_idx| index_map[&var_idx].into());
        //     constr.push(
        //         self.constraints
        //             .iter()
        //             .map(|(_, constr)| {
        //                 constr.evaluate_sample(&sample, |var_idx| index_map[&var_idx].into())
        //             })
        //             .collect(),
        //     );
        //     vb.push(
        //         self.environment
        //             .access()
        //             .evaluate_bounds(&sample, |var_idx| index_map[&var_idx].into()),
        //     );
        //     obj_values.push(obj_val);
        // }
        // sol.add_eval_data(obj_values, constr, vb);
        // Ok(sol)
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
