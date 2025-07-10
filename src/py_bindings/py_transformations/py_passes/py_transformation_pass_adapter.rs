use std::{fmt::Debug, rc::Rc};

use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyType};

use crate::{
    core::{Model, Solution},
    py_bindings::{py_model::PyModel, py_sol::PySolution},
    transformations::{
        analysis_cache::{AnalysisCache, PyAnalysisCache},
        base_passes::{BasePass, TransformationPass, TransformationPassResult},
        errors::TransformationPassError,
    },
};

use super::py_transformation_pass::{PyTransformationOutcome, PyTransformationPass};

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
    fn invalidates(&self) -> Vec<String> {
        Python::with_gil(|py| {
            self.inner
                .getattr(py, "invalidates")
                .and_then(|res| res.extract::<Vec<String>>(py))
                .expect("no 'invalidates' method")
        })
    }

    fn run(&self, model: Model, cache: &AnalysisCache) -> TransformationPassResult {
        let py_outcome = Python::with_gil(|py| {
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
                .map_err(|e| self.map_err(&e))?;
            let py_outcome: PyTransformationOutcome =
                py_res.extract(py).map_err(|e| self.map_err(&e))?;
            Ok(py_outcome)
        })?;
        let outcome = py_outcome.try_into().map_err(|e| self.map_err(&e))?;
        Ok(outcome)
    }

    fn backwards(&self, solution: Solution, cache: &AnalysisCache) -> Solution {
        let py_sol = Python::with_gil(|py| {
            let py_res = self
                .inner
                .call_method1(
                    py,
                    "backwards",
                    (
                        PySolution::new(solution),
                        PyAnalysisCache::new(cache.clone_py(py)),
                    ),
                )
                .map_err(|e| self.map_err(&e))?;
            let py_sol: PySolution = py_res.extract(py).map_err(|e| self.map_err(&e))?;
            Ok::<PySolution, TransformationPassError>(py_sol)
        }).unwrap(); // Backwards cannot have error currently.
        let sol: Solution = Rc::into_inner(py_sol.0 .0)
            .ok_or(self.map_err(&"Solution reference leaked out of backwards scope.")).unwrap();
        sol
    }
}

impl Debug for PyTransformationPassAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Clone for PyTransformationPassAdapter {
    fn clone(&self) -> Self {
        Python::with_gil(|py| PyTransformationPassAdapter {
            inner: self.inner.clone_ref(py),
        })
    }
}
