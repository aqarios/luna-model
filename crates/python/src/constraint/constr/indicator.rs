//! Indicator constraints on the Python constraint wrapper.

use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyConstraint;
use crate::{args::PyVarArg, variable::PyVariable};

#[unwindable]
#[pymethods]
impl PyConstraint {
    fn when(&self, var: PyVarArg, value: bool) -> PyResult<PyConstraint> {
        Ok(self.c.read_arc().when(&var.v, value)?.into())
    }

    fn indicator(&self) -> PyResult<Option<(PyVariable, bool, PyConstraint)>> {
        let indicator = self.c.read_arc().indicator()?;
        Ok(indicator.map(|i| (PyVariable::new(i.variable), i.value, i.constraint.into())))
    }
}
