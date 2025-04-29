use crate::py_bindings::py_env::{PyEnvironment, CURRENT_ENV};
use crate::py_bindings::py_exceptions::NoActiveEnvironmentFoundError;
use crate::py_bindings::py_sol::PySolution;
use crate::py_bindings::py_timing::PyTiming;
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyUntypedArrayMethods};
use crate::translator::AwsTranslator;
use pyo3::prelude::*;
use pyo3::ffi::c_str;

#[pyclass(unsendable, name = "AwsTranslator", module = "aqmodels.translator")]
pub struct PyAwsTranslator(pub AwsTranslator);

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
        
        Ok(PySolution(AwsTranslator::from_aws_result(
            sol_agg.as_slice()?,
            counts.as_slice()?,
            indices.as_slice()?,
            energies.as_slice()?,
            sol_agg.shape(),
            timing.map(|t| t.into()),
            environment.into(),
        )?))
    }

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
