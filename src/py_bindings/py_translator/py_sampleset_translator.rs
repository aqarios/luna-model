use crate::py_bindings::py_env::{PyEnvironment, CURRENT_ENV};
use crate::py_bindings::py_exceptions::NoActiveEnvironmentFoundError;
use crate::py_bindings::py_sol::PySolution;
use crate::py_bindings::py_timing::PyTiming;
use crate::translator::SampleSetTranslator;
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::{ffi::c_str, prelude::*};

#[pyclass(
    unsendable,
    name = "SampleSetTranslator",
    module = "aqmodels.translator"
)]
pub struct PySampleSetTranslator(pub SampleSetTranslator);

#[pymethods]
impl PySampleSetTranslator {
    #[staticmethod]
    #[pyo3(signature=(samples, num_occurrences, energy, timing=None, env=None))]
    fn translate(
        samples: PyReadonlyArray2<i64>,
        num_occurrences: PyReadonlyArray1<i64>,
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
        Ok(PySolution(
            SampleSetTranslator::from_dimod_sample_set(
                samples.as_slice()?,
                num_occurrences.as_slice()?,
                energy.as_slice()?,
                samples.shape(),
                timing.map(|t| t.into()),
                environment.into(),
            ).unwrap()))
    }

    #[staticmethod]
    #[pyo3(signature = (sampleset, timing=None, env=None))]
    fn from_dimod_sample_set(
        py: Python,
        sampleset: PyObject,
        timing: Option<PyObject>,
        env: Option<PyEnvironment>,
        // ) -> PyResult<PySolution> {
    ) -> PyResult<Py<PyAny>> {
        let extractor: Py<PyAny> = PyModule::from_code(
            py,
            c_str!(
                "
import numpy as np
from aqmodels._core import translator

def extract(sampleset, timing, env):
    sampleset = sampleset.aggregate()
    record = sampleset.record
    sample = record.sample.astype(np.int64, order='C')
    num_occurrences = record.num_occurrences.astype(np.int64, order='C')
    energy = record.energy.astype(np.float64, order='C')
    return translator.SampleSetTranslator.translate(
        sample, num_occurrences, energy, timing, env
    )"
            ),
            c_str!(""),
            c_str!(""),
        )?
            .getattr("extract")?
            .into();
        let args = (sampleset, timing, env);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
