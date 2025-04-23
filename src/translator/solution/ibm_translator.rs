use std::rc::Rc;

use crate::{
    core::{ConcreteSolution, RcSolution, Solution},
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
