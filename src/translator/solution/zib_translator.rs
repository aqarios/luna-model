use crate::core::environment::SharedEnvironment;
use crate::core::expression::IndexConstraints;
use crate::core::solution::sol::SampleCol;
use crate::core::{RcSolution, Solution, Timing, Vtype};
use crate::errors::SolutionCreationErr;
use std::collections::HashMap;
use std::rc::Rc;

pub struct ZibTranslator {}

impl ZibTranslator {
    pub fn from_zib<Index>(
        sample: HashMap<String, f64>,
        timing: Option<Timing>,
        env: SharedEnvironment
    ) -> Result<RcSolution, SolutionCreationErr>
    where
        Index: IndexConstraints,
    {
        let mut sol = Solution::default();
        for v in env.borrow().variables.iter() {
            match v.vtype {
                Vtype::Binary => sol.add_column(SampleCol::Binary(Vec::with_capacity(1))),
                Vtype::Spin => sol.add_column(SampleCol::Spin(Vec::with_capacity(1))),
                Vtype::Integer => sol.add_column(SampleCol::Integer(Vec::with_capacity(1))),
                Vtype::Real => sol.add_column(SampleCol::Real(Vec::with_capacity(1))),
            }
        }
        sol.timing = timing;
        sol.variable_names = env.borrow().iter().map(|v| v.name.clone()).collect();
        let sample_vec: Vec<_> = env
            .borrow()
            .variables
            .iter()
            .map(|x| *sample.get(&x.name).unwrap())
            .collect();
        sol.extend::<f64, f64>(&sample_vec, 1, None)?;
        Ok(RcSolution(Rc::new(sol)))
    }
}
