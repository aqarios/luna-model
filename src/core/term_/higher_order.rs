use std::collections::HashMap;

#[cfg(feature = "py")]
use pyo3::prelude::*;

use super::{number::Number, TermAddition, TermMultiplication, TermSubtraction};

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, PartialEq)]
pub struct HigherOrder {
    variables: HashMap<u64, Number>,
}

impl HigherOrder {
    pub fn empty() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

impl TermSubtraction<HigherOrder> for HigherOrder {
    fn sub_assign(&mut self, rhs: &HigherOrder) {
        // todo: implement me
        // unimplemented!()
    }
}

impl TermAddition<HigherOrder> for HigherOrder {
    fn add_assign(&mut self, rhs: &HigherOrder) {
        // todo: implement me
        // unimplemented!()
    }
}

impl TermMultiplication<f64> for HigherOrder {
    fn mul_assign(&mut self, rhs: &f64) {
        for (_, v) in self.variables.iter_mut() {
            v.mul_assign(rhs);
        }
    }
}
