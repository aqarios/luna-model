//! Python wrapper for reversible transformation records.

use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
use lunamodel_transpiler::TransformationRecord;
use lunamodel_unwind::*;
use pyo3::{Bound, Py, PyAny, PyResult, Python, pyclass, pymethods, types::PyBytes, types::PyType};

use crate::{
    PySolution,
    transform::{entry::PyPassEntry, error::to_pyerr},
};

#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyTransformationRecord {
    pub tr: TransformationRecord,
}

impl From<TransformationRecord> for PyTransformationRecord {
    fn from(tr: TransformationRecord) -> Self {
        Self { tr }
    }
}

#[unwindable]
#[pymethods]
impl PyTransformationRecord {
    #[getter]
    fn entries(&self) -> Vec<PyPassEntry> {
        self.tr.entries().map(|e| e.into()).collect()
    }

    fn backward(&self, solution: PySolution) -> PyResult<PySolution> {
        Ok(self
            .tr
            .backward(solution.s.read_arc().clone())
            .map_err(to_pyerr)?
            .into())
    }

    #[pyo3(signature=(compress=true, level=3))]
    fn encode(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<Py<PyAny>> {
        let bytes = self.tr.encode(compress, level)?;
        Ok(PyBytes::new(py, bytes.as_slice()).into())
    }

    #[classmethod]
    fn decode(_cls: &Bound<'_, PyType>, py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        let record: TransformationRecord =
            data.as_bytes(py).unversionize().decompress()?.decode(())?;
        Ok(record.into())
    }

    fn find(&self, query: String, exact: bool) -> PyResult<PyPassEntry> {
        Ok(self.tr.find(&query, exact).map_err(to_pyerr)?.into())
    }
}
