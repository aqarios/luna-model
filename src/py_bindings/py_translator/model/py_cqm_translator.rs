use pyo3::ffi::c_str;
use pyo3::prelude::*;
use std::ffi::CStr;

#[cfg(not(feature = "lq"))]
static PY_CODE_TO_AQ: &'static CStr = c_str!(
    "
from dimod import ConstrainedQuadraticModel
from dimod import lp as dimod_lp

from aqmodels._core import translator

def extract(cqm):
    if not isinstance(cqm, ConstrainedQuadraticModel):
        raise TypeError(f'Expected cqm to be of type CQM, received: {type(cqm)}')
    cqm_lp = dimod_lp.dumps(cqm)
    return translator.LpTranslator.to_aq(cqm_lp)
"
);

#[cfg(feature = "lq")]
static PY_CODE_TO_AQ: &'static CStr = c_str!(
    "
from dimod import ConstrainedQuadraticModel
from dimod import lp as dimod_lp

from luna_quantum._core import translator

def extract(cqm):
    if not isinstance(cqm, ConstrainedQuadraticModel):
        raise TypeError(f'Expected cqm to be of type CQM, received: {type(cqm)}')
    cqm_lp = dimod_lp.dumps(cqm)
    return translator.LpTranslator.to_aq(cqm_lp)
"
);

#[cfg(not(feature = "lq"))]
static PY_CODE_FROM_AQ: &'static CStr = c_str!(
    "
from dimod import ConstrainedQuadraticModel
from dimod import lp as dimod_lp

from aqmodels._core import translator

def extract(model):
    lp = translator.LpTranslator.from_aq(model)
    return dimod_lp.loads(lp)
"
);
#[cfg(feature = "lq")]
static PY_CODE_FROM_AQ: &'static CStr = c_str!(
    "
from dimod import ConstrainedQuadraticModel
from dimod import lp as dimod_lp

from luna_quantum._core import translator

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
/// >>> from luna_quantum import CqmTranslator, Vtype
/// >>> bqm = dimod.generators.gnm_random_bqm(5, 10, "BINARY")
///
/// Create a model from a matrix:
///
/// >>> model = CqmTranslator.to_aq(bqm, name="bqm_model")
///
/// Convert it back to a dense matrix:
///
/// >>> recovered = CqmTranslator.from_aq(model)
#[pyclass(unsendable, name = "CqmTranslator", module = "aqmodels.translator")]
pub struct PyCqmTranslator {}

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
    fn from_aq<'a>(py: Python<'a>, model: PyObject) -> PyResult<PyObject> {
        let extractor: PyObject = PyModule::from_code(py, PY_CODE_FROM_AQ, c_str!(""), c_str!(""))?
            .getattr("extract")?
            .into();
        let args = (model,);
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
    #[pyo3(signature=(cqm))]
    fn to_aq(py: Python, cqm: PyObject) -> PyResult<PyObject> {
        let extractor: PyObject = PyModule::from_code(py, PY_CODE_TO_AQ, c_str!(""), c_str!(""))?
            .getattr("extract")?
            .into();
        let args = (cqm,);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
