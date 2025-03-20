use super::py_model::PyModel;
use crate::{core::Vtype, translator::matrix_translator::MatrixTranslator};
use numpy::{PyArray2, PyArrayMethods, PyReadonlyArray2, PyUntypedArrayMethods, ToPyArray};
use pyo3::prelude::*;

#[pyclass(unsendable, name = "MatrixTranslator")]
pub struct PyMatrixTranslator {}

#[pymethods]
impl PyMatrixTranslator {
    #[staticmethod]
    #[pyo3(signature=(qubo, name=None, vtype=None))]
    fn to_model(
        qubo: PyReadonlyArray2<f64>,
        name: Option<String>,
        vtype: Option<Vtype>,
    ) -> PyModel {
        let dense = qubo.as_slice().expect("failed to convert to slice");
        PyModel(MatrixTranslator::model_from_dense(
            name,
            dense,
            qubo.shape()[0].into(),
            vtype.unwrap_or(Vtype::Binary),
        ))
    }

    #[staticmethod]
    #[pyo3(signature=(model))]
    fn to_dense<'a>(py: Python<'a>, model: &PyModel) -> PyResult<Bound<'a, PyArray2<f64>>> {
        let (vec, nvars) = MatrixTranslator::model_to_dense(&model.0)?;
        Ok(vec.to_pyarray(py).reshape((nvars, nvars))?)
    }
}
