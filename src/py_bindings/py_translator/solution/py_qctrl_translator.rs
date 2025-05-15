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
