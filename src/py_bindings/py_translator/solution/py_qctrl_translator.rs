use numpy::PyReadonlyArray1;
use pyo3::{ffi::c_str, prelude::*};

use crate::{
    py_bindings::{
        py_env::{PyEnvironment, CURRENT_ENV},
        py_exceptions::NoActiveEnvironmentFoundError,
        py_sol::PySolution,
        py_timing::PyTiming,
    },
    translator::QctrlTranslator,
};

/// Utility class for converting between a QCTRL solution and an AqSolution (ours).
///
/// `QctrlTranslator` provides methods to:
/// - Convert a Qctrl-style solution into our solution `Solution`.
///
/// The conversions are especially required when interaction with external qctrl solvers/samplers or libraries that operate on qctrl-based problem solving/sampling.
///
/// Examples
/// --------
/// >>> import aqmodels as aqm
/// >>> ...
/// >>> qctrl_result = ...
/// >>> aqs = aqm.translator.QctrlTranslator.to_aq(qctrl_result)
#[pyclass(unsendable, name = "QctrlTranslator", module = "aqmodels.translator")]
pub struct PyQctrlTranslator(pub QctrlTranslator);

#[pymethods]
impl PyQctrlTranslator {
    #[staticmethod]
    #[pyo3(signature=(sample, energy, timing=None, env=None))]
    fn translate(
        sample: PyReadonlyArray1<i64>,
        energy: f64,
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
        Ok(PySolution(QctrlTranslator::from_qctrl(
            sample.as_slice()?,
            energy,
            timing.map(|t| t.into()),
            environment.into(),
        )?))
    }

    /// Convert a QCTRL result to an AqSolution.
    ///
    /// Parameters
    /// ----------
    /// result : dict[str, Any]
    ///     The qctrl result as a dictionary.
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
    ///     two following errors.
    /// SampleIncorrectLengthError
    ///     If a solution's sample has a different number of variables than the model
    ///     environment passed to the translator.
    /// ModelVtypeError
    ///     If the result's variable types are incompatible with the model environment's
    ///     variable types.
    #[staticmethod]
    #[pyo3(signature=(result, timing=None, env=None))]
    fn to_aq(
        py: Python,
        result: PyObject,
        timing: Option<PyTiming>,
        env: Option<PyEnvironment>,
    ) -> PyResult<Py<PyAny>> {
        let extractor: Py<PyAny> = PyModule::from_code(
            py,
            c_str!(
                "
import numpy as np
from aqmodels._core import translator

def extract(result, timing, env):
    sample = result.get('solution_bitstring')
    energy = result.get('final_aggregate_cost')
    return translator.QctrlTranslator.translate(
        np.array(sample, dtype=np.int64),
        energy,
        timing,
        env,
    )
"
            ),
            c_str!(""),
            c_str!(""),
        )?
            .getattr("extract")?
            .into();

        let args = (result, timing, env);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
