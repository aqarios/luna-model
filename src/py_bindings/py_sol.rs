use crate::core::{
    ConcreteAssignmentTypes, ConcreteBias, ConcreteIndex, IndexByValue, RcSolution, ResultIterator,
    ResultView, SampleIterator, VarAssignment,
};
use crate::py_bindings::py_timing::PyTiming;
use derive_more::{Deref, DerefMut};
use numpy::{PyArray1, ToPyArray};
use pyo3::exceptions::{PyIndexError, PyRuntimeError};
use pyo3::prelude::*;

#[derive(Deref, DerefMut)]
pub struct PyVarAssignment(VarAssignment<ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "ResultView", module = "aqmodels")]
#[derive(Deref, DerefMut, IntoPyObject)]
pub struct PyResultView(ResultView<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "Results", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PyResultIterator(ResultIterator<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "Sample", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PySampleIterator(SampleIterator<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "Solution", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PySolution(pub RcSolution<ConcreteBias, ConcreteAssignmentTypes>);

impl Into<ResultView<ConcreteBias, ConcreteAssignmentTypes>> for PyResultView {
    fn into(self) -> ResultView<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

impl Into<ResultIterator<ConcreteBias, ConcreteAssignmentTypes>> for PyResultIterator {
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
    pub fn iter<Idx>(&self) -> PyResultIterator {
        PyResultIterator(ResultIterator::new(RcSolution::clone(&self.0)))
    }
}

#[pymethods]
impl PySolution {
    #[getter]
    fn results<'a>(&self) -> PyResultIterator {
        self.iter::<ConcreteIndex>()
    }

    #[getter]
    fn obj_values<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<Option<ConcreteBias>>> {
        self.obj_values.to_pyarray(py)
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

    // TODO: implement human-readable solution representation
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResultIterator {
        slf.iter::<ConcreteIndex>()
    }

    fn __getitem__(&self, py: Python, index: PyObject) -> PyResult<PyObject> {
        if let Ok(res_idx) = index.extract::<usize>(py) {
            match self.get_result_view(res_idx) {
                None => Err(PyIndexError::new_err(format!(
                    "Index {res_idx} out of bounds"
                ))),
                Some(r) => PyResultView(r).into_pyobject(py),
            }
        } else if let Ok((res_idx, var_idx)) = index.extract::<(usize, usize)>(py) {
            match self.get_assignment(res_idx.into(), var_idx.into()) {
                None => Err(PyIndexError::new_err(format!(
                    "Index ({res_idx}, {var_idx}) out of bounds"
                ))),
                Some(v) => Ok(PyVarAssignment(v).into_pyobject(py)?.unbind()),
            }
        } else {
            Err(PyRuntimeError::new_err("unsupported type for indexing"))
        }
    }
}

#[pymethods]
impl PyResultView {
    #[getter]
    fn sample(&self) -> PySampleIterator {
        PySampleIterator(self.iter())
    }

    #[getter]
    fn obj_value(&self) -> Option<ConcreteBias> {
        self.0.obj_value()
    }

    #[getter]
    fn constraints<'a>(&self, py: Python<'a>) -> Option<Bound<'a, PyArray1<bool>>> {
        match &self.constraint_satisfaction() {
            None => None,
            Some(cs) => cs.map(|c| c.to_pyarray(py)),
        }
    }

    #[getter]
    fn feasible(&self) -> Option<bool> {
        self.0.feasible()
    }

    fn __getitem__(&self, py: Python, index: PyObject) -> PyResult<PyVarAssignment> {
        if let Ok(var_idx) = index.extract::<usize>(py) {
            match self.get_assignment(var_idx) {
                None => Err(PyIndexError::new_err(format!(
                    "Index {res_idx} out of bounds"
                ))),
                Some(r) => Ok(PyVarAssignment(r)),
            }
        } else {
            Err(PyRuntimeError::new_err("unsupported type for indexing"))
        }
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
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            VarAssignment::Binary(x) => Ok(x.into_py(py).into_bound(py)),
            VarAssignment::Spin(x) => Ok(x.into_py(py).into_bound(py)),
            VarAssignment::Integer(x) => Ok(x.into_py(py).into_bound(py)),
            VarAssignment::Real(x) => Ok(x.into_py(py).into_bound(py)),
        }
    }
}
