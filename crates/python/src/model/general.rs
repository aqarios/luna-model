use std::ops::Mul;

use lunamodel_core::{Expression, prelude::ContentEquality, solution::sample::SampleView};
use lunamodel_error::LunaModelResult;
use lunamodel_unwind::*;
use pyo3::{FromPyObject, PyResult, pymethods};

use super::PyModel;
use crate::{
    PyConstraintCollection, PyExpression, PyModelSpecs, PySolution, PyVariable,
    sol::{result::PyResultView, sample::PySampleView},
};

#[derive(FromPyObject)]
enum Replacement {
    Expr(PyExpression),
    Var(PyVariable),
}

impl Replacement {
    fn as_expr(self) -> LunaModelResult<Expression> {
        match self {
            Replacement::Expr(expr) => Ok(expr.into()),
            Replacement::Var(var) => (&var.v).mul(1.0),
        }
    }
}

#[unwindable]
#[pymethods]
impl PyModel {
    fn substitute(&mut self, target: &PyVariable, replacement: Replacement) -> PyResult<()> {
        Ok(self
            .m
            .write_arc()
            .substitute(&target.v, &replacement.as_expr()?)?)
    }

    fn evaluate(&self, solution: PySolution) -> PyResult<PySolution> {
        Ok(self
            .m
            .read_arc()
            .evaluate_solution(&solution.s.read_arc())?
            .into())
    }

    fn evaluate_sample(&self, sample: PySampleView) -> PyResult<PyResultView> {
        let mut sol = sample.sol.s.read_arc().extract(sample.idx);
        sol = self.m.read_arc().evaluate_solution(&sol)?;
        Ok(PyResultView::new(sol.into(), 0))
    }

    fn equal_contents(&self, other: &Self) -> bool {
        self.m.read_arc().equal_contents(&other.m.read_arc())
    }

    fn violated_constraints(&self, sample: PySampleView) -> PyResult<PyConstraintCollection> {
        let binding = sample.sol.s.read_arc();
        let sample = SampleView::new(&binding, sample.idx);
        Ok(self.m.read_arc().violated_constraints(&sample)?.into())
    }

    fn satisfies(&self, specs: &PyModelSpecs) -> bool {
        self.m.read_arc().satisfies(&specs.s)
    }

    fn deep_clone(&self) -> Self {
        self.m.read_arc().deep_clone().into()
    }
}
