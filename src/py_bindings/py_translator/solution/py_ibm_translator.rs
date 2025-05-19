use crate::{
    py_bindings::{
        py_env::{PyEnvironment, CURRENT_ENV},
        py_exceptions::NoActiveEnvironmentFoundError,
        py_sol::PySolution,
        py_timing::PyTiming,
        py_var::PyVariable,
    },
    translator::IbmTranslator,
};
use pyo3::{ffi::c_str, prelude::*};
use std::rc::Rc;

/// Utility class for converting between an IBM solution and an AqSolution (ours).
///
///
/// `IbmTranslator` provides methods to:
/// - Convert an IBM-style solution into our solution `Solution`.
///
/// The conversions are especially required when interaction with external ibm solvers/samplers or libraries that operate on ibm-based problem solving/sampling.
///
/// Examples
/// --------
/// >>> import aqmodels as aqm
/// >>> ...
/// >>> ibm_result = ...
/// >>> aqs = aqm.translator.IbmTranslator.to_aq(ibm_result)
#[pyclass(unsendable, name = "IbmTranslator", module = "aqmodels.translator")]
pub struct PyIbmTranslator {}

#[pymethods]
impl PyIbmTranslator {
    #[staticmethod]
    #[pyo3(signature=(samples, orderings, energies, counts, timing=None, env=None))]
    fn translate(
        samples: Vec<Vec<i64>>,
        orderings: Vec<PyVariable>,
        energies: Vec<f64>,
        counts: Vec<usize>,
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
        Ok(PySolution(IbmTranslator::from_ibm(
            &samples,
            &orderings.iter().map(|e| Rc::clone(&e.0)).collect(),
            &energies,
            counts,
            timing.map(|t| t.into()),
            environment.into(),
        )?))
    }

    /// Convert an IBM solution to an AqSolution.
    ///
    /// Parameters
    /// ----------
    /// result : PrimitiveResult[PubResult]
    ///     The ibm result.
    /// quadratic_program : QuadraticProgram
    ///     The quadratic program defining the optimization problem.
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
    #[pyo3(signature=(result, quadratic_program, timing=None, env=None))]
    fn to_aq(
        py: Python,
        result: PyObject,
        quadratic_program: PyObject,
        timing: Option<PyTiming>,
        env: Option<PyEnvironment>,
    ) -> PyResult<Py<PyAny>> {
        let extractor: Py<PyAny> = PyModule::from_code(
            py,
            c_str!(
                "
import numpy as np
from aqmodels._core import translator

def extract(result, qp, timing, env):
    meas: BitArray = result[0].data.meas
    counts: dict[str, int] = meas.get_counts()

    samples = []
    energies = []
    flat_counts = []

    ordering = []

    for n, (bitstring, count) in enumerate(counts.items()):
        sample = []
        for i, b in enumerate(bitstring):
            sample.append(int(b))
            
            if n == 0:
                ordering.append(env.get_variable(qp.variables[i].name))

        energies.append(float(qp.objective.evaluate(sample)))
        samples.append(sample[::-1]) # reverse ordering for correct bitstrings.
        flat_counts.append(count)

    return translator.IbmTranslator.translate(
        samples, 
        ordering, 
        energies, 
        flat_counts, 
        timing, 
        env
    )
"
            ),
            c_str!(""),
            c_str!(""),
        )?
            .getattr("extract")?
            .into();

        let args = (result, quadratic_program, timing, env);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
