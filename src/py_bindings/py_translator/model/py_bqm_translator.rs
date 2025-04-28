use crate::core::Vtype;
use crate::py_bindings::py_model::PyModel;
use crate::translator::model::BqmTranslator;
use numpy::{PyReadonlyArray1, ToPyArray};
use pyo3::ffi::c_str;
use pyo3::prelude::*;

#[pyclass(unsendable, name = "BqmTranslator", module = "aqmodels.translator")]
pub struct PyBqmTranslator {}

#[pymethods]
impl PyBqmTranslator {
    #[staticmethod]
    #[pyo3(signature=(vars, offset, linears, linear_indices, quads, quads_rows, quads_cols, vartype, name=None))]
    fn translate(
        py: Python,
        vars: PyObject,
        offset: f64,
        linears: PyReadonlyArray1<f64>,
        linear_indices: PyReadonlyArray1<u64>,
        quads: PyReadonlyArray1<f64>,
        quads_rows: PyReadonlyArray1<u64>,
        quads_cols: PyReadonlyArray1<u64>,
        vartype: String,
        name: Option<String>,
    ) -> PyResult<PyModel> {
        let vtype = if vartype == String::from("SPIN") {
            Vtype::Spin
        } else {
            Vtype::Binary
        };

        Ok(PyModel(BqmTranslator::model_from_bqm(
            vars.extract(py)?,
            vtype,
            offset,
            linears.as_slice().expect("failed to convert to slice"),
            linear_indices
                .as_slice()
                .expect("failed to convert to slice"),
            quads.as_slice().expect("failed to convert to slice"),
            quads_rows.as_slice().expect("failed to convert to slice"),
            quads_cols.as_slice().expect("failed to convert to slice"),
            name,
        )))
    }

    #[staticmethod]
    #[pyo3(signature=(model))]
    fn from_aq<'a>(py: Python<'a>, model: &PyModel) -> PyResult<PyObject> {
        let (offset, linear, quad, rows, cols, vtype, vars) =
            BqmTranslator::model_to_bqm(&model.0)?;
        let linear_py = linear.to_pyarray(py);
        let quadratic_py = quad.to_pyarray(py);
        let rows_py = rows.to_pyarray(py);
        let cols_py = cols.to_pyarray(py);
        let vtype_py = vtype.unwrap().to_string();
        let vars_py = vars.into_pyobject(py)?;

        let extractor: PyObject = PyModule::from_code(
            py,
            c_str!(
                "
from dimod import BinaryQuadraticModel

def to_bqm(offset, linear, quad, rows, cols, vtype, vars):
    vartype = vtype.upper()
    bqm = BinaryQuadraticModel.from_numpy_vectors(
        linear, (rows, cols, quad), offset, vartype, variable_order=vars
    )
    return bqm"
            ),
            c_str!(""),
            c_str!(""),
        )?
        .getattr("to_bqm")?
        .into();
        let args = (
            offset,
            linear_py,
            quadratic_py,
            rows_py,
            cols_py,
            vtype_py,
            vars_py,
        );
        let result = extractor.call1(py, args)?;
        Ok(result)
    }

    #[staticmethod]
    #[pyo3(signature=(bqm, name=None))]
    fn to_aq(py: Python, bqm: PyObject, name: Option<PyObject>) -> PyResult<PyObject> {
        let extractor: PyObject = PyModule::from_code(
            py,
            c_str!(
                "
import numpy as np
from dimod import BinaryQuadraticModel

from aqmodels._core import translator

def extract(bqm, name):
    if not isinstance(bqm, BinaryQuadraticModel):
        raise TypeError(f'Expected bqm to be of type BQM, received: {type(bqm)}')
    vars = np.array(bqm.variables.to_serializable())
    linears = np.array([bqm.get_linear(v) for v in vars])
    linear_indices, linears = tuple(zip(*[(i, v) for i, v in enumerate(linears) if v != 0]))
    intermediate = [
        (ui, vi, bqm.get_quadratic(vars[ui], vars[vi], default=0))
        for ui in range(len(vars))
        for vi in range(ui + 1, len(vars))
        if bqm.get_quadratic(vars[ui], vars[vi], default=0) != 0
    ]
    quads_rows, quads_cols, quads = tuple(zip(*intermediate)) if len(intermediate) > 0 else (np.array([]), np.array([]), np.array([]))
    vartype = bqm.vartype.name
    offset = float(bqm.offset)
    return translator.BqmTranslator.translate(
        vars, 
        offset, 
        np.array(linears, dtype=np.float64), 
        np.array(linear_indices, dtype=np.uint64), 
        np.array(quads, dtype=np.float64), 
        np.array(quads_rows, dtype=np.uint64), 
        np.array(quads_cols, dtype=np.uint64), 
        vartype, 
        name
    )"
            ),
            c_str!(""),
            c_str!(""),
        )?
        .getattr("extract")?
        .into();
        let args = (bqm, name);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
