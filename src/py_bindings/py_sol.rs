use crate::core::{ResultView, Solution};
use crate::py_bindings::py_timing::PyTiming;
use derive_more::{Deref, DerefMut};
use numpy::{PyArray1, PyArray2, ToPyArray};
use pyo3::prelude::*;

#[pyclass(unsendable, name = "Result")]
pub struct PyRes {
    sample: Py<PyArray1<f64>>,
    num_occurrences: usize,
    obj_value: Option<f64>,
    constraint_satisfaction: Option<Py<PyArray1<bool>>>,
    feasible: Option<bool>,
}

#[pyclass(unsendable, name = "Results")]
pub struct PyResults {
    solution: Solution<f64, f64>,
    current_index: usize,
}

#[pyclass(unsendable, name = "Solution")]
#[derive(Deref, DerefMut)]
pub struct PySolution(pub Solution<f64, f64>);

impl Into<Solution<f64, f64>> for PySolution {
    fn into(self) -> Solution<f64, f64> {
        self.0
    }
}

#[pymethods]
impl PySolution {
    #[getter]
    fn results<'a>(&self, py: Python<'a>) -> Vec<PyRes> {
        self.iter().map(|r| PyRes::from_res(r, py)).collect()
    }
    #[getter]
    fn samples<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray2<f64>> {
        PyArray2::from_vec2(py, &self.samples).unwrap()
    }
    #[getter]
    fn obj_values<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<f64>> {
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

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<PyResults>> {
        let res_iter = PyResults::new(slf.clone());
        Py::new(slf.py(), res_iter)
    }
}

impl PyRes {
    fn from_res<'a>(res: ResultView<'a, f64, f64>, py: Python<'a>) -> Self {
        let constr_sat = match res.constraint_satisfaction {
            None => None,
            Some(c) => Some(c.to_pyarray(py).unbind()),
        };
        Self {
            sample: res.sample.to_pyarray(py).unbind(),
            num_occurrences: res.num_occurrences,
            obj_value: res.obj_value,
            constraint_satisfaction: constr_sat,
            feasible: res.feasible,
        }
    }
}

impl PyResults {
    fn new(solution: Solution<f64, f64>) -> Self {
        Self {
            solution,
            current_index: 0,
        }
    }

    fn next(&mut self, py: Python) -> Option<PyRes> {
        let out = match self.solution.get_result(self.current_index) {
            None => None,
            Some(res) => Some(PyRes::from_res(res, py)),
        };
        self.current_index += 1;
        out
    }
}

#[pymethods]
impl PyRes {
    #[getter]
    fn sample(&self) -> &Py<PyArray1<f64>> {
        &self.sample
    }

    #[getter]
    fn num_occurrences(&self) -> usize {
        self.num_occurrences
    }

    #[getter]
    fn obj_value(&self) -> Option<f64> {
        self.obj_value
    }

    #[getter]
    fn constraint_satisfaction(&self) -> &Option<Py<PyArray1<bool>>> {
        &self.constraint_satisfaction
    }

    #[getter]
    fn feasible(&self) -> Option<bool> {
        self.feasible
    }
}

#[pymethods]
impl PyResults {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>, py: Python) -> Option<PyRes> {
        slf.next(py)
    }
}
