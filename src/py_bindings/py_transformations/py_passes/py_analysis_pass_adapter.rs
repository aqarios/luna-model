use std::fmt::Debug;

use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyType};

use crate::{
    core::Model,
    py_bindings::{py_model::PyModel, AnyPass, IntoAnyPass},
    transformations::{
        analysis_cache::{AnalysisCache, AnalysisCacheElement, PyAnalysisCache},
        base_passes::{AnalysisPass, AnalysisPassResult, BasePass},
    },
};

use super::py_analysis_pass::PyAnalysisPass;

pub struct PyAnalysisPassAdapter {
    inner: Py<PyAnalysisPass>,
}

impl PyAnalysisPassAdapter {
    pub fn new(inner: Py<PyAnalysisPass>) -> PyResult<Self> {
        let slf = Self { inner };
        slf.check_superclass()?;
        Ok(slf)
    }

    /// Check that the superclass implements all required methods.
    fn check_superclass(&self) -> Result<(), PyErr> {
        Python::with_gil(|py| {
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
        Python::with_gil(|py| {
            self.inner
                .getattr(py, "name")
                .and_then(|res| res.extract::<String>(py))
                .expect("no 'name' method")
        })
    }

    fn requires(&self) -> Vec<String> {
        Python::with_gil(|py| {
            self.inner
                .getattr(py, "requires")
                .and_then(|res| res.extract::<Vec<String>>(py))
                .expect("no 'requires' method")
        })
    }
}

impl AnalysisPass for PyAnalysisPassAdapter {
    fn run(&self, model: &Model, cache: &AnalysisCache) -> AnalysisPassResult {
        Python::with_gil(|py| {
            let py_res = self
                .inner
                .call_method1(
                    py,
                    "run",
                    (
                        PyModel::new(model.clone()),
                        PyAnalysisCache::new(cache.clone_py(py)),
                    ),
                )
                .map_err(|e| self.map_err(&e))?;
            let py_any: Py<PyAny> = py_res.extract(py).map_err(|e| self.map_err(&e))?;
            if py_any.is_none(py) {
                Ok(None)
            } else {
                Ok(Some(AnalysisCacheElement::PyAnalysis(py_any)))
            }
        })
    }
}

impl Debug for PyAnalysisPassAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Clone for PyAnalysisPassAdapter {
    fn clone(&self) -> Self {
        Python::with_gil(|py| PyAnalysisPassAdapter {
            inner: self.inner.clone_ref(py),
        })
    }
}

impl IntoAnyPass for PyAnalysisPassAdapter {
    fn as_anypass(&self) -> AnyPass {
        Python::with_gil(|py| AnyPass::PyAnalysisPass(self.inner.clone_ref(py)))
    }
}
