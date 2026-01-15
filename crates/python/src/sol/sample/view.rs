use std::collections::HashMap;

use lunamodel_core::solution::sample::{SampleView, SampleViewIdx};
use pyo3::{FromPyObject, Py, PyAny, PyResult, Python, pyclass, pymethods};

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

    pub(super) fn __getitem__(&self, py: Python, item: PySampleIndex) -> PyResult<f64> {
        let binding = self.sol.s.read_arc();
        let sample = SampleView::new(&binding, self.idx);
        Ok(match item {
            PySampleIndex::Num(index) => sample.try_get(SampleViewIdx::Num(index))?,
            PySampleIndex::Str(name) => sample.try_get(SampleViewIdx::Str(name))?,
            PySampleIndex::Var(var) => sample.try_get(SampleViewIdx::Var(var.v))?,
            PySampleIndex::Other(other) => {
                let pyvar: PyVariable = other.getattr(py, "_v")?.extract(py)?;
                sample.try_get(SampleViewIdx::Var(pyvar.v))?
            }
        })
    }

    pub(super) fn __len__(&self) -> usize {
        self.sol.s.read_arc().sample_len()
    }

    fn __iter__(&self) -> PySampleIterator {
        PySampleIterator::new(self.clone())
    }

    // fn __str__(&self) -> PyResult<String> {
    //     unimplemented!()
    // }
}
