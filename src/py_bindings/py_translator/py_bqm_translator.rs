use crate::core::Vtype;
use crate::py_bindings::py_model::PyModel;
use crate::translator::BqmTranslator;
use numpy::{PyReadonlyArray1, ToPyArray};
use pyo3::ffi::c_str;
use pyo3::prelude::*;

#[pyclass(unsendable, name = "BqmTranslator", module = "aqmodels.translator")]
pub struct PyBqmTranslator {}

#[pymethods]
impl PyBqmTranslator {
    #[staticmethod]
    #[pyo3(signature=(bqm, vars, vartype, name=None))]
    fn translate(
        py: Python,
        bqm: PyObject,
        vars: PyObject,
        vartype: String,
        name: Option<String>,
    ) -> PyResult<PyModel> {
        let vtype = if vartype == String::from("SPIN") {
            Vtype::Spin
        } else {
            Vtype::Binary
        };

        let linear_biases: PyReadonlyArray1<f64> = bqm.getattr(py, "linear_biases")?.extract(py)?;
        let quadratic = bqm.getattr(py, "quadratic")?;
        let quadratic_biases: PyReadonlyArray1<f64> =
            quadratic.getattr(py, "biases")?.extract(py)?;
        let col_indices: PyReadonlyArray1<i32> =
            quadratic.getattr(py, "col_indices")?.extract(py)?;
        let row_indices: PyReadonlyArray1<i32> =
            quadratic.getattr(py, "row_indices")?.extract(py)?;
        let offset = bqm.getattr(py, "offset")?;

        Ok(PyModel(BqmTranslator::model_from_bqm(
            vars.extract(py)?,
            vtype,
            offset.extract(py)?,
            linear_biases
                .as_slice()
                .expect("failed to convert to slice"),
            quadratic_biases
                .as_slice()
                .expect("failed to convert to slice"),
            col_indices.as_slice().expect("failed to convert to slice"),
            row_indices.as_slice().expect("failed to convert to slice"),
            name,
        )))
    }

    #[staticmethod]
    #[pyo3(signature=(model))]
    fn to_bqm<'a>(py: Python<'a>, model: &PyModel) -> PyResult<PyObject> {
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
        let args = (offset, linear_py, quadratic_py, rows_py, cols_py, vtype_py, vars_py);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }

    #[staticmethod]
    #[pyo3(signature=(bqm, name=None))]
    fn to_model(py: Python, bqm: PyObject, name: Option<PyObject>) -> PyResult<PyObject> {
        let extractor: PyObject = PyModule::from_code(
            py,
            c_str!(
                "
from dimod import BinaryQuadraticModel

from aqmodels._core import translator

def extract(bqm, name):
    if not isinstance(bqm, BinaryQuadraticModel):
        raise TypeError(f'Expected bqm to be of type BQM, received: {type(bqm)}')
    vars = bqm.variables.to_serializable()
    vartype = bqm.vartype.name
    bqm_np = bqm.to_numpy_vectors()
    return translator.BqmTranslator.translate(bqm_np, vars, vartype, name)"
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
