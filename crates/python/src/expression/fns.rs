use lunamodel_core::prelude::VarRef;
use numpy::PyArray1;
use pyo3::{Bound, FromPyObject, PyResult, Python, pymethods};

use super::{PyExprContent as PyEC, PyExpression};
use crate::{sol::PySolution, variable::PyVariable};

#[derive(FromPyObject)]
enum Replacement {
    Var(PyVariable),
    Expr(PyExpression),
}

#[pymethods]
impl PyExpression {
    fn separate(&self, variables: Vec<PyVariable>) -> (PyExpression, PyExpression) {
        let vars: Vec<VarRef> = variables.iter().map(|v| v.v.clone()).collect();
        let (left, right) = match &self.expr {
            PyEC::Expr(e) => e.read_arc().separate(vars.as_slice()),
            PyEC::Model(m) => m.read_arc().objective.separate(vars.as_slice()),
        };
        (left.into(), right.into())
    }

    fn evaluate(&self, py: Python<'_>, sol: &PySolution) -> PyResult<Bound<'_, PyArray1<f64>>> {
        _ = py;
        _ = sol;
        unimplemented!()
    }

    fn subsitute(
        &self,
        target: &PyVariable,
        replacement: Replacement,
    ) -> PyResult<Bound<'_, PyArray1<f64>>> {
        _ = target;
        match replacement {
            Replacement::Var(v) => _ = v,
            Replacement::Expr(e) => _ = e,
        }
        unimplemented!()
    }
}
