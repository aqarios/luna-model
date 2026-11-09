//! Python wrappers for concrete model translators.
mod bqm;
mod lp;
mod mps;
mod qubo;

use std::{fs, io::ErrorKind, path::PathBuf};

use pyo3::{
    FromPyObject, PyResult,
    exceptions::{PyFileNotFoundError, PyOSError},
};

/// Python wrapper for the BQM translator.
pub use bqm::PyBqmTranslator;
/// Python wrapper for the LP translator.
pub use lp::PyLpTranslator;
/// Python wrapper for the MPS translator.
pub use mps::PyMpsTranslator;
/// Python wrappers for QUBO translation.
pub use qubo::{PyQubo, PyQuboTranslator};

/// What `to_lm` of the file translators accepts: model text or the path of a file.
#[derive(FromPyObject)]
enum ModelInput {
    Text(String),
    Path(PathBuf),
}

/// The model text: a string spanning several lines is the model itself, anything else names
/// a file. LP and MPS models always span several lines, so a single line can only be a path.
fn model_text(input: ModelInput) -> PyResult<String> {
    let path = match input {
        ModelInput::Text(text) if text.contains('\n') => return Ok(text),
        ModelInput::Text(text) => PathBuf::from(text),
        ModelInput::Path(path) => path,
    };
    fs::read_to_string(&path).map_err(|e| {
        let msg = format!("{}: {e}", path.display());
        match e.kind() {
            ErrorKind::NotFound => PyFileNotFoundError::new_err(msg),
            _ => PyOSError::new_err(msg),
        }
    })
}
