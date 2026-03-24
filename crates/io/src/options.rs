#[cfg(feature = "py")]
use pyo3::{FromPyObject, PyErr, exceptions::PyValueError};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PrintLayout {
    Row,
    Col,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ShowMetadata {
    Before,
    After,
    Hide,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PySolFormatOpts {
    pub layout: PrintLayout,
    pub max_line_len: usize,
    pub max_col_len: usize,
    pub max_lines: usize,
    pub max_var_name_len: usize,
    pub show_metadata: ShowMetadata,
}

impl Default for PySolFormatOpts {
    fn default() -> Self {
        Self {
            layout: PrintLayout::Col,
            max_line_len: 80,
            max_col_len: 5,
            max_lines: 10,
            max_var_name_len: 10,
            show_metadata: ShowMetadata::After,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FormatOpt {
    Rs,
    #[cfg(feature = "py")]
    Py,
    #[cfg(feature = "py")]
    PySol(PySolFormatOpts),
}

#[cfg(feature = "py")]
impl<'a, 'py> FromPyObject<'a, 'py> for PrintLayout {
    type Error = PyErr;
    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        let mode: &str = obj.extract()?;
        match mode {
            "row" => Ok(PrintLayout::Row),
            "column" => Ok(PrintLayout::Col),
            _ => Err(PyValueError::new_err(format!(
                "Invalid spec '{mode}'. Expected one of 'row', 'column'."
            ))),
        }
    }
}

#[cfg(feature = "py")]
impl<'a, 'py> FromPyObject<'a, 'py> for ShowMetadata {
    type Error = PyErr;
    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        let mode: &str = obj.extract()?;
        match mode {
            "before" => Ok(ShowMetadata::Before),
            "after" => Ok(ShowMetadata::After),
            "hide" => Ok(ShowMetadata::Hide),
            _ => Err(PyValueError::new_err(format!(
                "Invalid spec '{mode}'. Expected one of 'before', 'after', 'hide'."
            ))),
        }
    }
}
