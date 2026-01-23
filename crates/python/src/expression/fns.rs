use lunamodel_core::prelude::VarRef;
use lunamodel_unwind::unwindable;
use numpy::{PyArray1, ToPyArray};
use pyo3::{Bound, FromPyObject, PyResult, Python, pymethods};

use super::{PyExprContent as PyEC, PyExpression};
use crate::unwind::unwind;
use crate::{sol::PySolution, variable::PyVariable};

#[derive(FromPyObject)]
enum Replacement {
    Var(PyVariable),
    Expr(PyExpression),
}

#[unwindable]
#[pymethods]
impl PyExpression {
    fn separate(&self, variables: Vec<PyVariable>) -> PyResult<(PyExpression, PyExpression)> {
        let vars: Vec<VarRef> = variables.iter().map(|v| v.v.clone()).collect();
        let (left, right) = match &self.expr {
            PyEC::Expr(e) => e.read_arc().separate(vars.as_slice()),
            PyEC::Model(m) => m.read_arc().objective.separate(vars.as_slice()),
        }?;
        Ok((left.into(), right.into()))
    }

    fn evaluate<'py>(
        &self,
        py: Python<'py>,
        sol: &PySolution,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = match &self.expr {
            PyEC::Expr(e) => e.read_arc().evaluate_sampleset(sol.s.read_arc().samples()),
            PyEC::Model(m) => m
                .read_arc()
                .objective
                .evaluate_sampleset(sol.s.read_arc().samples()),
        }?;
        Ok(values.to_pyarray(py))
    }

    fn substitute(&self, target: &PyVariable, replacement: Replacement) -> PyResult<PyExpression> {
        let r = match replacement {
            Replacement::Var(v) => &(v.v.into()),
            Replacement::Expr(e) => &(e.expr.into()),
        };
        Ok(self.expr.substitute(&target.v, r)?.into())
    }
}
