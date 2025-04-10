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
    fn to_model(filepath: PathBuf) -> PyModel {
        PyModel(LPTranslator::translate(filepath))
    }
}
