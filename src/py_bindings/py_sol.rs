use crate::core::{Res, Runtime, Solution};
use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

#[pyclass(unsendable, name = "Runtime")]
#[derive(Deref, DerefMut)]
pub struct PyRuntime(pub Runtime);

#[pyclass(unsendable, name = "Result")]
#[derive(Deref, DerefMut)]
pub struct PyRes(pub Res<'static, f64, f64>);

#[pyclass(unsendable, name = "Solution")]
#[derive(Deref, DerefMut)]
pub struct PySolution(pub Solution<f64, f64>);

impl Into<Res<f64, f64>> for PyRes {
    fn into(self) -> Res<f64, f64> {
        self.0
    }
}

impl Into<Solution<f64, f64>> for PySolution {
    fn into(self) -> Solution<f64, f64> {
        self.0
    }
}

#[pymethods]
impl PySolution {
    #[getter]
    fn results(&self) -> Vec<PyRes> {
        self.iter().map(|r| PyRes(r)).collect()
    }
    #[getter]
    fn samples(&self) -> &Vec<Vec<f64>> {
        &self.samples
    }
    #[getter]
    fn obj_values(&self) -> &Vec<f64> {
        &self.obj_values
    }
    #[getter]
    fn num_occurrences(&self) -> &Vec<usize> {
        &self.num_occurrences
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
    fn sample(&self) -> &Vec<f64> {
        self.sample.as_ref()
    }
}
