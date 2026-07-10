//! Python wrapper for the transpiler analysis context.

use std::sync::Arc;

use lunamodel_error::LunaModelError;
use lunamodel_transform::analysis::{
    CheckModelSpecsAnalysis, MaxBias, MaxBiasAnalysis, MinConstraintValues,
    MinValueForConstraintAnalysis, SpecsAnalysis,
};
use lunamodel_transpiler::{AnalysisKey, AnalysisManager, AnalysisPass, PassContext};
use lunamodel_types::Specs;
use pyo3::{IntoPyObjectExt, Py, PyAny, PyResult, Python, pyclass, pymethods};

use crate::{
    PyModelSpecs,
    transform::{
        adapter::{PyAnalysisPassAdapterResult, PyMetaAnalysisPassAdapterResult},
        builtin::analysis::{PyMaxBias, PyMinConstraintValues},
        error::to_pyerr,
    },
};

#[pyclass(subclass, from_py_object)]
#[derive(Clone)]
pub struct PyPassContext {
    /// Analysis cache and dependency manager shared across a pipeline run.
    pub manager: Arc<AnalysisManager>,
}

impl From<AnalysisManager> for PyPassContext {
    fn from(manager: AnalysisManager) -> Self {
        Self {
            manager: Arc::new(manager),
        }
    }
}

impl From<Arc<AnalysisManager>> for PyPassContext {
    fn from(manager: Arc<AnalysisManager>) -> Self {
        Self { manager }
    }
}

impl<'c> From<&'c PyPassContext> for PassContext<'c> {
    fn from(val: &'c PyPassContext) -> Self {
        PassContext::new(&val.manager)
    }
}

impl PyPassContext {
    pub fn inner(&self) -> Arc<AnalysisManager> {
        Arc::clone(&self.manager)
    }
}

#[pymethods]
impl PyPassContext {
    /// Create an empty analysis context.
    #[new]
    fn new() -> Self {
        Self::from(AnalysisManager::default())
    }

    /// Resolve an analysis result by key and convert it to a Python object.
    ///
    /// Built-in analyses have dedicated wrapper types so Python code sees a
    /// stable API. For adapter-based analyses, the method falls back to the
    /// erased result objects stored under their dynamic keys.
    fn require_analysis(&self, py: Python, key: String) -> PyResult<Py<PyAny>> {
        let res = match key.as_str() {
            x if x == CheckModelSpecsAnalysis::PROVIDES => {
                let _: &() = self
                    .manager
                    .require(&CheckModelSpecsAnalysis::key())
                    .map_err(to_pyerr)?;
                &py.None()
            }
            x if x == MaxBiasAnalysis::PROVIDES => {
                let a: &MaxBias = self
                    .manager
                    .require(&MaxBiasAnalysis::key())
                    .map_err(to_pyerr)?;
                &PyMaxBias(*a).into_py_any(py)?
            }
            x if x == MinValueForConstraintAnalysis::PROVIDES => {
                let a: &MinConstraintValues = self
                    .manager
                    .require(&MinValueForConstraintAnalysis::key())
                    .map_err(to_pyerr)?;
                &PyMinConstraintValues(a.clone()).into_py_any(py)?
            }
            x if x == SpecsAnalysis::PROVIDES => {
                let a: &Specs = self
                    .manager
                    .require(&SpecsAnalysis::key())
                    .map_err(to_pyerr)?;
                &PyModelSpecs::from(a.clone()).into_py_any(py)?
            }
            _ => {
                // It can either be a PyAnalysisPassAdapterResult or a PyMetaAnalysisPassAdapaterResult.
                let from_analysis =
                    self.manager
                        .require(&AnalysisKey::<PyAnalysisPassAdapterResult>::new(
                            key.clone(),
                        ));
                let from_meta = self
                    .manager
                    .require(&AnalysisKey::<PyMetaAnalysisPassAdapterResult>::new(key));
                match (from_analysis, from_meta) {
                    (Ok(a), Err(_)) => &a.0,
                    (Err(_), Ok(m)) => &m.0,
                    (Ok(_), Ok(_)) => {
                        return Err(LunaModelError::Internal(
                            "found multiple entries for the same key.".into(),
                        ))?;
                    }
                    (Err(ea), Err(_)) => {
                        return Err(ea).map_err(to_pyerr)?;
                    }
                }
            }
        };
        Ok(res.clone_ref(py))
    }
}
