use crate::core::environment::SharedEnvironment;
use crate::core::{RcSolution, Sense, Solution, Timing};
use crate::errors::SolutionCreationErr;
use std::collections::HashMap;
use std::rc::Rc;

pub struct ZibTranslator {}

impl ZibTranslator {
    pub fn from_zib(
        sample: HashMap<String, f64>,
        sense: Sense,
        timing: Option<Timing>,
        env: SharedEnvironment,
    ) -> Result<RcSolution, SolutionCreationErr> {
        let mut sol = Solution::with_sense(sense);
        sol.create_columns(&env, 1);
        sol.timing = timing;
        sol.variable_names = env.variable_names();
        let sample_vec: Vec<_> = env
            .borrow()
            .variables()
            .iter()
            .map(|x| *sample.get(&x.name).unwrap())
            .collect();
        sol.extend::<f64, f64>(&sample_vec, 1, None)?;
        Ok(RcSolution(Rc::new(sol)))
    }
}
