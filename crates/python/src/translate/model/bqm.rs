use lunamodel_types::{Bias, Vtype};
use numpy::PyReadonlyArray1;
use pyo3::{PyResult, pyclass, pymethods};

use crate::PyModel;

#[pyclass]
pub struct PyBqmTranslator;

#[pymethods]
impl PyBqmTranslator {
    #[staticmethod]
    fn to_lm(
        vars: Vec<String>,
        vtype: Vtype,
        offset: Bias,
        linears: PyReadonlyArray1<f64>,
        linear_indices: PyReadonlyArray1<u64>,
        quads: PyReadonlyArray1<f64>,
        quads_rows: PyReadonlyArray1<u64>,
        quads_cols: PyReadonlyArray1<u64>,
        name: Option<String>,
    ) -> PyResult<PyModel> {
        _ = vars;
        _ = vtype;
        _ = offset;
        _ = linears;
        _ = linear_indices;
        _ = quads;
        _ = quads_rows;
        _ = quads_cols;
        _ = name;

        todo!()
    }
}
