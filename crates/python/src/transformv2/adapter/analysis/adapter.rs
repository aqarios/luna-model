use lunamodel_core::Model;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_transpiler::{AnalysisKey, AnalysisPass, PassContext};
use pyo3::{Py, PyAny, Python, types::PyAnyMethods};

use super::{PyAnalysisPass, PyAnalysisPassAdapterResult};
use crate::{
    PyModel,
    transformv2::{PyPassContext, adapter::utils::map_pyerr},
};

pub struct PyAnalysisPassAdapter {
    inner: Py<PyAnalysisPass>,
    name: String,
    requires: Vec<String>,
    provides: String,
}

impl PyAnalysisPassAdapter {
    pub fn new(py: Python, inner: Py<PyAnalysisPass>) -> LunaModelResult<Self> {
        let name: String = inner
            .call_method0(py, "name")
            .map_err(map_pyerr)?
            .extract(py)
            .map_err(map_pyerr)?;
        let requires: Vec<String> = inner
            .call_method0(py, "requires")
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
            requires,
            provides,
        })
    }
}

impl AnalysisPass for PyAnalysisPassAdapter {
    type Result = PyAnalysisPassAdapterResult;

    const NAME: &'static str = "lunamodel::PyAnalysisPassAdapter";
    const PROVIDES: &'static str = "lunamodel::PyProvided";

    fn name(&self) -> &str {
        &self.name
    }

    fn provides(&self) -> &str {
        &self.provides
    }

    fn requires(&self) -> &[String] {
        &self.requires
    }

    fn key<PyAnalysisPassAdapterResult>() -> AnalysisKey<PyAnalysisPassAdapterResult> {
        // AnalysisKey::new(Self::PROVIDES.to_string())
        unimplemented!(
            "the key on the PyAnalysisPassAdapter is not stable and should not be called."
        )
    }

    fn run(&self, model: &Model, ctx: &PassContext) -> LunaModelResult<Self::Result> {
        let py_result: Py<PyAny> = Python::attach(|py| {
            let obj = self.inner.bind(py);
            let res = obj
                .call_method1(
                    "_run",
                    (
                        PyModel::from(model.clone()),
                        PyPassContext::from(ctx.manager().clone()),
                    ),
                )
                .map_err(map_pyerr)?;
            Ok::<Py<PyAny>, LunaModelError>(res.unbind())
        })?;
        Ok(PyAnalysisPassAdapterResult(py_result))
    }
}
