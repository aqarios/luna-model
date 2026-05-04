//! General model operations exposed to Python.

use std::ops::Mul;

use lunamodel_core::{Expression, prelude::ContentEquality, solution::sample::SampleView};
use lunamodel_error::LunaModelResult;
use lunamodel_unwind::*;
use pyo3::{FromPyObject, PyResult, pymethods};

use super::PyModel;
use crate::{
    PyConstraintCollection, PySolution,
    args::{PyExprArg, PyModelArg, PyModelSpecsArg, PySolArg, PyVarArg},
    sol::{result::PyResultView, sample::PySampleView},
};

#[derive(FromPyObject)]
enum Replacement {
    Expr(PyExprArg),
    Var(PyVarArg),
}

impl From<Replacement> for LunaModelResult<Expression> {
    fn from(value: Replacement) -> Self {
        match value {
            Replacement::Expr(expr) => Ok(expr.into()),
            Replacement::Var(var) => (&var.v).mul(1.0),
        }
    }
}

#[unwindable]
#[pymethods]
impl PyModel {
    /// Substitute one variable in the model with another expression.
    ///
    /// This mutates the underlying Rust model in place, updating objective and
    /// constraints through the core substitution logic.
    fn substitute(&mut self, target: PyVarArg, replacement: Replacement) -> PyResult<()> {
        Ok(self.m.write_arc().substitute(
            &target.v,
            &Into::<LunaModelResult<Expression>>::into(replacement)?,
        )?)
    }

    #[pyo3(signature = (solution, tol=None))]
    /// Evaluate the model against a full solution and return the enriched result.
    fn evaluate(&self, solution: PySolArg, tol: Option<f64>) -> PyResult<PySolution> {
        Ok(self
            .m
            .read_arc()
            .evaluate_solution_with_tol(&solution.s.read_arc(), tol)?
            .into())
    }

    #[pyo3(signature = (sample, tol=None))]
    /// Evaluate the model against a single sample view.
    ///
    /// The sample is first extracted into a one-row solution because the core
    /// evaluation path works on full `Solution` objects.
    fn evaluate_sample(&self, sample: PySampleView, tol: Option<f64>) -> PyResult<PyResultView> {
        let mut sol = sample.sol.s.read_arc().extract(sample.idx);
        sol = self.m.read_arc().evaluate_solution_with_tol(&sol, tol)?;
        Ok(PyResultView::new(sol.into(), 0))
    }

    /// Compare two models by content instead of pointer identity.
    fn equal_contents(&self, other: PyModelArg) -> bool {
        self.m.read_arc().equal_contents(&other.m.read_arc())
    }

    #[pyo3(signature = (sample, tol=None))]
    /// Return the constraints violated by a specific sample.
    fn violated_constraints(&self, sample: PySampleView, tol: Option<f64>) -> PyResult<PyConstraintCollection> {
        let binding = sample.sol.s.read_arc();
        let sample = SampleView::new(&binding, sample.idx);
        Ok(self.m.read_arc().violated_constraints(&sample, tol)?.into())
    }

    /// Test whether the model satisfies a set of structural specs.
    fn satisfies(&self, specs: PyModelSpecsArg) -> bool {
        self.m.read_arc().satisfies(&specs.s)
    }

    /// Deep-clone the model and its environment content.
    fn deep_clone(&self) -> Self {
        self.m.read_arc().deep_clone().into()
    }
}
