use std::fmt::Debug;

use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyType};

use crate::{
    py_bindings::{AnyPass, IntoAnyPass},
    transformations::{
        analysis_cache::{AnalysisCache, AnalysisCacheElement, PyAnalysisCache},
        base_passes::{BasePass, Pass},
        passes::special::meta_analysis::{MetaAnalysisPass, MetaAnalysisPassResult},
    },
};

use super::py_meta_analysis::PyMetaAnalysisPass;

pub struct PyMetaAnalysisPassAdapter {
    inner: Py<PyMetaAnalysisPass>,
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
    fn run(&self, pipeline: &Vec<Pass>, cache: &AnalysisCache) -> MetaAnalysisPassResult {
        let passes: Vec<AnyPass> = pipeline.iter().map(|p| p.as_anypass()).collect();
        Python::attach(|py| {
            let py_res = self
                .inner
                .call_method1(
                    py,
                    "run",
                    (
                        passes,
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

impl IntoAnyPass for PyMetaAnalysisPassAdapter {
    fn as_anypass(&self) -> AnyPass {
        Python::attach(|py| AnyPass::PyMetaAnalysisPass(self.inner.clone_ref(py)))
    }
}
