use indexmap::IndexMap;
use lunamodel_core::{TryIndex, prelude::VarRef, solution::sample::SampleView};
use lunamodel_error::LunaModelError;
use lunamodel_types::Bias;
use lunamodel_unwind::*;
use numpy::{PyArray1, ToPyArray};
use pyo3::{Bound, FromPyObject, PyResult, Python, pymethods};

use super::{PyExprContent as PyEC, PyExpression};
use crate::{
    sol::{PySolution, sample::PySampleView},
    utils::VarKey,
    variable::PyVariable,
};

#[derive(FromPyObject)]
enum Replacement {
    Var(PyVariable),
    Expr(PyExpression),
}

struct DirectSample(pub IndexMap<String, f64>);

impl TryFrom<IndexMap<VarKey, f64>> for DirectSample {
    type Error = LunaModelError;

    fn try_from(value: IndexMap<VarKey, f64>) -> Result<Self, Self::Error> {
        let mut data = IndexMap::with_capacity(value.len());
        for (key, bias) in value {
            match key {
                VarKey::Str(name) => _ = data.insert(name, bias),
                VarKey::Var(var) => _ = data.insert(var.v.name()?, bias),
            }
        }
        Ok(Self(data))
    }
}

impl TryIndex<&str> for DirectSample {
    type Err = LunaModelError;
    type Output = Bias;

    fn try_index(&self, var: &str) -> Result<&Self::Output, Self::Err> {
        if self.0.contains_key(var) {
            Ok(&self.0[var])
        } else {
            Err(LunaModelError::VariableNotExisting(var.into()))
        }
    }
}

#[derive(FromPyObject)]
enum SampleIn {
    Sample(PySampleView),
    Dict(IndexMap<VarKey, f64>),
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

    fn evaluate_sample(&self, sample: SampleIn) -> PyResult<f64> {
        let res = match sample {
            SampleIn::Sample(pyview) => {
                let sol = &pyview.sol.s.read_arc();
                let view = SampleView::new(&sol, pyview.idx);
                match &self.expr {
                    PyEC::Expr(e) => e.read_arc().evaluate_sample(&view)?,
                    PyEC::Model(m) => m.read_arc().objective.evaluate_sample(&view)?,
                }
            }
            SampleIn::Dict(sample) => {
                let direct: DirectSample = sample.try_into()?;
                match &self.expr {
                    PyEC::Expr(e) => e.read_arc().evaluate_sample(&direct)?,
                    PyEC::Model(m) => m.read_arc().objective.evaluate_sample(&direct)?,
                }
            }
        };
        Ok(res)
    }

    fn substitute(&self, target: &PyVariable, replacement: Replacement) -> PyResult<PyExpression> {
        let r = match replacement {
            Replacement::Var(v) => &(v.v.into()),
            Replacement::Expr(e) => &(e.expr.into()),
        };
        Ok(self.expr.substitute(&target.v, r)?.into())
    }
}
