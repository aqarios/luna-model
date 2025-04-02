use crate::core::SampleSetTranslator;
use pyo3::pyclass;

#[pyclass(unsendable, name = "SampleSetTranslator")]
pub struct PySampleSetTranslator(pub SampleSetTranslator);

// #[pymethods]
// impl PySampleSetTranslator {
//     #[staticmethod]
//     fn from_dimod_sample_set(
//         samples: PyReadonlyArray2<i64>,
//         num_occurrences: PyReadonlyArray1<i64>,
//         timing: Option<PyTiming>,
//     ) -> PyResult<PySolution> {
//         Ok(PySolution(SampleSetTranslator::from_dimod_sample_set(
//             samples.as_slice()?,
//             num_occurrences.as_slice()?,
//             samples.shape(),
//             timing.map(|t| t.into()),
//         )))
//     }
// }
