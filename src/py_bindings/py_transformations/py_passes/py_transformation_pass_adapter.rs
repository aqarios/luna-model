use std::fmt::Debug;

use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyType};

use crate::{
    core::{Model, Solution},
    py_bindings::py_model::PyModel,
    transformations::{
        analysis_cache::{AnalysisCache, PyAnalysisCache},
        base_passes::{BasePass, TransformationPass, TransformationPassResult, TransformationType},
        errors::TransformationPassError,
    },
};

use super::py_transformation_pass::PyTransformationPass;

pub struct PyTransformationPassAdapter {
    inner: Py<PyTransformationPass>,
}

impl PyTransformationPassAdapter {
    pub fn new(inner: Py<PyTransformationPass>) -> PyResult<Self> {
        let slf = Self { inner };
        slf.check_superclass()?;
        Ok(slf)
    }

    /// Check that the superclass implements all required methods.
    fn check_superclass(&self) -> Result<(), PyErr> {
        Python::with_gil(|py| {
            let base_cls = py.get_type::<PyTransformationPass>();
            let cls = self.inner.getattr(py, "__class__")?;
            let cls_name: String = cls.getattr(py, "__name__")?.extract(py)?;
            Self::check_overridden(py, "name", &base_cls, &cls, &cls_name)?;
            Self::check_overridden(py, "run", &base_cls, &cls, &cls_name)?;
            Self::check_overridden(py, "backwards", &base_cls, &cls, &cls_name)?;
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
                "{} is not a valid TransformationPass: must override '{}'",
                cls_name, name,
            )))
        } else {
            Ok(())
        }
    }
}

impl BasePass for PyTransformationPassAdapter {
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

impl TransformationPass for PyTransformationPassAdapter {
    fn invalidates(&self) -> &[&str] {
        &[]
    }

    fn run(&self, mut model: Model, cache: &AnalysisCache) -> TransformationPassResult {
        Python::with_gil(|py| {
            let fallback_name = String::from("PyTransformationPassAdapter");
            let cls_name: String = self
                .inner
                .getattr(py, "__class__")
                .map_err(|e| TransformationPassError(fallback_name.clone(), e.to_string()))?
                .getattr(py, "__name__")
                .map_err(|e| TransformationPassError(fallback_name.clone(), e.to_string()))?
                .extract(py)
                .map_err(|e| TransformationPassError(fallback_name.clone(), e.to_string()))?;
            let py_res = self
                .inner
                .call_method1(
                    py,
                    "run",
                    (
                        PyModel::new(model),
                        PyAnalysisCache::new(cache.clone_py(py)),
                    ),
                )
                .map_err(|e| TransformationPassError(cls_name.clone(), e.to_string()))?;
            let (py_model, py_tt): (Py<PyModel>, Py<TransformationType>) = py_res
                .extract(py)
                .map_err(|e| TransformationPassError(cls_name.clone(), e.to_string()))?;
            let py_model_borrow = py_model.borrow(py);
            let pymodel = py_model_borrow.clone();
            model = pymodel.concrete_model.borrow().clone();
            let tt = py_tt.borrow(py);
            Ok((model, tt.clone()))
        })
    }

    fn backwards(&self, mut _solution: Solution, _cache: &AnalysisCache) -> Solution {
        todo!()
    }
}

impl Debug for PyTransformationPassAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
