use lunamodel_types::{Ctype, EnumSetFromVec, Sense, Specs, Vtype};
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyModelSpecs;

#[unwindable]
#[pymethods]
impl PyModelSpecs {
    #[new]
    fn new(
        sense: Option<Sense>,
        vtypes: Option<Vec<Vtype>>,
        constraints: Option<Vec<Ctype>>,
        max_degree: Option<usize>,
        max_constraint_degree: Option<usize>,
        max_num_variables: Option<usize>,
    ) -> Self {
        Self {
            s: Specs {
                sense,
                vtypes: vtypes.map_or_else(|| None, |vs| Some(vs.to_enumset())),
                constraints: constraints.map_or_else(|| None, |cs| Some(cs.to_enumset())),
                max_degree,
                max_constraint_degree,
                max_num_variables,
            },
        }
    }

    #[getter]
    fn sense(&self) -> Option<Sense> {
        self.s.sense
    }

    #[getter]
    fn vtypes(&self) -> Option<Vec<Vtype>> {
        self.s.vtypes.map(|t| t.iter().collect())
    }

    #[getter]
    fn constraints(&self) -> Option<Vec<Ctype>> {
        self.s.constraints.map(|c| c.iter().collect())
    }

    #[getter]
    fn max_degree(&self) -> Option<usize> {
        self.s.max_degree
    }

    #[getter]
    fn max_constraint_degree(&self) -> Option<usize> {
        self.s.max_constraint_degree
    }

    #[getter]
    fn max_num_variables(&self) -> Option<usize> {
        self.s.max_num_variables
    }

    fn satisfies(&self, other: &Self) -> bool {
        self.s.satisfies(&other.s)
    }

    fn __str__(&self) -> String {
        self.s.to_string()
    }
}
