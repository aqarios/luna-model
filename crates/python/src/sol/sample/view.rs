use std::collections::HashMap;

use itertools::Itertools;
use lunamodel_core::solution::{
    Assignment,
    sample::{SampleView, SampleViewIdx},
};
use pyo3::{FromPyObject, IntoPyObjectExt, Py, PyAny, PyResult, Python, pyclass, pymethods};

use crate::{
    PyVariable,
    sol::{PySolution, sample::iter::PySampleIterator},
};

#[pyclass]
#[derive(Clone)]
pub struct PySampleView {
    pub sol: PySolution,
    pub idx: usize,
}

impl PySampleView {
    pub fn new(sol: PySolution, idx: usize) -> Self {
        Self { sol, idx }
    }
}

#[derive(FromPyObject)]
pub(super) enum PySampleIndex {
    Num(usize),
    Str(String),
    Var(PyVariable),
    Other(Py<PyAny>),
}

#[pymethods]
impl PySampleView {
    fn to_dict(&self) -> HashMap<String, f64> {
        self.sol
            .s
            .read_arc()
            .samples
            .iter()
            .map(|(varname, col)| (varname.clone(), col[self.idx]))
            .collect()
    }

    pub(super) fn __getitem__(&self, py: Python, item: PySampleIndex) -> PyResult<Py<PyAny>> {
        let binding = self.sol.s.read_arc();
        let sample = SampleView::new(&binding, self.idx);
        let assignemnt = match item {
            PySampleIndex::Num(index) => sample.try_get(SampleViewIdx::Num(index))?,
            PySampleIndex::Str(name) => sample.try_get(SampleViewIdx::Str(name))?,
            PySampleIndex::Var(var) => sample.try_get(SampleViewIdx::Var(var.v))?,
            PySampleIndex::Other(other) => {
                let pyvar: PyVariable = other.getattr(py, "_v")?.extract(py)?;
                sample.try_get(SampleViewIdx::Var(pyvar.v))?
            }
        };

        match assignemnt {
            Assignment::Binary(b) => b.into_py_any(py),
            Assignment::Spin(s) => s.into_py_any(py),
            Assignment::Integer(i) => i.into_py_any(py),
            Assignment::Real(r) => r.into_py_any(py),
        }
    }

    pub(super) fn __len__(&self) -> usize {
        self.sol.s.read_arc().sample_len()
    }

    fn __iter__(&self) -> PySampleIterator {
        PySampleIterator::new(self.clone())
    }

    fn __str__(&self) -> String {
        format!(
            "[{}]",
            self.sol
                .s
                .read_arc()
                .samples
                .iter()
                .map(|(_, col)| col.as_assignment(self.idx).to_string())
                .join(", ")
        )
    }
}
