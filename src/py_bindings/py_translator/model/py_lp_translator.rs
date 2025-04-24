use crate::core::{ConcreteBias, ConcreteIndex};
use crate::translator::base::BackTranslator;
use crate::translator::LPTranslator;
use crate::{py_bindings::py_model::PyModel, translator::base::Translator};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::path::PathBuf;

#[pyclass(unsendable, name = "LpTranslator", module = "aqmodels.translator")]
pub struct PyLpTranslator {}

#[pymethods]
impl PyLpTranslator {
    #[staticmethod]
    #[pyo3(signature=(file))]
    fn to_model(py: Python, file: PyObject) -> PyResult<PyModel> {
        if let Ok(file) = file.extract::<String>(py) {
            Ok(PyModel(LPTranslator::translate(file)?))
        } else if let Ok(filepath) = file.extract::<PathBuf>(py) {
            let file = LPTranslator::<ConcreteIndex, ConcreteBias>::read_file(filepath)?;
            Ok(PyModel(LPTranslator::translate(file)?))
        } else {
            Err(PyRuntimeError::new_err(
                "file must be either a Path object or the LP String",
            ))
        }
    }

    #[staticmethod]
    #[pyo3(signature=(model, filepath=None))]
    fn from_model(model: &PyModel, filepath: Option<PathBuf>) -> PyResult<Option<String>> {
        Ok(LPTranslator::back_translate((&model.0, filepath))?)
    }
}
