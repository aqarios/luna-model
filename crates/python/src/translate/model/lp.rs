use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use lunamodel_error::LunaModelResult;
use lunamodel_translate::model::LpTranslator;
use pyo3::{FromPyObject, PyResult, pyclass, pymethods};

use crate::PyModel;

#[derive(FromPyObject)]
enum PyLpTranslatorToLmInput {
    Str(String),
    Buf(PathBuf),
}

#[pyclass]
pub struct PyLpTranslator;

#[pymethods]
impl PyLpTranslator {
    #[staticmethod]
    fn to_lm(file: PyLpTranslatorToLmInput) -> PyResult<PyModel> {
        let model = match file {
            PyLpTranslatorToLmInput::Str(raw) => LpTranslator::translate(raw)?,
            PyLpTranslatorToLmInput::Buf(buf) => LpTranslator::translate(read_buf(buf)?)?,
        };
        Ok(model.into())
    }

    #[staticmethod]
    fn from_lm(model: PyModel, filepath: Option<PathBuf>) -> PyResult<Option<String>> {
        Ok(LpTranslator::back_translate(&model.m.read_arc(), filepath)?)
    }
}

fn read_buf(buf: PathBuf) -> LunaModelResult<String> {
    let file = File::open(buf)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}
