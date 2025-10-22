use crate::py_bindings::unwind;
use crate::translator::base::BackTranslator;
use crate::translator::LPTranslator;
use crate::{py_bindings::py_model::PyModel, translator::base::Translator};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use std::path::PathBuf;
use unwind_macros::unwindable;

/// Utility class for converting between LP files and symbolic models.
///
/// `LpTranslator` provides methods to:
/// - Convert an LP file into a symbolic `Model`
/// - Convert a `Model` into an Lp file.
///
/// These conversions are especially useful when interacting with external solvers
/// or libraries that operate on LP-based problem definitions.
///
/// Examples
/// --------
/// >>> from pathlib import Path
/// >>> from luna_model import LpTranslator
/// >>> lp_filepath = Path("path/to/the/lp_file")
///
/// >>> model = LpTranslator.to_aq(lp_filepath)
///
/// Convert it back to an LP file:
///
/// >>> recovered = LpTranslator.to_file(model)
#[pyclass(name = "LpTranslator", module = "luna_model._core.translator")]
pub struct PyLpTranslator {}

#[unwindable]
#[pymethods]
impl PyLpTranslator {
    /// Convert an LP file into a symbolic `Model`.
    ///
    /// Parameters
    /// ----------
    /// file: Path | String
    ///     An LP file representing a symbolic model, either given as a
    ///     Path object to the LP file or its contents as a string.
    ///     If you pass the path as a string, it will be interpreted as a
    ///     model and thus fail to be parsed to a Model.
    ///
    /// Returns
    /// -------
    /// Model
    ///     A symbolic model representing the given lp file structure.
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If `file` is not of type `str` or `Path`.
    /// TranslationError
    ///     If the translation fails for a different reason.
    #[staticmethod]
    #[pyo3(signature=(file))]
    fn to_aq(py: Python, file: Py<PyAny>) -> PyResult<PyModel> {
        if let Ok(file) = file.extract::<String>(py) {
            // Here we need to help the user a bit. Let's check if we can make a PathBuf from this.
            // If not possible we try to read the string as is. And throw an error if both fails.
            // from the translation.
            let pathbuf = PathBuf::from(&file);
            let file = if pathbuf.exists() {
                // We have a real path. So we can call the translate on the pathbuf.
                LPTranslator::read_file(pathbuf)?
            } else {
                // We have a string representing a model.
                // We don't need to do anything here.
                file
            };
            Ok(PyModel::new(LPTranslator::translate(file)?))
        } else if let Ok(filepath) = file.extract::<PathBuf>(py) {
            let file = LPTranslator::read_file(filepath)?;
            Ok(PyModel::new(LPTranslator::translate(file)?))
        } else {
            Err(PyTypeError::new_err(
                "file must be either a Path object or the LP String",
            ))
        }
    }

    /// Convert a symbolic model to an LP file representation.
    ///
    /// Parameters
    /// ----------
    /// model : Model
    ///     The symbolic model to convert.
    /// file : Path, optional
    ///     The filepath to write the model contents to.
    ///
    /// Returns
    /// -------
    /// str
    ///     If no file to write to is given, i.e., the file is None.
    ///
    /// Raises
    /// ------
    /// TranslationError
    ///     If the translation fails for some reason.
    #[staticmethod]
    #[pyo3(signature=(model, filepath=None))]
    fn from_aq(model: &PyModel, filepath: Option<PathBuf>) -> PyResult<Option<String>> {
        Ok(LPTranslator::back_translate((&model.access(), filepath))?)
    }
}
