//! Runtime adapter for Python meta-analysis passes.

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_transpiler::{AnalysisKey, MetaAnalysisPass, PipelineStep};
use pyo3::{Py, PyAny, Python, types::PyAnyMethods};

use crate::transform::utils::{FromSteps, map_pyerr};

use super::{PyMetaAnalysisPass, result::PyMetaAnalysisPassAdapterResult};

pub struct PyMetaAnalysisPassAdapter {
    inner: Py<PyMetaAnalysisPass>,
    name: String,
    provides: String,
}

impl PyMetaAnalysisPassAdapter {
    pub fn new(py: Python, inner: Py<PyMetaAnalysisPass>) -> LunaModelResult<Self> {
        let name: String = inner
            .call_method0(py, "name")
            .map_err(map_pyerr)?
            .extract(py)
            .map_err(map_pyerr)?;
        let provides: String = inner
            .call_method0(py, "provides")
            .map_err(map_pyerr)?
            .extract(py)
            .map_err(map_pyerr)?;

        Ok(Self {
            inner,
            name,
            provides,
        })
    }

    pub fn inner(&self, py: Python) -> Py<PyMetaAnalysisPass> {
        self.inner.clone_ref(py)
    }
}

impl MetaAnalysisPass for PyMetaAnalysisPassAdapter {
    type Result = PyMetaAnalysisPassAdapterResult;

    const NAME: &'static str = "lunamodel::PyMetaAnalysisPassAdapter";
    const PROVIDES: &'static str = "lunamodel::PyMetaAnalysisProvided";

    fn name(&self) -> &str {
        &self.name
    }

    fn provides(&self) -> &str {
        &self.provides
    }

    fn key<PyMetaAnalysisPassAdapterResult>() -> AnalysisKey<PyMetaAnalysisPassAdapterResult> {
        // AnalysisKey::new(Self::PROVIDES.to_string())
        unimplemented!(
            "the key on the PyMetaAnalysisPassAdapter is not stable and should not be called."
        )
    }

    fn run(&self, steps: &[PipelineStep]) -> LunaModelResult<Self::Result> {
        let py_result: Py<PyAny> = Python::attach(|py| {
            let obj = self.inner.bind(py);
            let py_steps: Vec<Py<PyAny>> = steps.to_pypasses(py)?;
            let res = obj.call_method1("_run", (py_steps,)).map_err(map_pyerr)?;
            Ok::<Py<PyAny>, LunaModelError>(res.unbind())
        })?;
        Ok(PyMetaAnalysisPassAdapterResult(py_result))
    }
}
