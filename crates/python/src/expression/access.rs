//! Accessors for Python expressions.

use lunamodel_types::VarIdx;
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::iteration::PyExpressionIterator;
use crate::{PyEnvironment, PyExpression, PyVariable, args::PyVarArg};

#[unwindable]
#[pymethods]
impl PyExpression {
    #[getter]
    pub fn environment(&self) -> PyEnvironment {
        self.expr.read_with(|e| e.env.clone().into())
    }

    #[getter]
    fn num_variables(&self) -> usize {
        self.read_with(|e| e.num_vars())
    }

    pub fn get_offset(&self) -> f64 {
        self.read_with(|e| e.offset)
    }

    pub fn get_linear(&self, var: PyVarArg) -> f64 {
        self.read_with(|e| e.linear(var.v.id()))
    }

    pub fn get_quadratic(&self, u: PyVarArg, v: PyVarArg) -> f64 {
        self.read_with(|e| e.quadratic(u.v.id(), v.v.id()))
    }

    pub fn get_higher_order(&self, vars: Vec<PyVarArg>) -> f64 {
        let varsidx: Vec<VarIdx> = vars.iter().map(|v| v.v.id()).collect();
        self.read_with(|e| e.higher_order(varsidx.as_slice()))
    }

    pub fn items(&self) -> PyExpressionIterator {
        PyExpressionIterator::new(self)
    }

    pub fn variables(&self) -> Vec<PyVariable> {
        self.read_with(|e| e.vars().map(PyVariable::new).collect())
    }

    pub fn degree(&self) -> usize {
        self.read_with(|e| e.degree())
    }

    pub fn linear_items(&self) -> Vec<(PyVariable, f64)> {
        self.read_with(|e| {
            e.linear_items()
                .map(|(v, b)| (PyVariable::new(v), b))
                .collect()
        })
    }

    pub fn quadratic_items(&self) -> Vec<(PyVariable, PyVariable, f64)> {
        self.read_with(|e| {
            e.quadratic_items()
                .map(|(u, v, b)| (PyVariable::new(u), PyVariable::new(v), b))
                .collect()
        })
    }

    pub fn higher_order_items(&self) -> Vec<(Vec<PyVariable>, f64)> {
        self.read_with(|e| {
            e.higher_order_items()
                .map(|(vs, b)| (vs.into_iter().map(PyVariable::new).collect(), b))
                .collect()
        })
    }

    pub fn is_constant(&self) -> bool {
        self.read_with(|e| e.is_constant())
    }

    pub fn has_quadratic(&self) -> bool {
        self.read_with(|e| e.has_quadratic())
    }

    pub fn has_higher_order(&self) -> bool {
        self.read_with(|e| e.has_higher_order())
    }
}
