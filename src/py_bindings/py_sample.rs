use crate::core::{
    ConcreteAssignmentTypes, ConcreteBias, RcSolution, ResultIterator, Sample, SampleIterator,
    Samples, SamplesIterator,
};
use crate::py_bindings::py_sol::PyVarAssignment;
use derive_more::{Deref, DerefMut};
use either::Either;
use pyo3::exceptions::{PyIndexError, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::IntoPyObjectExt;

#[pyclass(unsendable, name = "SamplesIterator", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PySamplesIterator(pub SamplesIterator<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "SampleIterator", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PySampleIterator(pub SampleIterator<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "Samples", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PySamples(pub Samples<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "Sample", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PySample(pub Sample<ConcreteBias, ConcreteAssignmentTypes>);

impl Into<SamplesIterator<ConcreteBias, ConcreteAssignmentTypes>> for PySamplesIterator {
    fn into(self) -> SamplesIterator<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

impl Into<SampleIterator<ConcreteBias, ConcreteAssignmentTypes>> for PySampleIterator {
    fn into(self) -> SampleIterator<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

#[pymethods]
impl PySamples {
    fn tolist(&self, py: Python) -> Vec<Vec<PyObject>> {
        ResultIterator::new(RcSolution::clone(&self))
            .into_iter()
            .map(|r| {
                SampleIterator::from_res_view(&r)
                    .into_iter()
                    .map(|v| PyVarAssignment(v).into_pyobject(py).unwrap().unbind())
                    .collect()
            })
            .collect()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __getitem__(&self, py: Python, index: PyObject) -> PyResult<PyObject> {
        if let Ok(res_idx) = index.extract::<usize>(py) {
            match self.get_sample(res_idx) {
                None => Err(PyIndexError::new_err(format!(
                    "Index {res_idx} out of bounds"
                ))),
                Some(r) => PySample(r).into_pyobject(py)?.into_py_any(py),
            }
        } else if let Ok((res_idx, var_idx)) = index.extract::<(usize, usize)>(py) {
            match self.get_assignment(res_idx, var_idx) {
                None => Err(PyIndexError::new_err(format!(
                    "Index ({res_idx}, {var_idx}) out of bounds"
                ))),
                Some(v) => Ok(PyVarAssignment(v).into_pyobject(py)?.unbind()),
            }
        } else {
            Err(PyRuntimeError::new_err("unsupported type for indexing"))
        }
    }

    fn __len__(&self) -> usize {
        self.n_samples
    }

    fn __iter__(&self) -> PySamplesIterator {
        PySamplesIterator(self.0.iter())
    }
}

#[pymethods]
impl PySample {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __getitem__(&self, py: Python, index: PyObject) -> PyResult<PyVarAssignment> {
        if let Ok(var_idx) = index.extract::<usize>(py) {
            match self.get_assignment(var_idx) {
                None => Err(PyIndexError::new_err(format!(
                    "Index {var_idx} out of bounds"
                ))),
                Some(v) => Ok(PyVarAssignment(v)),
            }
        } else {
            Err(PyRuntimeError::new_err("unsupported type for indexing"))
        }
    }

    fn __len__(&self) -> usize {
        match &self.0.0 {
            Either::Left(r) => r.sol.samples.len(),
            Either::Right(r) => r.len(),
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PySampleIterator {
        PySampleIterator(slf.0.iter())
    }
}

#[pymethods]
impl PySampleIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyVarAssignment> {
        slf.next().map(|res| PyVarAssignment(res))
    }
}

#[pymethods]
impl PySamplesIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PySample> {
        slf.next().map(|s| PySample(s))
    }
}
