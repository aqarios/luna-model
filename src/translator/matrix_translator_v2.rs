use std::{u32, usize};

#[cfg(feature = "py")]
use numpy::PyReadonlyArray2;
use numpy::{PyArrayMethods, PyUntypedArrayMethods};
#[cfg(feature = "py")]
use pyo3::prelude::*;

use crate::core::QuadraticModelBase;

#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd)]
struct U32(u32);

impl Into<usize> for U32 {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for U32 {
    fn from(value: usize) -> Self {
        assert!(value <= u32::MAX as usize, "value out of range for u32");
        U32(value as u32)
    }
}

type QuadModel = QuadraticModelBase<f64, U32>;

#[cfg_attr(feature = "py", pyclass)]
pub struct PyQuadraticModel {
    inner: QuadModel,
}

impl PyQuadraticModel {
    fn default() -> Self {
        PyQuadraticModel {
            inner: QuadModel::default(),
        }
    }
}

#[cfg_attr(feature = "py", pyclass)]
pub struct MatrixTranslatorV2 {}

#[pymethods]
impl MatrixTranslatorV2 {
    #[staticmethod]
    #[pyo3(signature=(qubo, name=None))]
    fn to_model(qubo: PyReadonlyArray2<f64>, name: Option<String>) -> PyQuadraticModel {
        let dense = qubo.as_slice().expect("failed to convert to slice");
        let mut out = PyQuadraticModel::default();
        let num_variables = qubo.shape()[0].into();
        out.inner.resize(num_variables);
        out.inner.add_quadratic_from_dense(dense, num_variables);
        out
    }
}
