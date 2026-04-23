use indexmap::IndexMap;
use lunamodel_core::{TryIndex, prelude::VarRef, solution::sample::SampleView};
use lunamodel_error::LunaModelError;
use lunamodel_types::Bias;
use lunamodel_unwind::*;
use numpy::{PyArray1, ToPyArray};
use pyo3::{Bound, FromPyObject, PyResult, Python, pymethods};

use super::PyExpression;
use crate::{
    args::{PyExprArg, PySolArg, PyVarArg},
    sol::sample::PySampleView,
    utils::VarKey,
};

#[derive(FromPyObject)]
enum Replacement {
    Var(PyVarArg),
    Expr(PyExprArg),
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
    fn separate(&self, variables: Vec<PyVarArg>) -> PyResult<(PyExpression, PyExpression)> {
        let vars: Vec<VarRef> = variables.iter().map(|v| v.v.clone()).collect();
        let (left, right) = self.read_with(|e| e.separate(vars.as_slice()))?;
        Ok((left.into(), right.into()))
    }

    fn evaluate<'py>(&self, py: Python<'py>, sol: PySolArg) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = self.read_with(|e| e.evaluate_sampleset(sol.s.read_arc().samples()))?;
        Ok(values.to_pyarray(py))
    }

    fn evaluate_sample(&self, sample: SampleIn) -> PyResult<f64> {
        let res = match sample {
            SampleIn::Sample(pyview) => {
                let sol = &pyview.sol.s.read_arc();
                let view = SampleView::new(sol, pyview.idx);
                self.read_with(|e| e.evaluate_sample(&view))?
            }
            SampleIn::Dict(sample) => {
                let direct: DirectSample = sample.try_into()?;
                self.read_with(|e| e.evaluate_sample(&direct))?
            }
        };
        Ok(res)
    }

    fn substitute(&self, target: PyVarArg, replacement: Replacement) -> PyResult<PyExpression> {
        let r = match replacement {
            Replacement::Var(v) => &(v.0.v.into()),
            Replacement::Expr(e) => &(e.0.expr.into()),
        };
        Ok(self.read_with(|e| e.substitute(&target.v, r))?.into())
    }
}
