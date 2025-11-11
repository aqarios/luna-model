use crate::py_bindings::py_env::{PyEnvironment, CURRENT_ENV};
use crate::py_bindings::py_exceptions::NoActiveEnvironmentFoundError;
use crate::py_bindings::py_sol::PySolution;
use crate::py_bindings::py_timing::PyTiming;
use crate::py_bindings::unwind;
use crate::translator::NpArrayTranslator;
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use std::ffi::CStr;
use unwind_macros::unwindable;

static PY_CODE: &'static CStr = c_str!(
    "
import numpy as np
from luna_model._core import translator

def extract(result, energies, timing, env):
    (sol_agg, indices, num_occ) = np.unique(
        result, return_index=True, return_counts=True, axis=0
    )

    sol_agg = sol_agg.astype(np.float64, order='C')
    indices = indices.astype(np.uint64, order='C')
    num_occ = num_occ.astype(np.uint64, order='C')
    energies = energies.astype(np.float64, order='C')

    return translator.AwsTranslator.translate(
        sol_agg, indices, num_occ, energies, timing, env
    )
"
);

/// Utility class for converting between a result consisting of numpy arrays and our solution
/// format.
///
/// `NumpyTranslator` provides methods to:
/// - Convert a numpy-array result into our solution `Solution`.
///
/// Examples
/// --------
/// >>> import luna_model as lm
/// >>> from numpy.typing import NDArray
/// >>> result: NDArray = ...
/// >>> energies: NDArray = ...
/// >>> lms = lm.translator.NumpyTranslator.to_aq(result, energies)
#[pyclass(name = "NumpyTranslator", module = "luna_model._core.translator")]
pub struct PyNumpyTranslator {}

#[unwindable]
#[pymethods]
impl PyNumpyTranslator {
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

        Ok(PySolution::new(NpArrayTranslator::from_numpy_arrays(
            sol_agg.as_slice()?,
            counts.as_slice()?,
            indices.as_slice()?,
            energies.as_slice()?,
            sol_agg.shape(),
            timing.map(|t| t.into()),
            environment.0.clone(),
        )?))
    }

    /// Convert a solution in the format of numpy arrays to our solution format.
    /// Note that the optimization sense is always assumed to be minimization.
    ///
    /// Parameters
    /// ----------
    /// result : NDArray
    ///     The samples as a 2D array where each row corresponds to one sample.
    /// energies : NDArray
    ///     The energies of the single samples as a 1D array.
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
    #[pyo3(signature = (result, energies, timing=None, env=None))]
    pub fn to_aq(
        py: Python,
        result: Py<PyAny>,
        energies: Py<PyAny>,
        timing: Option<PyTiming>,
        env: Option<PyEnvironment>,
    ) -> PyResult<Py<PyAny>> {
        let extractor: Py<PyAny> = PyModule::from_code(py, PY_CODE, c_str!(""), c_str!(""))?
            .getattr("extract")?
            .into();
        let args = (result, energies, timing, env);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
