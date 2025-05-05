use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use num::NumCast;

use crate::{
    core::{
        expression::IndexConstraints, solution::sol::SampleCol, ConcreteSolution, MutRcEnvironment,
        RcSolution, Solution, Timing, VarRef, Vtype,
    },
    errors::SolutionCreatorErr,
};

pub struct IbmTranslator {}

impl IbmTranslator {
    pub fn from_ibm<S, E, Index>(
        samples: &Vec<Vec<S>>,
        orderings: &Vec<Vec<Rc<VarRef<Index>>>>,
        energies: &Vec<E>,
        counts: Vec<usize>,
        timing: Option<Timing>,
        env: MutRcEnvironment<Index>,
    ) -> Result<ConcreteSolution, SolutionCreatorErr>
    where
        S: Copy + NumCast + Default + Display + Debug,
        E: Copy + NumCast + Debug,
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
        let index_list: Vec<Vec<usize>> = orderings
            .iter()
            .map(|l| l.iter().map(|e| e.id.into()).collect())
            .collect();
        // For each sample:
        // Map the sample to the correct order.
        for (((sample, indices), energy), occ) in
            samples.iter().zip(index_list).zip(energies).zip(counts)
        {
            let mut s: Vec<S> = vec![S::default(); sample.len()];
            for (&idx, val) in indices.iter().zip(sample) {
                s[idx] = *val;
            }
            sol.extend(s, occ, Some(*energy))?;
        }
        Ok(RcSolution(Rc::new(sol)))
    }
}
