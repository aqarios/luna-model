use crate::py_bindings::py_env::{PyEnvironment, CURRENT_ENV};
use crate::py_bindings::py_exceptions::NoActiveEnvironmentFoundError;
use crate::py_bindings::py_sol::PySolution;
use crate::py_bindings::py_timing::PyTiming;
use crate::translator::ZibTranslator;
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::pyclass;
use std::collections::HashMap;

#[pyclass(unsendable, name = "ZibTranslator", module = "aqmodels.translator")]
pub struct PyZibTranslator(pub ZibTranslator);

#[pymethods]
impl PyZibTranslator {
    #[staticmethod]
    fn translate(
        // hashbrown::HashMap does not work here ;(
        sample: HashMap<String, f64>,
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
            sample, timing.map(|t| t.into()),
            environment.into(),
        )?))
    }

    #[staticmethod]
    fn from_zib(
        py: Python,
        model: PyObject,
        timing: Option<PyTiming>,
        env: Option<PyEnvironment>,
    ) -> PyResult<PyObject> {
        let extractor: Py<PyAny> = PyModule::from_code(
            py,
            c_str!(
                "
from aqmodels._core import translator

def extract(model, timing, env):
    sample = {x.name: model.getVal(x) for x in model.getVars()}
    return translator.ZibTranslator.translate(sample, timing, env)
"
            ),
            c_str!(""),
            c_str!(""),
        )?
            .getattr("extract")?
            .into();
        let args = (model, timing, env);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
