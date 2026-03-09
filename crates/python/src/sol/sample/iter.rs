use itertools::Itertools;
use lunamodel_core::Solution;
use lunamodel_core::solution::Assignment;
use lunamodel_unwind::*;
use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::{
    FromPyObject, IntoPyObjectExt, Py, PyAny, PyErr, PyRef, PyRefMut, PyResult, Python, pyclass,
    pymethods,
};

use super::view::PySampleView;
use crate::sol::PySolution;
use crate::sol::sample::view::PySampleIndex;

pub enum PySamplesIndex {
    Sample(usize),
    Assignment((usize, usize)),
}

impl<'a, 'py> FromPyObject<'a, 'py> for PySamplesIndex {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        let simple_py_usize: Result<i128, PyErr> = obj.extract();
        if let Ok(n) = simple_py_usize {
            if n < 0 {
                Err(PyValueError::new_err(format!(
                    "Expected a non-negative number, received: {n}"
                )))
            } else {
                Ok(PySamplesIndex::Sample(n as usize))
            }
        } else {
            let (r, c): (i128, i128) = obj.extract()?;
            if r < 0 {
                Err(PyValueError::new_err(format!(
                    "Expected a non-negative number, received: {r}"
                )))
            } else if c < 0 {
                Err(PyValueError::new_err(format!(
                    "Expected a non-negative number, received: {c}"
                )))
            } else {
                Ok(PySamplesIndex::Assignment((r as usize, c as usize)))
            }
        }
    }
}

#[pyclass]
pub struct PySamplesIterator {
    sol: PySolution,
    idx: usize,
}

impl PySamplesIterator {
    pub fn new(sol: PySolution) -> Self {
        Self { sol, idx: 0 }
    }
}

#[unwindable]
#[pymethods]
impl PySamplesIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PySampleView> {
        let binding: &Solution = &slf.sol.s.read_arc();
        let res = binding.result(slf.idx);
        let out = res.map(|_| PySampleView::new(slf.sol.clone(), slf.idx));
        slf.idx += 1;
        out
    }

    fn tolist(slf: PyRef<'_, Self>, py: Python) -> PyResult<Vec<Vec<Py<PyAny>>>> {
        let mut out = Vec::new();
        for sample in slf.sol.s.read_arc().samples() {
            out.push(
                sample
                    .to_vec()
                    .iter()
                    .map(|e| match e {
                        Assignment::Binary(b) => b.into_py_any(py),
                        Assignment::Spin(s) => s.into_py_any(py),
                        Assignment::Integer(i) => i.into_py_any(py),
                        Assignment::Real(r) => r.into_py_any(py),
                    })
                    .collect::<Result<_, _>>()?,
            );
        }
        Ok(out)
    }

    fn __getitem__(slf: PyRef<'_, Self>, py: Python, index: PySamplesIndex) -> PyResult<Py<PyAny>> {
        let n_samples = slf.sol.s.read_arc().len();
        let sample_len = slf.sol.s.read_arc().sample_len();
        match index {
            PySamplesIndex::Sample(index) => {
                if index >= n_samples {
                    return Err(PyIndexError::new_err(format!(
                        "index '{index}' out of bounds for '{n_samples}' num samples"
                    )));
                }
                PySampleView::new(slf.sol.clone(), index.into()).into_py_any(py)
            }
            PySamplesIndex::Assignment((row, col)) => {
                if row >= n_samples || col >= sample_len {
                    return Err(PyIndexError::new_err(format!(
                        "index '({row}, {col})' out of bounds for '({n_samples}, {sample_len})' (num samples, sample len)"
                    )));
                }
                let row: usize = row.into();
                let col: usize = col.into();
                slf.sol.s.read_arc()[(row, col)].into_py_any(py)
            }
        }
    }

    fn __len__(slf: PyRef<'_, Self>) -> usize {
        slf.sol.s.read_arc().len()
    }

    fn __str__(slf: PyRef<'_, Self>) -> String {
        let sol = &slf.sol.s.read_arc();
        format!(
            "{{\n{}\n}}",
            sol.samples()
                .zip(&sol.counts)
                .map(|(sample, cnts)| format!("  {}: {}", sample.to_string(), cnts))
                .join(",\n")
        )
    }
}

#[pyclass]
pub struct PySampleIterator {
    sample: PySampleView,
    idx: usize,
}

impl PySampleIterator {
    pub fn new(sample: PySampleView) -> Self {
        Self { sample, idx: 0 }
    }
}

#[unwindable]
#[pymethods]
impl PySampleIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>, py: Python) -> Option<Py<PyAny>> {
        let res = slf.sample.__getitem__(py, PySampleIndex::Num(slf.idx)).ok();
        slf.idx += 1;
        res
    }

    fn __len__(slf: PyRef<'_, Self>) -> usize {
        slf.sample.sol.s.read_arc().sample_len()
    }
}
