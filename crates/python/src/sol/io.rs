use lunamodel_io::{
    CustomFormat, FormatOpt,
    sol::{PrintLayout, PySolFormatOpts, ShowMetadata},
};
use pyo3::{PyResult, exceptions::PyValueError, pymethods};

use crate::utils::PyUsize;

use super::PySolution;

#[pymethods]
impl PySolution {
    fn print(
        &self,
        layout: PrintLayout,
        max_line_len: PyUsize,
        max_col_len: PyUsize,
        max_lines: PyUsize,
        max_var_name_len: PyUsize,
        show_metadata: ShowMetadata,
    ) -> PyResult<String> {
        let max_line_len: usize = max_line_len.into();
        let max_col_len: usize = max_col_len.into();
        let max_lines: usize = max_lines.into();
        let max_var_name_len: usize = max_var_name_len.into();

        eprintln!("IN PRINT");
        if max_line_len < 5 {
            Err(PyValueError::new_err(format!(
                "`max_line_len needs` to be at least 5; actual value: {max_line_len}"
            )))
        } else if max_col_len < 1 {
            Err(PyValueError::new_err(format!(
                "`max_col_len` needs to be at least 1; actual value: {max_col_len}"
            )))
        } else if max_lines < 1 {
            Err(PyValueError::new_err(format!(
                "`max_lines` needs to be at least 1; actual value: {max_lines}"
            )))
        } else if max_var_name_len < 1 {
            Err(PyValueError::new_err(format!(
                "`max_var_name_len` needs to be at least 1; actual value: {max_var_name_len}"
            )))
        } else {
            Ok(format!(
                "{}",
                self.s.read_arc().format(FormatOpt::PySol(PySolFormatOpts {
                    layout,
                    max_line_len,
                    max_col_len,
                    max_lines,
                    max_var_name_len,
                    show_metadata
                }))
            ))
        }
    }

    fn __str__(&self) -> String {
        format!("{}", self.s.read_arc().format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.s.read_arc().format(FormatOpt::Py))
    }
}
