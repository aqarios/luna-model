use std::fmt::Debug;

use lunamodel_error::LunaModelError;
use lunamodel_transform::{
    AnalysisCache, AnalysisCacheElement, BasePass, Pass,
    passes::special::{MetaAnalysisPass, MetaAnalysisPassResult},
};
use pyo3::{
    Bound, CastError, Py, PyAny, PyErr, PyResult, Python,
    exceptions::PyRuntimeError,
    types::{PyAnyMethods, PyType},
};

use crate::transform::{PyAnalysisCache, interfaces::PyMetaAnalysisPass, pass::PyPass};

pub struct PyMetaAnalysisPassAdapter {
    pub(crate) inner: Py<PyMetaAnalysisPass>,
}

impl PyMetaAnalysisPassAdapter {
    pub fn new(inner: Py<PyMetaAnalysisPass>) -> PyResult<Self> {
        let slf = Self { inner };
        slf.check_superclass()?;
        Ok(slf)
    }

    /// Check that the superclass implements all required methods.
    fn check_superclass(&self) -> Result<(), PyErr> {
        Python::attach(|py| {
            let base_cls = py.get_type::<PyMetaAnalysisPass>();
            let cls = self.inner.getattr(py, "__class__")?;
            let cls_name: String = cls.getattr(py, "__name__")?.extract(py)?;
            Self::check_overridden(py, "name", &base_cls, &cls, &cls_name)?;
            Self::check_overridden(py, "run", &base_cls, &cls, &cls_name)?;
            Ok(())
        })
    }

    fn check_overridden(
        py: Python,
        name: &str,
        base: &Bound<PyType>,
        cls: &Py<PyAny>,
        cls_name: &String,
    ) -> PyResult<()> {
        let cls_method = cls.getattr(py, name)?;
        let base_method = base.getattr(name)?;

        if cls_method.is(&base_method) {
            Err(PyRuntimeError::new_err(format!(
                "{} is not a valid AnalysisPass: must override '{}'",
                cls_name, name,
            )))
        } else {
            Ok(())
        }
    }
}

impl BasePass for PyMetaAnalysisPassAdapter {
    fn name(&self) -> String {
        Python::attach(|py| {
            self.inner
                .getattr(py, "name")
                .and_then(|res| res.extract::<String>(py))
                .expect("no 'name' method")
        })
    }

    fn requires(&self) -> Vec<String> {
        Python::attach(|py| {
            self.inner
                .getattr(py, "requires")
                .and_then(|res| res.extract::<Vec<String>>(py))
                .expect("no 'requires' method")
        })
    }
}

impl MetaAnalysisPass for PyMetaAnalysisPassAdapter {
    fn run(&self, passes: &[Pass], cache: &AnalysisCache) -> MetaAnalysisPassResult {
        Python::attach(|py| {
            let py_passes: PyResult<Vec<PyPass>> =
                passes.iter().map(|p| PyPass::from_pass(p)).collect();

            let py_passes = py_passes
                .map_err(|e| LunaModelError::WithCause(Box::new(self.map_err(&e)), e.into()))?;
            let pyc: PyAnalysisCache = cache.clone_py(py).into();
            let py_res = self
                .inner
                .call_method1(py, "_run", (py_passes, pyc))
                .map_err(|e| LunaModelError::WithCause(Box::new(self.map_err(&e)), e.into()))?;
            let py_any: Py<PyAny> = py_res.extract(py).map_err(|e: CastError| {
                let mapped = self.map_err(&e);
                let pye: PyErr = e.into();
                LunaModelError::WithCause(Box::new(mapped), pye.into())
            })?;
            if py_any.is_none(py) {
                Ok(None)
            } else {
                Ok(Some(AnalysisCacheElement::PyAnalysis(py_any)))
            }
        })
    }

    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl Debug for PyMetaAnalysisPassAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Clone for PyMetaAnalysisPassAdapter {
    fn clone(&self) -> Self {
        Python::attach(|py| PyMetaAnalysisPassAdapter {
            inner: self.inner.clone_ref(py),
        })
    }
}
