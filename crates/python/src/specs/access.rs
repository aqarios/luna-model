//! Accessors for Python model specs.

use lunamodel_types::{Ctype, EnumSetFromVec, Specs, Vtype};
use lunamodel_unwind::*;
use pyo3::pymethods;

use crate::{
    args::PyModelSpecsArg,
    types::{PyCtype, PySense, PyVtype},
};

use super::PyModelSpecs;

#[unwindable]
#[pymethods]
impl PyModelSpecs {
    #[new]
    fn new(
        sense: Option<PySense>,
        vtypes: Option<Vec<PyVtype>>,
        constraints: Option<Vec<PyCtype>>,
        max_degree: Option<usize>,
        max_constraint_degree: Option<usize>,
        max_num_variables: Option<usize>,
    ) -> Self {
        Self {
            s: Specs {
                sense: sense.map(|s| s.into()),
                vtypes: vtypes.map_or_else(
                    || None,
                    |vs| {
                        Some(
                            vs.into_iter()
                                .map(|v| v.into())
                                .collect::<Vec<Vtype>>()
                                .to_enumset(),
                        )
                    },
                ),
                constraints: constraints.map_or_else(
                    || None,
                    |cs| {
                        Some(
                            cs.into_iter()
                                .map(|c| c.into())
                                .collect::<Vec<Ctype>>()
                                .to_enumset(),
                        )
                    },
                ),
                max_degree,
                max_constraint_degree,
                max_num_variables,
            },
        }
    }

    #[getter]
    fn sense(&self) -> Option<PySense> {
        self.s.sense.map(|s| s.into())
    }

    #[getter]
    fn vtypes(&self) -> Option<Vec<PyVtype>> {
        self.s
            .vtypes
            .map(|t| t.into_iter().map(|v| v.into()).collect())
    }

    #[getter]
    fn constraints(&self) -> Option<Vec<PyCtype>> {
        self.s
            .constraints
            .map(|c| c.iter().map(|c| c.into()).collect())
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

    fn satisfies(&self, other: PyModelSpecsArg) -> bool {
        self.s.satisfies(&other.s)
    }
}
