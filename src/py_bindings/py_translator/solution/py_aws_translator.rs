use crate::py_bindings::py_env::{PyEnvironment, CURRENT_ENV};
use crate::py_bindings::py_exceptions::NoActiveEnvironmentFoundError;
use crate::py_bindings::py_sol::PySolution;
use crate::py_bindings::py_timing::PyTiming;
use crate::translator::NpArrayTranslator;
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::ffi::c_str;
use pyo3::prelude::*;

/// Utility class for converting between an AWS result and an AqSolution (ours).
///
/// `AwsTranslator` provides methods to:
/// - Convert an AWS-style result into our solution `Solution`.
///
/// The conversions are especially required when interaction with external aws solvers/samplers or libraries that operate on aws-based problem solving/sampling.
///
/// Examples
/// --------
/// >>> import aqmodels as aqm
/// >>> aws_result = ...
/// >>> aqs = aqm.translator.AwsTranslator.to_aq(aws_result)
#[pyclass(unsendable, name = "AwsTranslator", module = "aqmodels.translator")]
pub struct PyAwsTranslator(pub NpArrayTranslator);

#[pymethods]
impl PyAwsTranslator {
    #[staticmethod]
    fn translate(
        sol_agg: PyReadonlyArray2<f64>,
        indices: PyReadonlyArray1<usize>,
        counts: PyReadonlyArray1<usize>,
        energies: PyReadonlyArray1<f64>,
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

        Ok(PySolution(NpArrayTranslator::from_numpy_arrays(
            sol_agg.as_slice()?,
            counts.as_slice()?,
            indices.as_slice()?,
            energies.as_slice()?,
            sol_agg.shape(),
            timing.map(|t| t.into()),
            environment.into(),
        )?))
    }

    /// Convert an AWS Braket result to an AqSolution.
    ///
    /// Parameters
    /// ----------
    /// result : dict[str, Any]
    ///     The aws braket result.
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
    #[pyo3(signature = (aws_result, timing=None, env=None))]
    fn to_aq(
        py: Python,
        aws_result: PyObject,
        timing: Option<PyTiming>,
        env: Option<PyEnvironment>,
    ) -> PyResult<PyObject> {
        let extractor: Py<PyAny> = PyModule::from_code(
            py,
            c_str!(
                "
import numpy as np
from aqmodels._core import translator

def extract(aws_result, timing, env):
    (sol_agg, indices, num_occ) = np.unique(
        aws_result['samples'], return_index=True, return_counts=True, axis=0
    )
    energies = aws_result['energies']

    sol_agg = sol_agg.astype(np.float64, order='C')
    indices = indices.astype(np.uint64, order='C')
    num_occ = num_occ.astype(np.uint64, order='C')
    energies = energies.astype(np.float64, order='C')

    return translator.AwsTranslator.translate(
        sol_agg, indices, num_occ, energies, timing, env
    )
"
            ),
            c_str!(""),
            c_str!(""),
        )?
            .getattr("extract")?
            .into();
        let args = (aws_result, timing, env);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
