//t Runtime adapter for Python analysis passes.

use std::any::Any;

use lunamodel_core::Model;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_transpiler::{AnalysisKey, AnalysisPass, PassContext, TranspileKindResult};
use pyo3::{Py, PyAny, Python, types::PyAnyMethods};

use super::{PyAnalysisPass, PyAnalysisPassAdapterResult};
use crate::{
    PyModel,
    transform::{PyPassContext, utils::map_pyerr},
};

pub struct PyAnalysisPassAdapter {
    inner: Py<PyAnalysisPass>,
    name: String,
    requires: Vec<String>,
    provides: String,
    /// Set when the pass opts into cross-language plugin consumption; its result must then be a
    /// protobuf message. `None` for native-Python-only analyses.
    result_type_tag: Option<String>,
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

        // If the pass declares its result is a shareable protobuf payload, register an on-demand
        // serializer so C-ABI plugins (any language) can consume it. Native-only passes return
        // `None` and stay unregistered (readable only by pure-Python consumers).
        let result_type_tag: Option<String> = match inner.call_method0(py, "result_type_tag") {
            Ok(value) => value.extract::<Option<String>>(py).unwrap_or(None),
            Err(_) => None,
        };
        if let Some(tag) = &result_type_tag {
            lunamodel_transpiler::register_analysis_serializer(
                provides.clone(),
                tag.clone(),
                serialize_py_result,
            );
        }

        Ok(Self {
            inner,
            name,
            requires,
            provides,
            result_type_tag,
        })
    }

    pub fn inner(&self, py: Python) -> Py<PyAnalysisPass> {
        self.inner.clone_ref(py)
    }
}

impl AnalysisPass for PyAnalysisPassAdapter {
    type Result = PyAnalysisPassAdapterResult;

    const PROVIDES: &'static str = "luna_model::PyAnalysisProvided";

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

    fn run(&self, model: &Model, ctx: &PassContext) -> TranspileKindResult<Self::Result> {
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
            // Enforce the opt-in contract: a plugin-consumable result must be a protobuf message.
            if self.result_type_tag.is_some() && !res.hasattr("SerializeToString").map_err(map_pyerr)? {
                return Err(map_pyerr(pyo3::exceptions::PyTypeError::new_err(format!(
                    "analysis '{}' declares result_type_tag, so run() must return a protobuf \
                     message (with SerializeToString())",
                    self.name,
                ))));
            }
            Ok::<Py<PyAny>, LunaModelError>(res.unbind())
        })?;
        Ok(PyAnalysisPassAdapterResult(py_result))
    }
}

/// Serializes a stored pure-Python analysis result — a protobuf message — to its wire bytes, so a
/// C-ABI plugin (any language) can consume it. Registered per-pass in [`PyAnalysisPassAdapter::new`]
/// only when the pass declares a `result_type_tag`; calls the result's `SerializeToString()`
/// (google protobuf) so the bytes match the shared `.proto` the consumer decodes.
fn serialize_py_result(value: &(dyn Any + Send + Sync)) -> Option<Vec<u8>> {
    let result = value.downcast_ref::<PyAnalysisPassAdapterResult>()?;
    Python::attach(|py| {
        let obj = result.0.bind(py);
        let bytes = obj.call_method0("SerializeToString").ok()?;
        bytes.extract::<Vec<u8>>().ok()
    })
}
