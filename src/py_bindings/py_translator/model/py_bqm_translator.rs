use crate::core::Vtype;
use crate::py_bindings::py_model::PyModel;
use crate::py_bindings::unwind;
use crate::translator::model::BqmTranslator;
use numpy::{PyReadonlyArray1, ToPyArray};
use pyo3::exceptions::PyTypeError;
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use std::ffi::CStr;
use unwind_macros::unwindable;

#[cfg(not(feature = "lq"))]
static PY_CODE: &'static CStr = c_str!(
    "
import numpy as np
from dimod import BinaryQuadraticModel

from aqmodels._core import translator

def extract(bqm, name):
    if not isinstance(bqm, BinaryQuadraticModel):
        raise TypeError(f'Expected bqm to be of type BQM, received: {type(bqm)}')
    bqm_vars_ser = bqm.variables.to_serializable()
    for v in bqm_vars_ser:
        if not isinstance(v, str):
            raise TypeError(f'All BQM variables have to be of type str, received: {type(v)}')
    vars = np.array(bqm_vars_ser)
    vars_pos = {var: i for i, var in enumerate(vars)}

    linears = []
    linear_indices = []
    for var, val in bqm.linear.items():
        linears.append(val)
        linear_indices.append(vars_pos[var])
    quads = []
    quad_row = []
    quad_col = []
    for (var1, var2), val in bqm.quadratic.items():
        quads.append(val)
        quad_row.append(vars_pos[var1])
        quad_col.append(vars_pos[var2])

    vartype = bqm.vartype.name
    offset = float(bqm.offset)
    return translator.BqmTranslator.translate(
        vars,
        offset,
        np.array(linears, dtype=np.float64),
        np.array(linear_indices, dtype=np.uint64),
        np.array(quads, dtype=np.float64),
        np.array(quad_row, dtype=np.uint64),
        np.array(quad_col, dtype=np.uint64),
        vartype,
        name
    )"
);
#[cfg(feature = "lq")]
static PY_CODE: &'static CStr = c_str!(
    "
import numpy as np
from dimod import BinaryQuadraticModel

from luna_quantum._core import translator

def extract(bqm, name):
    if not isinstance(bqm, BinaryQuadraticModel):
        raise TypeError(f'Expected bqm to be of type BQM, received: {type(bqm)}')
    bqm_vars_ser = bqm.variables.to_serializable()
    for v in bqm_vars_ser:
        if not isinstance(v, str):
            raise TypeError(f'All BQM variables have to be of type str, received: {type(v)}')
    vars = np.array(bqm_vars_ser)
    vars_pos = {var: i for i, var in enumerate(vars)}

    linears = []
    linear_indices = []
    for var, val in bqm.linear.items():
        linears.append(val)
        linear_indices.append(vars_pos[var])
    quads = []
    quad_row = []
    quad_col = []
    for (var1, var2), val in bqm.quadratic.items():
        quads.append(val)
        quad_row.append(vars_pos[var1])
        quad_col.append(vars_pos[var2])

    vartype = bqm.vartype.name
    offset = float(bqm.offset)
    return translator.BqmTranslator.translate(
        vars,
        offset,
        np.array(linears, dtype=np.float64),
        np.array(linear_indices, dtype=np.uint64),
        np.array(quads, dtype=np.float64),
        np.array(quad_row, dtype=np.uint64),
        np.array(quad_col, dtype=np.uint64),
        vartype,
        name
    )"
);

/// Utility class for converting between dimod.BinaryQuadraticModel (BQM) and symbolic
/// models.
///
/// `BqmTranslator` provides methods to:
/// - Convert a BQM into a symbolic `Model`
/// - Convert a `Model` (with quadratic objective) into a BQM
///
/// These conversions are especially useful when interacting with external solvers
/// or libraries that operate on BQMs.
///
/// Examples
/// --------
/// >>> import dimod
/// >>> import numpy as np
/// >>> from luna_quantum import BqmTranslator, Vtype
/// >>> bqm = dimod.generators.gnm_random_bqm(5, 10, "BINARY")
///
/// Create a model from a matrix:
///
/// >>> model = BqmTranslator.to_aq(bqm, name="bqm_model")
///
/// Convert it back to a dense matrix:
///
/// >>> recovered = BqmTranslator.from_aq(model)
#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "BqmTranslator", module = "aqmodels._core.translator")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "BqmTranslator", module = "luna_quantum._core.translator")
)]
pub struct PyBqmTranslator {}

#[unwindable]
#[pymethods]
impl PyBqmTranslator {
    #[staticmethod]
    #[pyo3(signature=(vars, offset, linears, linear_indices, quads, quads_rows, quads_cols, vartype, name=None)
    )]
    fn translate(
        py: Python,
        vars: Py<PyAny>,
        offset: f64,
        linears: PyReadonlyArray1<f64>,
        linear_indices: PyReadonlyArray1<u64>,
        quads: PyReadonlyArray1<f64>,
        quads_rows: PyReadonlyArray1<u64>,
        quads_cols: PyReadonlyArray1<u64>,
        vartype: String,
        name: Option<String>,
    ) -> PyResult<PyModel> {
        let vtype = if vartype.to_uppercase() == String::from("SPIN") {
            Ok(Vtype::Spin)
        } else if vartype.to_uppercase() == String::from("BINARY") {
            Ok(Vtype::Binary)
        } else {
            Err(PyTypeError::new_err(format!("unknown vartype '{vartype}'")))
        }?;

        Ok(PyModel::new(BqmTranslator::model_from_bqm(
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
        )?))
    }

    /// Convert a symbolic model to a dense QUBO matrix representation.
    ///
    /// Parameters
    /// ----------
    /// model : Model
    ///     The symbolic model to convert. The objective must be quadratic-only
    ///     and unconstrained.
    ///
    /// Returns
    /// -------
    /// BinaryQuadraticModel
    ///     The resulting BQM.
    ///
    /// Raises
    /// ------
    /// TranslationError
    ///     Generally if the translation fails. Might be specified by one of the
    ///     four following errors.
    /// ModelNotQuadraticError
    ///     If the objective contains higher-order (non-quadratic) terms.
    /// ModelNotUnconstrainedError
    ///     If the model contains any constraints.
    /// ModelSenseNotMinimizeError
    ///     If the model's optimization sense is 'maximize'.
    /// ModelVtypeError
    ///     If the model contains different vtypes or vtypes other than binary and
    ///     spin.
    #[staticmethod]
    #[pyo3(signature=(model))]
    fn from_aq<'a>(py: Python<'a>, model: &PyModel) -> PyResult<Py<PyAny>> {
        let (offset, linear, quad, rows, cols, vtype, vars) =
            BqmTranslator::model_to_bqm(&model.access())?;
        let linear_py = linear.to_pyarray(py);
        let quadratic_py = quad.to_pyarray(py);
        let rows_py = rows.to_pyarray(py);
        let cols_py = cols.to_pyarray(py);
        let vtype_py = vtype.unwrap().to_string();
        let vars_py = vars.into_pyobject(py)?;

        let extractor: Py<PyAny> = PyModule::from_code(
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

    /// Convert a BQM into a symbolic `Model`.
    ///
    /// Parameters
    /// ----------
    /// bqm : BinaryQuadraticModel
    ///     The BQM.
    /// name : str, optional
    ///     An optional name to assign to the resulting model.
    ///
    /// Returns
    /// -------
    /// Model
    ///     A symbolic model representing the given BQM.
    #[staticmethod]
    #[pyo3(signature=(bqm, name=None))]
    fn to_aq(py: Python, bqm: Py<PyAny>, name: Option<Py<PyAny>>) -> PyResult<Py<PyAny>> {
        let extractor: Py<PyAny> = PyModule::from_code(py, PY_CODE, c_str!(""), c_str!(""))?
            .getattr("extract")?
            .into();
        let args = (bqm, name);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
