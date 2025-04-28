use std::rc::Rc;

use numpy::PyReadonlyArray1;
use pyo3::{ffi::c_str, prelude::*};

use crate::{
    core::ConcreteRcVarRef,
    py_bindings::{
        py_env::{PyEnvironment, CURRENT_ENV},
        py_exceptions::NoActiveEnvironmentFoundError,
        py_sol::PySolution,
        py_timing::PyTiming,
        py_var::PyVariable,
    },
    translator::QctrlTranslator,
};

#[pyclass(unsendable, name = "QctrlTranslator", module = "aqmodels.translator")]
pub struct PyQctrlTranslator(pub QctrlTranslator);

#[pymethods]
impl PyQctrlTranslator {
    #[staticmethod]
    #[pyo3(signature=(sample, energy, variable_list=None, timing=None, env=None))]
    fn translate(
        sample: PyReadonlyArray1<i64>,
        energy: f64,
        variable_list: Option<Vec<PyVariable>>,
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
            variable_list.and_then(|vl| {
                Some(
                    vl.iter()
                        .map(|e| Rc::clone(&e.0))
                        .collect::<Vec<ConcreteRcVarRef>>(),
                )
            }),
            timing.map(|t| t.into()),
            environment.into(),
        )?))
    }

    #[staticmethod]
    #[pyo3(signature=(result, variable_list=None, timing=None, env=None))]
    fn from_qctrl(
        py: Python,
        result: PyObject,
        variable_list: Option<Vec<PyVariable>>,
        timing: Option<PyTiming>,
        env: Option<PyEnvironment>,
    ) -> PyResult<Py<PyAny>> {
        let extractor: Py<PyAny> = PyModule::from_code(
            py,
            c_str!(
                "
import numpy as np
from aqmodels._core import translator

def extract(result, variable_list, timing, env):
    sample = result.get('solution_bitstring')
    energy = result.get('final_aggregate_cost')
    return translator.QctrlTranslator.translate(
        np.array(sample, dtype=np.int64),
        energy,
        variable_list,
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

        let args = (result, variable_list, timing, env);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
