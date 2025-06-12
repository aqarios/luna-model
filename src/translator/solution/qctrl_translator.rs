use std::rc::Rc;

use num::NumCast;

use crate::{
    core::{
        environment::SharedEnvironment, expression::IndexConstraints, solution::sol::SampleCol, RcSolution, Solution, Timing, Vtype
    },
    errors::SolutionCreationErr,
};

pub struct QctrlTranslator {}

impl QctrlTranslator {
    pub fn from_qctrl<S, E, Index>(
        samples: Vec<Vec<S>>,
        counts: Vec<usize>,
        energies: Vec<Option<E>>,
        timing: Option<Timing>,
        env: SharedEnvironment
    ) -> Result<RcSolution, SolutionCreationErr>
    where
        S: Copy + NumCast + Default,
        E: Copy + NumCast,
        Index: IndexConstraints,
    {
        let mut sol = Solution::default();
        for v in env.borrow().variables.iter() {
            match v.vtype {
                Vtype::Binary => {
                    sol.add_column(SampleCol::Binary(Vec::with_capacity(samples.len())))
                }
                Vtype::Spin => sol.add_column(SampleCol::Spin(Vec::with_capacity(samples.len()))),
                Vtype::Integer => {
                    sol.add_column(SampleCol::Integer(Vec::with_capacity(samples.len())))
                }
                Vtype::Real => sol.add_column(SampleCol::Real(Vec::with_capacity(samples.len()))),
            }
        }
        sol.timing = timing;
        sol.variable_names = env.borrow().iter().map(|v| v.name.clone()).collect();

        for ((sample, count), energy) in samples.iter().zip(counts).zip(energies) {
            sol.extend(&sample, count, energy)?;
        }
        Ok(RcSolution(Rc::new(sol)))
    }
}
