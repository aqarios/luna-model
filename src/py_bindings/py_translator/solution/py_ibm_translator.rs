use std::rc::Rc;
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

#[pyclass(unsendable, name = "IbmTranslator", module = "aqmodels.translator")]
pub struct PyIbmTranslator {}

#[pymethods]
impl PyIbmTranslator {
    #[staticmethod]
    #[pyo3(signature=(samples, orderings, energies, num_occurences, timing=None, env=None))]
    fn translate(
        samples: Vec<Vec<i64>>,
        orderings: Vec<Vec<PyVariable>>,
        energies: Vec<f64>,
        num_occurences: Vec<usize>,
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
        let some = samples.as_slice();
        Ok(PySolution(IbmTranslator::from_ibm(
            &samples,
            &orderings
                .iter()
                .map(|l| l.iter().map(|e| Rc::clone(&e.0)).collect())
                .collect(),
            &energies,
            num_occurences,
            timing.map(|t| t.into()),
            environment.into(),
        )?))
    }

    #[staticmethod]
    #[pyo3(signature=(result, quadratic_program, timing=None, env=None))]
    fn from_ibm(
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
    orderings = []
    energies = []
    num_occurences = []

    for bitstring, count in counts.items():
        sample = []
        order = []
        for i, b in enumerate(bitstring):
            sample.append(int(b))
            order.append(env.get_variable(qp.variables[i].name))

        samples.append(sample)
        orderings.append(order)
        energies.append(float(qp.objective.evaluate(sample)))
        num_occurences.append(count)

        sample = {qp.variables[i].name: int(b) for i, b in enumerate(bitstring)}

    return translator.IbmTranslator.translate(
        samples, 
        orderings, 
        energies, 
        num_occurences, 
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
