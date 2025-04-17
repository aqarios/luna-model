use std::rc::Rc;

use num::NumCast;

use crate::{
    core::{
        expression::IndexConstraints, solution::sol::SampleCol, ConcreteSolution, MutRcEnvironment,
        RcSolution, Solution, Timing, VarRef, Vtype,
    },
    errors::SolutionCreatorErr,
};

pub struct QctrlTranslator {}

impl QctrlTranslator {
    pub fn from_qctrl<S, E, Index>(
        sample: &[S],
        energy: E,
        variable_list: Option<Vec<Rc<VarRef<Index>>>>,
        timing: Option<Timing>,
        env: MutRcEnvironment<Index>,
    ) -> Result<ConcreteSolution, SolutionCreatorErr>
    where
        S: Copy + NumCast + Default,
        E: Copy + NumCast,
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
        // used to determine the order of each assignment in the sample.
        // if not specified by the parameter, we use the order in which the variables
        // where created.
        let index_list: Vec<usize> = variable_list
            .and_then(|vl| Some(vl.iter().map(|e| e.id.into()).collect()))
            .unwrap_or_else(|| Self::create_variable_list(env));
        // Map the sample to the correct order.
        let mut s: Vec<S> = vec![S::default(); sample.len()];
        for (&idx, val) in index_list.iter().zip(sample) {
            s[idx] = *val;
        }
        sol.extend(s, 1, Some(energy))?;
        Ok(RcSolution(Rc::new(sol)))
    }

    fn create_variable_list<Index: IndexConstraints>(env: MutRcEnvironment<Index>) -> Vec<usize> {
        (0..env.borrow().varcount.into()).collect()
    }
}
