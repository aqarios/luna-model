use std::collections::HashMap;

#[cfg(feature = "py")]
use pyo3::prelude::*;

use crate::core::varref::{VarRef, DEFAULT_SCALER_VALUE};

use super::{TermAddition, TermMultiplication, TermSubtraction};

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, PartialEq)]
pub struct Quadratic {
    variables: HashMap<u64, f64>,
}

impl Quadratic {
    pub fn from_vars(a: &VarRef, b: &VarRef) -> Self {
        let mut variables = HashMap::new();
        let key = quadratic_varref_key(a.id, b.id);
        variables.insert(key, DEFAULT_SCALER_VALUE);
        Quadratic { variables }
    }

    pub fn empty() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

impl TermSubtraction<Quadratic> for Quadratic {
    fn sub_assign(&mut self, rhs: &Quadratic) {
        // todo: implement me
        // unimplemented!()
    }
}

impl TermAddition<Quadratic> for Quadratic {
    fn add_assign(&mut self, rhs: &Quadratic) {
        // todo: implement me
        // unimplemented!()
    }
}

impl TermMultiplication<f64> for Quadratic {
    fn mul_assign(&mut self, rhs: &f64) {
        for (_, v) in self.variables.iter_mut() {
            v.mul_assign(rhs);
        }
    }
}

fn quadratic_varref_key(a: u32, b: u32) -> u64 {
    // The larger key is also at the end.
    if a < b {
        let au64 = (a as u64) << 32;
        au64 | (b as u64)
    } else if a > b {
        let bu64 = (b as u64) << 32;
        bu64 | (a as u64)
    } else {
        unimplemented!("this should never happen and we need to throw and error")
    }
}

impl TermMultiplication<VarRef> for Quadratic {
    fn mul_assign(&mut self, rhs: &VarRef) {
        for (k, v) in self.variables.iter_mut() {
            // here, we need to check if the rhs is contained in the
            // key `k`. We should be able to do this efficently using
            // binary operations.
            // The `id` of the `rhs` has to be present either on the
            // first 32 or the second 32 bits.
            let left_key = (*k >> 32) as u32;
            let right_key = *k as u32;

            if rhs.id == left_key || rhs.id == right_key {
                // the varId of the variable is already in the current
                // interaction. Thus, this multiplication creates a
                // HigherOrder expression.
            }
        }
    }
}
