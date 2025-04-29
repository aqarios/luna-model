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
    fn to_aq(py: Python, file: PyObject) -> PyResult<PyModel> {
        if let Ok(file) = file.extract::<String>(py) {
            // Here we need to help the user a bit. Let's check if we can make a PathBuf from this.
            // If not possible we try to read the string as is. And throw an error if both fails.
            // from the translation.
            let pathbuf = PathBuf::from(&file);
            let file = if pathbuf.exists() {
                // We have a real path. So we can call the translate on the pathbuf.
                LPTranslator::<ConcreteIndex, ConcreteBias>::read_file(pathbuf)?
            } else {
                // We have a string representing a model.
                // We don't need to do anything here.
                file
            };
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
    fn from_aq(model: &PyModel, filepath: Option<PathBuf>) -> PyResult<Option<String>> {
        Ok(LPTranslator::back_translate((&model.0, filepath))?)
    }
}
