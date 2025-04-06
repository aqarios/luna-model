use crate::core::{ConcreteAssignmentTypes, ConcreteBias, OwnedResult, RcSolution, ResultIterator, ResultView, Sample, SampleIterator, Samples, Solution, VarAssignment};
use crate::py_bindings::py_timing::PyTiming;
use derive_more::{Deref, DerefMut};
use either::{Either, Left};
use numpy::{PyArray1, ToPyArray};
use pyo3::exceptions::{PyIndexError, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::IntoPyObjectExt;
use std::rc::Rc;

#[derive(Deref, DerefMut)]
pub struct PyVarAssignment(VarAssignment<ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "ResultView", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PyResultView(ResultView<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "Result", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PyOwnedResult(OwnedResult<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "ResultIterator", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PyResultIterator(ResultIterator<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "SamplesIterator", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PySamplesIterator(ResultIterator<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "SampleIterator", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PySampleIterator(SampleIterator<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "Samples", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PySamples(Samples<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "Sample", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PySample(Sample<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "Solution", module = "aqmodels")]
#[derive(Deref, DerefMut, Debug)]
pub struct PySolution(pub RcSolution<ConcreteBias, ConcreteAssignmentTypes>);

impl Into<ResultView<ConcreteBias, ConcreteAssignmentTypes>> for PyResultView {
    fn into(self) -> ResultView<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

impl Into<OwnedResult<ConcreteBias, ConcreteAssignmentTypes>> for PyOwnedResult {
    fn into(self) -> OwnedResult<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

impl Into<ResultIterator<ConcreteBias, ConcreteAssignmentTypes>> for PyResultIterator {
    fn into(self) -> ResultIterator<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

impl Into<ResultIterator<ConcreteBias, ConcreteAssignmentTypes>> for PySamplesIterator {
    fn into(self) -> ResultIterator<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

impl Into<SampleIterator<ConcreteBias, ConcreteAssignmentTypes>> for PySampleIterator {
    fn into(self) -> SampleIterator<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

impl Into<RcSolution<ConcreteBias, ConcreteAssignmentTypes>> for PySolution {
    fn into(self) -> RcSolution<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

impl PySolution {
    pub fn iter(&self) -> PyResultIterator {
        PyResultIterator(ResultIterator::new(RcSolution::clone(&self.0)))
    }
}

impl PySample {
    pub fn iter(&self) -> PySampleIterator {
        match &self.0.0 {
            Either::Left(r) => PySampleIterator(SampleIterator::from_res_view(&r)),
            Either::Right(r) => PySampleIterator(SampleIterator::from_sample_vec(Rc::clone(r))),
        }
    }
}

#[pymethods]
impl PySolution {
    #[getter]
    fn results<'a>(&self) -> PyResultIterator {
        self.iter()
    }

    #[getter]
    fn samples(&self) -> PySamples {
        PySamples(Samples(RcSolution::clone(&self)))
    }

    #[getter]
    fn obj_values<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<PyObject>> {
        self.obj_values
            .iter()
            .map(|x| x.into_py_any(py).unwrap())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn raw_energies<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<PyObject>> {
        self.raw_energies
            .iter()
            .map(|x| x.into_py_any(py).unwrap())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn num_occurrences<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<usize>> {
        self.num_occurrences.to_pyarray(py)
    }

    #[getter]
    fn runtime(&self) -> Option<PyTiming> {
        self.timing.map(|t| PyTiming(t))
    }

    #[getter]
    fn best_sample_idx(&self) -> Option<usize> {
        self.0.best_sample_idx
    }

    #[pyo3(signature=(compress=None, level=None))]
    fn encode(&self, py: Python, compress: Option<bool>, level: Option<i32>) -> PyResult<PyObject> {
        let _compress = compress.unwrap_or(level.is_some());
        // TODO: implement actual compression logic then update this method
        Ok(PyBytes::new(py, &Vec::new().as_slice()).into())
    }

    #[pyo3(signature=(compress=None, level=None))]
    fn serialize(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<PyObject> {
        self.encode(py, compress, level)
    }

    #[staticmethod]
    fn decode(_py: Python, _data: Py<PyBytes>) -> PyResult<Self> {
        // TODO: implement actual compression logic then update this method
        Ok(PySolution(
            RcSolution(Rc::new(Solution::default()))
        ))
    }

    #[staticmethod]
    fn deserialize(py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Self::decode(py, data)
    }

    // TODO: implement human-readable solution representation
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResultIterator {
        slf.iter()
    }

    fn __getitem__(&self, py: Python, index: PyObject) -> PyResult<PyResultView> {
        if let Ok(res_idx) = index.extract::<usize>(py) {
            match self.get_result_view(res_idx) {
                None => Err(PyIndexError::new_err(format!(
                    "Index {res_idx} out of bounds"
                ))),
                Some(r) => Ok(PyResultView(r)),
            }
        } else {
            Err(PyRuntimeError::new_err("unsupported type for indexing"))
        }
    }
}

#[pymethods]
impl PyResultView {
    // TODO: implement human-readable solution representation
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    #[getter]
    fn sample(&self) -> PySample {
        PySample(self.get_sample())
    }

    #[getter]
    fn num_occurrences(&self) -> usize {
        self.sol.num_occurrences[self.row_idx]
    }

    #[getter]
    fn obj_value(&self) -> Option<ConcreteBias> {
        self.0.obj_value()
    }

    #[getter]
    fn raw_energy(&self) -> Option<ConcreteBias> {
        self.0.raw_energy()
    }

    #[getter]
    fn constraints<'a>(&self, py: Python<'a>) -> Option<Bound<'a, PyArray1<bool>>> {
        self.constraint_satisfaction()
            .as_ref()
            .map(|c| c.to_pyarray(py))
    }

    #[getter]
    fn feasible(&self) -> Option<bool> {
        self.0.feasible()
    }
}

#[pymethods]
impl PyOwnedResult {
    #[getter]
    fn sample(&self) -> PySample {
        PySample(self.get_sample())
    }

    #[getter]
    fn obj_value(&self) -> Option<ConcreteBias> {
        self.obj_value
    }

    #[getter]
    fn constraints<'a>(&self, py: Python<'a>) -> Option<Bound<'a, PyArray1<bool>>> {
        self.constraint_satisfaction
            .as_ref()
            .map(|c| c.to_pyarray(py))
    }

    #[getter]
    fn feasible(&self) -> Option<bool> {
        self.feasible
    }

    // TODO: implement human-readable solution representation
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
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

    // TODO: implement human-readable solution representation
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
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
        PySamplesIterator(ResultIterator::new(RcSolution::clone(&self)))
    }
}

#[pymethods]
impl PySample {
    // TODO: implement human-readable solution representation
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
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
            Left(r) => r.sol.samples.len(),
            Either::Right(r) => r.len(),
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PySampleIterator {
        slf.iter()
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
        slf.next().map(|res| PySample(Sample(Left(res.clone()))))
    }
}

#[pymethods]
impl PyResultIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyResultView> {
        slf.next().map(|res| PyResultView(res))
    }
}

impl<'py> IntoPyObject<'py> for PyVarAssignment {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            VarAssignment::Binary(x) => Ok(x.into_py_any(py)?.into_bound(py)),
            VarAssignment::Spin(x) => Ok(x.into_py_any(py)?.into_bound(py)),
            VarAssignment::Integer(x) => Ok(x.into_py_any(py)?.into_bound(py)),
            VarAssignment::Real(x) => Ok(x.into_py_any(py)?.into_bound(py)),
        }
    }
}
