use lunamodel_core::Model;
use lunamodel_types::{Sense, Vtype};
use lunamodel_utils::unique_by;
use pyo3::{PyResult, pymethods};

use crate::{PyConstraintCollection, PyEnvironment, PyExpression, PyVariable};

use super::PyModel;

#[pymethods]
impl PyModel {
    #[getter]
    fn get_name(&self) -> String {
        self.m.read_arc().name.clone()
    }

    #[getter]
    fn get_sense(&self) -> Sense {
        self.m.read_arc().sense
    }

    #[getter]
    fn num_variables(&self) -> usize {
        self.m.read_arc().num_variables()
    }

    #[getter]
    fn num_constraints(&self) -> usize {
        self.m.read_arc().constraints.len()
    }

    #[getter]
    fn get_objective(&self) -> PyExpression {
        PyExpression::from(self.m.clone())
    }

    #[getter]
    fn get_constraints(&self) -> PyConstraintCollection {
        self.m.read_arc().constraints.clone().into()
    }

    #[getter]
    fn environment(&self) -> PyEnvironment {
        self.m.read_arc().environment.clone().into()
    }

    fn variables(&self) -> Vec<PyVariable> {
        self.m.read_arc().vars().map(|v| v.into()).collect()
    }

    fn vtypes(&self) -> Vec<Vtype> {
        self.m.read_arc().vtypes().collect()
    }

    fn get_variable(&self, name: String) -> PyResult<PyVariable> {
        Ok(self.m.read_arc().var(&name)?.into())
    }
}
