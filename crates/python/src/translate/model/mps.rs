//! Python wrapper for the MPS translator.

use std::path::PathBuf;

use lunamodel_translate::model::MpsTranslator;
use lunamodel_unwind::*;
use pyo3::{PyResult, pyclass, pymethods};

use super::{ModelInput, model_text};
use crate::{PyModel, args::PyModelArg};

#[pyclass]
pub struct PyMpsTranslator;

#[unwindable]
#[pymethods]
impl PyMpsTranslator {
    /// Translate an MPS string or MPS file into a LunaModel model; see [`model_text`].
    #[staticmethod]
    fn to_lm(file: ModelInput) -> PyResult<PyModel> {
        Ok(MpsTranslator::translate(model_text(file)?)?.into())
    }

    /// Serialize a LunaModel model back to MPS text or write it to a file.
    #[staticmethod]
    #[pyo3(signature=(model, filepath=None))]
    fn from_lm(model: PyModelArg, filepath: Option<PathBuf>) -> PyResult<Option<String>> {
        Ok(MpsTranslator::back_translate(
            &model.m.read_arc(),
            filepath,
        )?)
    }
}
