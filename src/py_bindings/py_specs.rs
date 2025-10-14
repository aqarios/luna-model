use super::unwind;
use enumset::EnumSet;
use pyo3::prelude::*;
use unwind_macros::unwindable;

use crate::core::{ConstraintType, ModelSpecs, Sense, Vtype};

#[cfg_attr(
    not(feature = "lq"),
    pyclass(subclass, name = "ModelSpecs", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(subclass, name = "ModelSpecs", module = "luna_quantum._core")
)]
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
        let vtypes = if let Some(vt) = vtypes {
            let mut out = EnumSet::new();
            for t in vt {
                out.insert(t);
            }
            Some(out)
        } else {
            None
        };
        let constraints = if let Some(ct) = constraints {
            let mut out = EnumSet::new();
            for t in ct {
                out.insert(t);
            }
            Some(out)
        } else {
            None
        };
        PyModelSpecs(ModelSpecs {
            sense,
            vtypes,
            constraints,
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
