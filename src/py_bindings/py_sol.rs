use crate::core::{
    ConcreteAssignmentTypes, ConcreteBias, RcSolution, Samples, Solution, VarAssignment,
};
use crate::py_bindings::py_res::{PyResultIterator, PyResultView};
use crate::py_bindings::py_sample::PySamples;
use crate::py_bindings::py_timing::PyTiming;
use derive_more::{Deref, DerefMut};
use numpy::{PyArray1, ToPyArray};
use pyo3::exceptions::{PyIndexError, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::IntoPyObjectExt;
use std::rc::Rc;

#[derive(Deref, DerefMut)]
pub struct PyVarAssignment(pub VarAssignment<ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "Solution", module = "aqmodels")]
#[derive(Deref, DerefMut, Debug)]
pub struct PySolution(pub RcSolution<ConcreteBias, ConcreteAssignmentTypes>);

impl Into<RcSolution<ConcreteBias, ConcreteAssignmentTypes>> for PySolution {
    fn into(self) -> RcSolution<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

#[pymethods]
impl PySolution {
    #[getter]
    fn results<'a>(&self) -> PyResultIterator {
        PyResultIterator(self.0.iter_results())
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
        Ok(PySolution(RcSolution(Rc::new(Solution::default()))))
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
        PyResultIterator(slf.0.iter_results())
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
