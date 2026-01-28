use std::fmt::Debug;

use lunamodel_core::Model;
use lunamodel_transform::{
    AnalysisCache, AnalysisCacheElement, AnalysisPass, AnalysisPassResult, BasePass,
};
use pyo3::{
    Bound, Py, PyAny, PyErr, PyResult, Python,
    exceptions::PyRuntimeError,
    types::{PyAnyMethods, PyType},
};

use crate::{
    model::PyModel,
    transform::{cache::PyAnalysisCache, interfaces::PyAnalysisPass},
};

pub struct PyAnalysisPassAdapter {
    pub(crate) inner: Py<PyAnalysisPass>,
}

impl PyAnalysisPassAdapter {
    pub fn new(inner: Py<PyAnalysisPass>) -> PyResult<Self> {
        let slf = Self { inner };
        slf.check_superclass()?;
        Ok(slf)
    }

    /// Check that the superclass implements all required methods.
    fn check_superclass(&self) -> Result<(), PyErr> {
        Python::attach(|py| {
            let base_cls = py.get_type::<PyAnalysisPass>();
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

impl BasePass for PyAnalysisPassAdapter {
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

impl AnalysisPass for PyAnalysisPassAdapter {
    fn run(&self, model: &Model, cache: &AnalysisCache) -> AnalysisPassResult {
        Python::attach(|py| {
            let pym: PyModel = model.clone().into();
            let pyc: PyAnalysisCache = cache.clone_py(py).into();
            let py_res = self
                .inner
                .call_method1(py, "_run", (pym, pyc))
                .map_err(|e| self.map_err(&e))?;
            let py_any: Py<PyAny> = py_res.extract(py).map_err(|e| self.map_err(&e))?;
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

impl Debug for PyAnalysisPassAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Clone for PyAnalysisPassAdapter {
    fn clone(&self) -> Self {
        Python::attach(|py| PyAnalysisPassAdapter {
            inner: self.inner.clone_ref(py),
        })
    }
}
