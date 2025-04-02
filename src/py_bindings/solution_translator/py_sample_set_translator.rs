use crate::core::SampleSetTranslator;
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
    fn from_dimod_sample_set(
        samples: PyReadonlyArray2<i64>,
        num_occurrences: PyReadonlyArray1<i64>,
        timing: Option<PyTiming>,
    ) -> PyResult<PySolution> {
        Ok(PySolution(Rc::new(
            SampleSetTranslator::from_dimod_sample_set(
                samples.as_slice()?,
                num_occurrences.as_slice()?,
                samples.shape(),
                timing.map(|t| t.into()),
            )
            .unwrap(),
        )))
    }
}
