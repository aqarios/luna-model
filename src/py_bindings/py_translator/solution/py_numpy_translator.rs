use crate::py_bindings::py_env::{PyEnvironment, CURRENT_ENV};
use crate::py_bindings::py_exceptions::NoActiveEnvironmentFoundError;
use crate::py_bindings::py_sol::PySolution;
use crate::py_bindings::py_timing::PyTiming;
use crate::translator::NpArrayTranslator;
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::ffi::c_str;
use pyo3::prelude::*;

#[pyclass(unsendable, name = "NumpyTranslator", module = "aqmodels.translator")]
pub struct PyNumpyTranslator {}

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

    #[staticmethod]
    #[pyo3(signature = (result, energies, timing=None, env=None))]
    fn to_aq(
        py: Python,
        result: PyObject,
        energies: PyObject,
        timing: Option<PyTiming>,
        env: Option<PyEnvironment>,
    ) -> PyResult<PyObject> {
        let extractor: Py<PyAny> = PyModule::from_code(
            py,
            c_str!(
                "
import numpy as np
from aqmodels._core import translator

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
            ),
            c_str!(""),
            c_str!(""),
        )?
        .getattr("extract")?
        .into();
        let args = (result, energies, timing, env);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
