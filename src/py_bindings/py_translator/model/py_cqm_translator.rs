use crate::py_bindings::py_model::PyModel;
use crate::py_bindings::unwind;
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use std::ffi::CStr;
use unwind_macros::unwindable;

static PY_CODE_TO_AQ: &'static CStr = c_str!(
    "
from dimod import ConstrainedQuadraticModel
from dimod import lp as dimod_lp

from luna_model._core import translator

def extract(cqm):
    if not isinstance(cqm, ConstrainedQuadraticModel):
        raise TypeError(f'Expected cqm to be of type CQM, received: {type(cqm)}')
    cqm_lp = dimod_lp.dumps(cqm)
    return translator.LpTranslator.to_aq(cqm_lp)
"
);

static PY_CODE_FROM_AQ: &'static CStr = c_str!(
    "
from dimod import ConstrainedQuadraticModel
from dimod import lp as dimod_lp

from luna_model._core import translator

def extract(model):
    lp = translator.LpTranslator.from_aq(model)
    return dimod_lp.loads(lp)
"
);

/// Utility class for converting between dimod.BinaryQuadraticModel (CQM) and symbolic
/// models.
///
/// `CqmTranslator` provides methods to:
/// - Convert a CQM into a symbolic `Model`
/// - Convert a `Model` (with quadratic objective) into a CQM
///
/// These conversions are especially useful when interacting with external solvers
/// or libraries that operate on CQMs.
///
/// Examples
/// --------
/// >>> import dimod
/// >>> import numpy as np
/// >>> from luna_model import CqmTranslator, Vtype
/// >>> bqm = dimod.generators.gnm_random_bqm(5, 10, "BINARY")
///
/// Create a model from a matrix:
///
/// >>> model = CqmTranslator.to_aq(bqm, name="bqm_model")
///
/// Convert it back to a dense matrix:
///
/// >>> recovered = CqmTranslator.from_aq(model)
#[pyclass(name = "CqmTranslator", module = "luna_model._core.translator")]
pub struct PyCqmTranslator {}

#[unwindable]
#[pymethods]
impl PyCqmTranslator {
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
    ///     The resulting CQM.
    ///
    /// Raises
    /// ------
    /// TranslationError
    ///     If the translation fails for some reason.
    #[staticmethod]
    #[pyo3(signature=(model))]
    pub fn from_aq<'a>(py: Python<'a>, model: &PyModel) -> PyResult<Py<PyAny>> {
        let extractor: Py<PyAny> =
            PyModule::from_code(py, PY_CODE_FROM_AQ, c_str!(""), c_str!(""))?
                .getattr("extract")?
                .into();
        let args = (model.clone(),);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }

    /// Convert a CQM into a symbolic `Model`.
    ///
    /// Parameters
    /// ----------
    /// cqm : ConstrainedQuadraticModel
    ///     The CQM.
    ///
    /// Returns
    /// -------
    /// Model
    ///     A symbolic model representing the given CQM.
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If `cqm` is not of type `ConstrainedQuadraticModel`.
    /// TranslationError
    ///     If the translation fails for some reason.
    #[staticmethod]
    pub fn to_aq(py: Python, cqm: Py<PyAny>) -> PyResult<Py<PyAny>> {
        let extractor: Py<PyAny> = PyModule::from_code(py, PY_CODE_TO_AQ, c_str!(""), c_str!(""))?
            .getattr("extract")?
            .into();
        let args = (cqm,);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
