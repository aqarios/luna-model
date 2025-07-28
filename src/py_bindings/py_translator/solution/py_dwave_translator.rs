use crate::py_bindings::py_env::{PyEnvironment, CURRENT_ENV};
use crate::py_bindings::py_exceptions::NoActiveEnvironmentFoundError;
use crate::py_bindings::py_sol::PySolution;
use crate::py_bindings::py_timing::PyTiming;
use crate::translator::solution::DwaveTranslator;
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::{ffi::c_str, prelude::*};
use std::ffi::CStr;
use unwind_macros::unwindable;
use crate::py_bindings::unwind;

#[cfg(not(feature = "lq"))]
static PY_CODE: &'static CStr = c_str!(
    "
import numpy as np
from aqmodels._core import translator

def extract(sampleset, timing, env):
    sampleset = sampleset.aggregate()
    variables = sampleset.variables
    record = sampleset.record
    sample = record.sample.astype(np.int64, order='C')
    counts = record.num_occurrences.astype(np.int64, order='C')
    energy = record.energy.astype(np.float64, order='C')
    return translator.DwaveTranslator.translate(
        sample, variables, counts, energy, timing, env
    )"
);
#[cfg(feature = "lq")]
static PY_CODE: &'static CStr = c_str!(
    "
import numpy as np
from luna_quantum._core import translator

def extract(sampleset, timing, env):
    sampleset = sampleset.aggregate()
    variables = sampleset.variables
    record = sampleset.record
    sample = record.sample.astype(np.int64, order='C')
    counts = record.num_occurrences.astype(np.int64, order='C')
    energy = record.energy.astype(np.float64, order='C')
    return translator.DwaveTranslator.translate(
        sample, variables, counts, energy, timing, env
    )"
);

/// Utility class for converting between a DWAVE solution and our solution format.
///
/// `DWaveSolutionTranslator` provides methods to:
/// - Convert a dimod-style solution into our solution `Solution`.
///
/// The conversions are especially required when interacting with external dwave/dimod
/// solvers/samplers or libraries that operate on dwave/dimod-based problem-solving/sampling.
///
/// Examples
/// --------
/// >>> import dimod
/// >>> import luna_quantum as lq
/// >>> dwave_sampleset = ...
/// >>> aqs = lq.translator.DwaveTranslator.to_aq(dwave_sampleset)
#[cfg_attr(not(feature = "lq"), pyclass(name = "DwaveTranslator", module = "aqmodels._core.translator"))]
#[cfg_attr(feature = "lq",      pyclass(name = "DwaveTranslator", module = "luna_quantum._core.translator"))]
pub struct PyDwaveTranslator(pub DwaveTranslator);

#[unwindable]
#[pymethods]
impl PyDwaveTranslator {
    #[staticmethod]
    #[pyo3(signature=(samples, variables_order, counts, energy, timing=None, env=None))]
    fn translate(
        samples: PyReadonlyArray2<i64>,
        variables_order: Vec<String>,
        counts: PyReadonlyArray1<i64>,
        energy: PyReadonlyArray1<f64>,
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
        Ok(PySolution::new(DwaveTranslator::from_dimod_sample_set(
            samples.as_slice()?,
            variables_order.as_slice(),
            counts.as_slice()?,
            energy.as_slice()?,
            samples.shape(),
            timing.map(|t| t.into()),
            environment.0.clone(),
        )?))
    }

    /// Convert a DWave SampleSet to our solution format.
    ///
    /// Parameters
    /// ----------
    /// sample_set : SampleSet
    ///     The SampleSet returned by a DWave solver.
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
    #[pyo3(signature = (sample_set, timing=None, env=None))]
    fn to_aq(
        py: Python,
        sample_set: PyObject,
        timing: Option<PyObject>,
        env: Option<PyEnvironment>,
    ) -> PyResult<Py<PyAny>> {
        let extractor: Py<PyAny> = PyModule::from_code(py, PY_CODE, c_str!(""), c_str!(""))?
            .getattr("extract")?
            .into();
        let args = (sample_set, timing, env);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
