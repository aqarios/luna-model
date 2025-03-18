use crate::core::SampleSetTranslator;
use crate::py_bindings::py_sol::PySolution;
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::{pyclass, pymethods, PyResult};

#[pyclass(unsendable, name = "SampleSetTranslator")]
pub struct PySampleSetTranslator(pub SampleSetTranslator);

#[pymethods]
impl PySampleSetTranslator {
    #[staticmethod]
    fn from_dimod_sample_set(
        samples: PyReadonlyArray2<i64>,
        num_occurrences: PyReadonlyArray1<i64>,
    ) -> PyResult<PySolution> {
        Ok(PySolution(SampleSetTranslator::from_dimod_sample_set(
            samples.as_slice()?,
            num_occurrences.as_slice()?,
            samples.shape(),
        )))
    }
}
