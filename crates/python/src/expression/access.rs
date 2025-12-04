use lunamodel_types::VarIdx;
use pyo3::pymethods;

use super::{content::PyExprContent as PyE, iteration::PyExpressionIterator};
use crate::{PyEnvironment, PyExpression, PyVariable};

#[pymethods]
impl PyExpression {
    #[getter]
    fn environment(&self) -> PyEnvironment {
        match &self.expr {
            PyE::Expr(e) => e.read_arc().env.clone().into(),
            PyE::Model(m) => m.read_arc().environment.clone().into(),
        }
    }

    #[getter]
    fn num_variables(&self) -> usize {
        match &self.expr {
            PyE::Expr(e) => e.read_arc().num_vars,
            PyE::Model(m) => m.read_arc().objective.num_vars,
        }
    }

    pub fn get_offset(&self) -> f64 {
        match &self.expr {
            PyE::Expr(e) => e.read_arc().offset,
            PyE::Model(m) => m.read_arc().objective.offset,
        }
    }

    pub fn get_linear(&self, var: PyVariable) -> f64 {
        match &self.expr {
            PyE::Expr(e) => e.read_arc().linear(var.v.id()),
            PyE::Model(m) => m.read_arc().objective.linear(var.v.id()),
        }
    }

    pub fn get_quadratic(&self, u: PyVariable, v: PyVariable) -> f64 {
        match &self.expr {
            PyE::Expr(e) => e.read_arc().quadratic(u.v.id(), v.v.id()),
            PyE::Model(m) => m.read_arc().objective.quadratic(u.v.id(), v.v.id()),
        }
    }

    pub fn get_higher_order(&self, vars: Vec<PyVariable>) -> f64 {
        let varsidx: Vec<VarIdx> = vars.iter().map(|v| v.v.id()).collect();
        match &self.expr {
            PyE::Expr(e) => e.read_arc().higher_order(varsidx.as_slice()),
            PyE::Model(m) => m.read_arc().objective.higher_order(varsidx.as_slice()),
        }
    }

    pub fn items(&self) -> PyExpressionIterator {
        PyExpressionIterator::new(&self)
    }

    pub fn variables(&self) -> Vec<PyVariable> {
        match &self.expr {
            PyE::Expr(e) => e.read_arc().vars().map(|v| PyVariable::new(v)).collect(),
            PyE::Model(m) => m
                .read_arc()
                .objective
                .vars()
                .map(|v| PyVariable::new(v))
                .collect(),
        }
    }

    pub fn degree(&self) -> usize {
        match &self.expr {
            PyE::Expr(e) => e.read_arc().degree(),
            PyE::Model(m) => m.read_arc().objective.degree(),
        }
    }

    pub fn linear_items(&self) -> Vec<(PyVariable, f64)> {
        match &self.expr {
            PyE::Expr(e) => e
                .read_arc()
                .linear_items()
                .map(|(v, b)| (PyVariable::new(v), b))
                .collect(),
            PyE::Model(m) => m
                .read_arc()
                .objective
                .linear_items()
                .map(|(v, b)| (PyVariable::new(v), b))
                .collect(),
        }
    }

    pub fn quadratic_items(&self) -> Vec<(PyVariable, PyVariable, f64)> {
        match &self.expr {
            PyE::Expr(e) => e
                .read_arc()
                .quadratic_items()
                .map(|(u, v, b)| (PyVariable::new(u), PyVariable::new(v), b))
                .collect(),
            PyE::Model(m) => m
                .read_arc()
                .objective
                .quadratic_items()
                .map(|(u, v, b)| (PyVariable::new(u), PyVariable::new(v), b))
                .collect(),
        }
    }

    pub fn higher_order_items(&self) -> Vec<(Vec<PyVariable>, f64)> {
        match &self.expr {
            PyE::Expr(e) => e
                .read_arc()
                .higher_order_items()
                .map(|(vs, b)| (vs.into_iter().map(|v| PyVariable::new(v)).collect(), b))
                .collect(),
            PyE::Model(m) => m
                .read_arc()
                .objective
                .higher_order_items()
                .map(|(vs, b)| (vs.into_iter().map(|v| PyVariable::new(v)).collect(), b))
                .collect(),
        }
    }

    pub fn is_constant(&self) -> bool {
        match &self.expr {
            PyE::Expr(e) => e.read_arc().is_constant(),
            PyE::Model(m) => m.read_arc().objective.is_constant(),
        }
    }

    pub fn has_quadratic(&self) -> bool {
        match &self.expr {
            PyE::Expr(e) => e.read_arc().has_quadratic(),
            PyE::Model(m) => m.read_arc().objective.has_quadratic(),
        }
    }

    pub fn has_higher_order(&self) -> bool {
        match &self.expr {
            PyE::Expr(e) => e.read_arc().has_higher_order(),
            PyE::Model(m) => m.read_arc().objective.has_higher_order(),
        }
    }
}
