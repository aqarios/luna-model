use pyo3::{ffi::c_str, prelude::*};
use std::{collections::HashMap, ffi::CStr};

use crate::py_bindings::py_usize::PyUsize;
use crate::{
    py_bindings::{
        py_env::{PyEnvironment, CURRENT_ENV},
        py_exceptions::{NoActiveEnvironmentFoundError, SolutionTranslationError},
        py_sol::PySolution,
        py_timing::PyTiming,
    },
    translator::QctrlTranslator,
};

#[cfg(not(feature = "lq"))]
static PY_CODE: &'static CStr = c_str!(
    "
import numpy as np
from aqmodels._core import translator

def extract(result, timing, env):
    return translator.QctrlTranslator.translate(
        result.get('solution_bitstring', None),
        result.get('solution_bitstring_cost', None),
        result.get('final_bitstring_distribution', None),
        result.get('variables_to_bitstring_index_map', None),
        timing,
        env,
    )
"
);
#[cfg(feature = "lq")]
static PY_CODE: &'static CStr = c_str!(
    "
import numpy as np
from luna_quantum._core import translator

def extract(result, timing, env):
    return translator.QctrlTranslator.translate(
        result.get('solution_bitstring', None),
        result.get('solution_bitstring_cost', None),
        result.get('final_bitstring_distribution', None),
        result.get('variables_to_bitstring_index_map', None),
        timing,
        env,
    )
"
);

/// Utility class for converting between a QCTRL solution and our solution format.
///
/// `QctrlTranslator` provides methods to:
/// - Convert a Qctrl-style solution into our solution `Solution`.
///
/// The conversions are especially required when interacting with external qctrl solvers/samplers or
/// libraries that operate on qctrl-based problem-solving/sampling.
///
/// Examples
/// --------
/// >>> import luna_quantum as lq
/// >>> ...
/// >>> qctrl_result = ...
/// >>> aqs = lq.translator.QctrlTranslator.to_aq(qctrl_result)
#[cfg_attr(not(feature = "lq"), pyclass(unsendable, name = "QctrlTranslator", module = "aqmodels.translator"))]
#[cfg_attr(feature = "lq",      pyclass(unsendable, name = "QctrlTranslator", module = "luna_quantum.translator"))]
pub struct PyQctrlTranslator(pub QctrlTranslator);

#[pymethods]
impl PyQctrlTranslator {
    #[staticmethod]
    #[pyo3(signature=(solution_bitstring, solution_bitstring_cost, final_bitstring_distribution, variables_to_bitstring_index_map, timing=None, env=None))]
    fn translate(
        solution_bitstring: Option<String>,
        solution_bitstring_cost: Option<f64>,
        final_bitstring_distribution: Option<HashMap<String, PyUsize>>,
        variables_to_bitstring_index_map: Option<HashMap<String, PyUsize>>,
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
        if solution_bitstring.is_none() {
            return Err(SolutionTranslationError::new_err(
                "QCTRL result does not contain a 'solution_bitstring'.",
            ));
        }
        if solution_bitstring_cost.is_none() {
            return Err(SolutionTranslationError::new_err(
                "QCTRL result does not contain a 'solution_bitstring_cost'.",
            ));
        }
        if final_bitstring_distribution.is_none() {
            return Err(SolutionTranslationError::new_err(
                "QCTRL result does not contain a 'final_bitstring_distribution'.",
            ));
        }
        if variables_to_bitstring_index_map.is_none() {
            return Err(SolutionTranslationError::new_err(
                "QCTRL result does not contain a 'variables_to_bitstring_index_map'.",
            ));
        }
        let mapper: HashMap<usize, usize> = variables_to_bitstring_index_map
            .unwrap()
            .iter()
            .map(|(k, &v)| {
                let index = regex::Regex::new(r"\[([^\]]+)\]")
                    .unwrap()
                    .captures(k)
                    .unwrap()[1]
                    .parse::<usize>()
                    .unwrap();
                (index, v.into())
            })
            .collect();
        let mut samples: Vec<Vec<usize>> = Vec::new();
        let mut counts = Vec::new();
        let mut energies = Vec::new();
        for (bs, &count) in final_bitstring_distribution.unwrap().iter() {
            if bs == solution_bitstring.as_ref().unwrap() {
                energies.push(Some(solution_bitstring_cost.unwrap()));
            } else {
                energies.push(None);
            };
            let unordered_sample: Vec<usize> = bs
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
            let sample: Vec<usize> = (0..unordered_sample.len())
                .into_iter()
                .map(|i| unordered_sample[mapper[&i]])
                .collect();
            samples.push(sample);
            counts.push(count.into());
        }

        Ok(PySolution(QctrlTranslator::from_qctrl(
            samples,
            counts,
            energies,
            timing.map(|t| t.into()),
            environment.0.clone(),
        )?))
    }

    /// Convert a QCTRL result to our solution format.
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
        let extractor: Py<PyAny> = PyModule::from_code(py, PY_CODE, c_str!(""), c_str!(""))?
            .getattr("extract")?
            .into();

        let args = (result, timing, env);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
