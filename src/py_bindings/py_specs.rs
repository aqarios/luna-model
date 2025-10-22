use super::unwind;
use pyo3::prelude::*;
use unwind_macros::unwindable;

use crate::core::{ConstraintType, EnumSetFromVec, ModelSpecs, Sense, Vtype};

#[pyclass(subclass, name = "ModelSpecs", module = "luna_model._core")]
#[derive(Clone)]
pub struct PyModelSpecs(pub ModelSpecs);

#[unwindable]
#[pymethods]
impl PyModelSpecs {
    #[new]
    #[pyo3(signature=(sense=None, vtypes=None, constraints=None, max_degree=None, max_constraint_degree=None, max_num_variables=None))]
    fn py_new(
        sense: Option<Sense>,
        vtypes: Option<Vec<Vtype>>,
        constraints: Option<Vec<ConstraintType>>,
        max_degree: Option<usize>,
        max_constraint_degree: Option<usize>,
        max_num_variables: Option<usize>,
    ) -> Self {
        PyModelSpecs(ModelSpecs {
            sense,
            vtypes: vtypes.map_or_else(|| None, |vs| Some(vs.to_enumset())),
            constraints: constraints.map_or_else(|| None, |cs| Some(cs.to_enumset())),
            max_degree,
            max_constraint_degree,
            max_num_variables,
        })
    }

    #[getter]
    fn get_sense(&self) -> Option<Sense> {
        self.0.sense
    }

    #[getter]
    fn get_vtypes(&self) -> PyResult<Option<Vec<Vtype>>> {
        match self.0.vtypes {
            Some(vt) => Ok(Some(vt.iter().collect())),
            None => Ok(None),
        }
    }

    #[getter]
    fn get_constraints(&self) -> PyResult<Option<Vec<Vtype>>> {
        match self.0.vtypes {
            Some(vt) => Ok(Some(vt.iter().collect())),
            None => Ok(None),
        }
    }

    #[getter]
    fn get_max_degree(&self) -> Option<usize> {
        self.0.max_degree
    }

    #[getter]
    fn get_max_constraint_degree(&self) -> Option<usize> {
        self.0.max_constraint_degree
    }

    #[getter]
    fn get_max_num_variables(&self) -> Option<usize> {
        self.0.max_num_variables
    }

    fn satisfies(&self, other: Self) -> bool {
        return self.0.satisfies(other.0);
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
}
