use lunamodel_hashing::hash_model;
use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyModel;
use crate::{
    PyConstraintCollection, PyEnvironment, PyExpression, PyModelMetadata, PyModelSpecs, PyVariable,
    types::{PySense, PyVtype},
};

#[unwindable]
#[pymethods]
impl PyModel {
    #[getter]
    fn _metadata(&self) -> PyModelMetadata {
        self._metadata.clone()
    }

    #[getter]
    fn get_name(&self) -> String {
        self.m.read_arc().name.clone()
    }

    #[getter]
    fn get_sense(&self) -> PySense {
        self.m.read_arc().sense.into()
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
        PyConstraintCollection::for_model(self.m.clone())
    }

    #[getter]
    fn environment(&self) -> PyEnvironment {
        self.m.read_arc().environment.clone().into()
    }

    fn variables(&self) -> Vec<PyVariable> {
        self.m.read_arc().vars().map(|v| v.into()).collect()
    }

    fn vtypes(&self) -> Vec<PyVtype> {
        self.m.read_arc().vtypes().map(|v| v.into()).collect()
    }

    fn get_variable(&self, name: String) -> PyResult<PyVariable> {
        Ok(self.m.read_arc().var(&name)?.into())
    }

    fn hash(&self) -> PyResult<u64> {
        Ok(hash_model(&self.m.read_arc()))
    }

    fn __hash__(&self) -> PyResult<u64> {
        self.hash()
    }

    fn get_specs(&self) -> PyModelSpecs {
        self.m.read_arc().specs().into()
    }
}
