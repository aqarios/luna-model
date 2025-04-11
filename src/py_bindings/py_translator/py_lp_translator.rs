use crate::translator::base::BackTranslator;
use crate::translator::LPTranslator;
use crate::{py_bindings::py_model::PyModel, translator::base::Translator};
use pyo3::prelude::*;
use std::path::PathBuf;

#[pyclass(unsendable, name = "LpTranslator", module = "aqmodels.translator")]
pub struct PyLpTranslator {}

#[pymethods]
impl PyLpTranslator {
    #[staticmethod]
    #[pyo3(signature=(filepath))]
    fn to_model(filepath: PathBuf) -> PyResult<PyModel> {
        Ok(PyModel(LPTranslator::translate(filepath)?))
    }

    #[staticmethod]
    #[pyo3(signature=(model, filepath))]
    fn from_model(model: &PyModel, filepath: PathBuf) -> PyResult<()> {
        Ok(LPTranslator::back_translate((&model.0, filepath))?)
    }
}
