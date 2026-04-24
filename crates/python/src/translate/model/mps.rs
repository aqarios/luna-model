//! Python wrapper for the MPS translator.

use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use lunamodel_error::LunaModelResult;
use lunamodel_translate::model::MpsTranslator;
use lunamodel_unwind::*;
use pyo3::{FromPyObject, PyResult, pyclass, pymethods};

use crate::{PyModel, args::PyModelArg};

#[derive(FromPyObject)]
enum PyMpsTranslatorToLmInput {
    /// Treat the string either as raw MPS content or as a filesystem path.
    Str(String),
    /// Read MPS content from the provided path.
    Buf(PathBuf),
}

#[pyclass]
pub struct PyMpsTranslator;

#[unwindable]
#[pymethods]
impl PyMpsTranslator {
    /// Translate an MPS string or file into a LunaModel model.
    #[staticmethod]
    fn to_lm(file: PyMpsTranslatorToLmInput) -> PyResult<PyModel> {
        let model = match file {
            PyMpsTranslatorToLmInput::Str(mayberaw) => {
                let pathbuf = PathBuf::from(&mayberaw);
                let file = if pathbuf.exists() {
                    // We have a real path. So we can call the translate on the pathbuf.
                    read_buf(pathbuf)?
                } else {
                    // We have a string representing a model.
                    // We don't need to do anything here.
                    mayberaw
                };
                MpsTranslator::translate(file)?
            }
            PyMpsTranslatorToLmInput::Buf(buf) => MpsTranslator::translate(read_buf(buf)?)?,
        };
        Ok(model.into())
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

/// Read an MPS file into memory before handing it to the Rust translator.
fn read_buf(buf: PathBuf) -> LunaModelResult<String> {
    let file = File::open(buf)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}
