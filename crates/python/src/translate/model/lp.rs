//! Python wrapper for the LP translator.

use std::path::PathBuf;

use lunamodel_translate::model::LpTranslator;
use lunamodel_unwind::*;
use pyo3::{PyResult, pyclass, pymethods};

use super::{ModelInput, model_text};
use crate::{PyModel, args::PyModelArg};

#[pyclass]
pub struct PyLpTranslator;

#[unwindable]
#[pymethods]
impl PyLpTranslator {
    /// Translate an LP string or LP file into a LunaModel model; see [`model_text`].
    #[staticmethod]
    fn to_lm(file: ModelInput) -> PyResult<PyModel> {
        Ok(LpTranslator::translate(model_text(file)?)?.into())
    }

    /// Serialize a LunaModel model back to LP text or write it to a file.
    #[staticmethod]
    #[pyo3(signature=(model, filepath=None))]
    fn from_lm(model: PyModelArg, filepath: Option<PathBuf>) -> PyResult<Option<String>> {
        Ok(LpTranslator::back_translate(&model.m.read_arc(), filepath)?)
    }
}
