//! Python wrapper for the LP translator.

use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use lunamodel_error::LunaModelResult;
use lunamodel_translate::model::LpTranslator;
use lunamodel_unwind::*;
use pyo3::{FromPyObject, PyResult, pyclass, pymethods};

use crate::{PyModel, args::PyModelArg};

#[derive(FromPyObject)]
enum PyLpTranslatorToLmInput {
    /// Treat the string either as raw LP content or as a filesystem path.
    Str(String),
    /// Read LP content from the provided path.
    Buf(PathBuf),
}

#[pyclass]
pub struct PyLpTranslator;

#[unwindable]
#[pymethods]
impl PyLpTranslator {
    /// Translate an LP string or LP file into a LunaModel model.
    ///
    /// String inputs are interpreted as paths first when they resolve to an
    /// existing file. This keeps the Python API compact but means callers should
    /// pass explicit file-like objects when a model string could also name a path.
    #[staticmethod]
    fn to_lm(file: PyLpTranslatorToLmInput) -> PyResult<PyModel> {
        let model = match file {
            PyLpTranslatorToLmInput::Str(mayberaw) => {
                let pathbuf = PathBuf::from(&mayberaw);
                let file = if pathbuf.exists() {
                    // We have a real path. So we can call the translate on the pathbuf.
                    read_buf(pathbuf)?
                } else {
                    // We have a string representing a model.
                    // We don't need to do anything here.
                    mayberaw
                };
                LpTranslator::translate(file)?
            }
            PyLpTranslatorToLmInput::Buf(buf) => LpTranslator::translate(read_buf(buf)?)?,
        };
        Ok(model.into())
    }

    /// Serialize a LunaModel model back to LP text or write it to a file.
    #[staticmethod]
    #[pyo3(signature=(model, filepath=None))]
    fn from_lm(model: PyModelArg, filepath: Option<PathBuf>) -> PyResult<Option<String>> {
        Ok(LpTranslator::back_translate(&model.m.read_arc(), filepath)?)
    }
}

/// Read a UTF-8 text model from disk into memory for translator consumption.
fn read_buf(buf: PathBuf) -> LunaModelResult<String> {
    let file = File::open(buf)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}
