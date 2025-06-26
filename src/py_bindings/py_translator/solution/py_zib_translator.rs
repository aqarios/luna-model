use crate::core::Sense;
use crate::py_bindings::py_env::{PyEnvironment, CURRENT_ENV};
use crate::py_bindings::py_exceptions::NoActiveEnvironmentFoundError;
use crate::py_bindings::py_sol::PySolution;
use crate::py_bindings::py_timing::PyTiming;
use crate::translator::ZibTranslator;
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::pyclass;
use std::collections::HashMap;
use std::ffi::CStr;

#[cfg(not(feature = "lq"))]
static PY_CODE: &'static CStr = c_str!(
    "
from aqmodels._core import translator, Sense

def extract(model, timing, env):
    sample = {x.name: model.getVal(x) for x in model.getVars()}
    sense = Sense.Max if model.getObjectiveSense() == 'maximize' else Sense.Min
    return translator.ZibTranslator.translate(sample, sense, timing, env)
"
);
#[cfg(feature = "lq")]
static PY_CODE: &'static CStr = c_str!(
    "
from aqmodels._core import translator

def extract(model, timing, env):
    sample = {x.name: model.getVal(x) for x in model.getVars()}
    sense = Sense.Max if model.getObjectiveSense() == 'maximize' else Sense.Min
    return translator.ZibTranslator.translate(sample, sense, timing, env)
"
);

/// Utility class for converting between a Zib solution and our solution format.
///
/// `ZibTranslator` provides methods to:
///
///     - Convert a Zib-style solution into our solution `Solution`.
///
/// The conversions are especially required when interacting with external zib solvers/samplers or
/// libraries that operate on zib-based problem-solving/sampling.
///
/// Examples
/// --------
/// >>> import luna_quantum as lq
/// >>> from pyscipopt import Model
/// >>> model = Model()
/// >>> model.readProblem("./path/to/my/model.lp")
/// >>> model.optimize()
/// >>> aqs = lq.translator.ZibTranslator.to_aq(model)
#[cfg_attr(feature = "lq",      pyclass(unsendable, name = "ZibTranslator", module = "luna_quantum.translator"))]
#[cfg_attr(not(feature = "lq"), pyclass(unsendable, name = "ZibTranslator", module = "aqmodels.translator"))]
pub struct PyZibTranslator(pub ZibTranslator);

#[pymethods]
impl PyZibTranslator {
    #[staticmethod]
    fn translate(
        // hashbrown::HashMap does not work here ;(
        sample: HashMap<String, f64>,
        sense: Sense,
        timing: Option<PyTiming>,
        env: Option<PyEnvironment>,
    ) -> PyResult<PySolution> {
        let environment: PyEnvironment = match env {
            Some(env) => env.clone(),
            None => CURRENT_ENV.with(|current| {
                current.borrow().clone().ok_or_else(|| {
                    NoActiveEnvironmentFoundError::new_err("no active environment found.")
                })
            })?,
        };
        Ok(PySolution(ZibTranslator::from_zib(
            sample,
            sense,
            timing.map(|t| t.into()),
            environment.0.clone(),
        )?))
    }

    /// Extract a solution from a ZIB model.
    ///
    /// Parameters
    /// ----------
    /// model : pyscipopt.Model
    ///     The Model that ran the optimization.
    /// timing : Timing, optional
    ///     The timing object produced while generating the result.
    /// env : Environment, optional
    ///     The environment of the model for which the result is produced.
    ///
    /// Raises
    /// ------
    /// NoActiveEnvironmentFoundError
    ///     If no environment is passed to the method or available from the context.
    /// SolutionTranslationError
    ///     Generally if the solution translation fails. Might be specified by one of the
    ///         two following errors.
    /// SampleIncorrectLengthError
    ///     If a solution's sample has a different number of variables than the model
    ///     environment passed to the translator.
    /// ModelVtypeError
    ///     If the result's variable types are incompatible with the model environment's
    ///     variable types.
    #[staticmethod]
    #[pyo3(signature=(model, timing=None, env=None))]
    fn to_aq(
        py: Python,
        model: PyObject,
        timing: Option<PyTiming>,
        env: Option<PyEnvironment>,
    ) -> PyResult<PyObject> {
        let extractor: Py<PyAny> = PyModule::from_code(py, PY_CODE, c_str!(""), c_str!(""))?
            .getattr("extract")?
            .into();
        let args = (model, timing, env);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
