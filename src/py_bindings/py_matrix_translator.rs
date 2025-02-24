use numpy::{PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::prelude::*;

use super::py_model::PyModel;
use crate::{core::Vtype, translator::matrix_translator::MatrixTranslator};

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
}
