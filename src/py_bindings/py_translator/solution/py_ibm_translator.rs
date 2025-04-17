use crate::{
    py_bindings::{py_env::PyEnvironment, py_sol::PySolution, py_timing::PyTiming},
    translator::IbmTranslator,
};
use pyo3::{ffi::c_str, prelude::*};

#[pyclass(unsendable, name = "IbmTranslator", module = "aqmodels.translator")]
pub struct PyIbmTranslator {}

#[pymethods]
impl PyIbmTranslator {
    #[staticmethod]
    #[pyo3(signature=(samples, ordering, energies, num_occurences, timing=None, env=None))]
    fn translate(
        samples: Vec<Vec<i64>>,
        ordering: Vec<Vec<String>>,
        energies: Vec<f64>,
        num_occurences: Vec<i64>,
        timing: Option<PyTiming>,
        env: Option<PyEnvironment>,
    ) -> PyResult<PySolution> {
        println!("{samples:?}");
        println!("{ordering:?}");
        println!("{energies:?}");
        println!("{num_occurences:?}");
        Ok(PySolution(IbmTranslator::from_ibm()?))
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

def extract(result, quadratic_program, timing, env):
    counts = result[0].data.meas.get_counts()
    
    samples = []
    ordering = []
    energies = []
    num_occurences = []
    
    for bitstring, count in counts.items():
        vars, sample = tuple(zip(*[
            (qp.variables[i].name, int(b))
            for i, b in enumerate(bitstring)
        ]))
        obj_val = qp.objective.evaluate(sample)

        samples.append(sample)
        ordering.append(vars)
        num_occurences.append(count)
        energies.append(obj_val)
        

    return translator.IbmTranslator.translate(
        samples,
        ordering,
        energies,
        num_occurences,
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

        let args = (result, quadratic_program, timing, env);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
