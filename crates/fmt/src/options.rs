//! Format-option enums used by custom display implementations.

#[cfg(feature = "py")]
use pyo3::{FromPyObject, PyErr, exceptions::PyValueError};

/// Layout options for pretty-printing solutions.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PrintLayout {
    /// Show one sample per row.
    Row,
    /// Show one variable per row.
    Col,
}

/// Placement of solution metadata in pretty-printed output.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ShowMetadata {
    /// Show metadata rows before assignments.
    Before,
    /// Show metadata rows after assignments.
    After,
    /// Omit metadata from the pretty-printed output.
    Hide,
}

/// Python-oriented pretty-print configuration for solutions.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PySolFormatOpts {
    /// Row/column layout strategy.
    pub layout: PrintLayout,
    /// Maximum rendered line length before truncation/wrapping.
    pub max_line_len: usize,
    /// Maximum width allocated to rendered column values.
    pub max_col_len: usize,
    /// Maximum number of displayed lines.
    pub max_lines: usize,
    /// Maximum displayed variable name width.
    pub max_var_name_len: usize,
    /// Whether metadata rows are shown and where.
    pub show_metadata: ShowMetadata,
}

impl Default for PySolFormatOpts {
    /// Returns the default Python-style pretty-print configuration.
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

/// Top-level formatting mode selector used by [`crate::CustomFormat`].
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FormatOpt {
    /// Rust-facing formatting.
    Rs,
    #[cfg(feature = "py")]
    /// Python-facing formatting.
    Py,
    #[cfg(feature = "py")]
    /// Python solution pretty-print formatting with explicit options.
    PySol(PySolFormatOpts),
}

#[cfg(feature = "py")]
impl<'a, 'py> FromPyObject<'a, 'py> for PrintLayout {
    type Error = PyErr;
    /// Parses a Python string into a [`PrintLayout`].
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
    /// Parses a Python string into a [`ShowMetadata`] value.
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
