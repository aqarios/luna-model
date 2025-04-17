use std::rc::Rc;

use num::NumCast;

use crate::{
    core::{expression::IndexConstraints, ConcreteSolution, RcSolution, Solution},
    errors::SolutionCreatorErr,
};

pub struct IbmTranslator {}

impl IbmTranslator {
    pub fn from_ibm() -> Result<ConcreteSolution, SolutionCreatorErr>
where
        // S: Copy, // + NumCast,
        // Index: IndexConstraints,
    {
        let sol = Solution::default();
        Ok(RcSolution(Rc::new(sol)))
    }
}
