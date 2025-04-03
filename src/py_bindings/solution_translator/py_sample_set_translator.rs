use crate::core::SampleSetTranslator;
use crate::py_bindings::py_env::{PyEnvironment, CURRENT_ENV};
use crate::py_bindings::py_exceptions::NoActiveEnvironmentFoundError;
use crate::py_bindings::py_sol::PySolution;
use crate::py_bindings::py_timing::PyTiming;
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::prelude::*;
use std::rc::Rc;

#[pyclass(unsendable, name = "SampleSetTranslator")]
pub struct PySampleSetTranslator(pub SampleSetTranslator);

#[pymethods]
impl PySampleSetTranslator {
    #[staticmethod]
    #[pyo3(signature=(samples, num_occurrences, timing=None, env=None))]
    fn from_dimod_sample_set(
        samples: PyReadonlyArray2<i64>,
        num_occurrences: PyReadonlyArray1<i64>,
        timing: Option<PyTiming>,
        env: Option<&mut PyEnvironment>,
    ) -> PyResult<PySolution> {
        let environment: PyEnvironment = match env {
            Some(env) => env.clone(),
            None => CURRENT_ENV.with(|current| {
                current.borrow().clone().ok_or_else(|| {
                    NoActiveEnvironmentFoundError::new_err("no active environment found.")
                })
            })?,
        };
        Ok(PySolution(Rc::new(
            SampleSetTranslator::from_dimod_sample_set(
                samples.as_slice()?,
                num_occurrences.as_slice()?,
                samples.shape(),
                timing.map(|t| t.into()),
                environment.into(),
            )
            .unwrap(),
        )))
    }
}
