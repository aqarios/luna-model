use crate::core::{Runtime, Solution};
use derive_more::{Deref, DerefMut};
use numpy::{PyArray1, PyArray2, ToPyArray};
use pyo3::prelude::*;

#[pyclass(unsendable, name = "Runtime")]
#[derive(Deref, DerefMut)]
pub struct PyRuntime(pub Runtime);

#[pyclass(unsendable, name = "Result")]
pub struct PyRes {
    sample: Py<PyArray1<f64>>,
    num_occurrences: usize,
    obj_value: Option<f64>,
    constraint_satisfaction: Option<Py<PyArray1<bool>>>,
    feasible: Option<bool>,
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
        self.iter()
            .map(|r| {
                let constr_sat = match r.constraint_satisfaction {
                    None => None,
                    Some(c) => Some(c.to_pyarray(py).unbind()),
                };
                PyRes {
                    sample: r.sample.to_pyarray(py).unbind(),
                    num_occurrences: r.num_occurrences,
                    obj_value: r.obj_value,
                    constraint_satisfaction: constr_sat,
                    feasible: r.feasible,
                }
            })
            .collect()
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

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
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
