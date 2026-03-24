use std::{fmt::Debug, sync::Arc};

use lunamodel_core::{Model, Solution};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_transform::{AnalysisCache, BasePass, TransformationPass, TransformationPassResult};
use pyo3::{
    Bound, Py, PyAny, PyErr, PyResult, Python,
    exceptions::PyRuntimeError,
    pyclass::PyClassGuardError,
    types::{PyAnyMethods, PyType},
};

use crate::{
    model::PyModel,
    sol::PySolution,
    transform::{
        cache::PyAnalysisCache,
        interfaces::{PyTransformationOutcome, PyTransformationPass},
    },
};

pub struct PyTransformationPassAdapter {
    pub(crate) inner: Py<PyTransformationPass>,
}

impl PyTransformationPassAdapter {
    pub fn new(inner: Py<PyTransformationPass>) -> PyResult<Self> {
        let slf = Self { inner };
        slf.check_superclass()?;
        Ok(slf)
    }

    /// Check that the superclass implements all required methods.
    fn check_superclass(&self) -> Result<(), PyErr> {
        Python::attach(|py| {
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

impl TransformationPass for PyTransformationPassAdapter {
    fn invalidates(&self) -> Vec<String> {
        Python::attach(|py| {
            self.inner
                .getattr(py, "invalidates")
                .and_then(|res| res.extract::<Vec<String>>(py))
                .expect("no 'invalidates' method")
        })
    }

    fn run(&self, model: Model, cache: &AnalysisCache) -> TransformationPassResult {
        let py_outcome = Python::attach(|py| {
            let pym: PyModel = model.into();
            let pyc: PyAnalysisCache = cache.clone_py(py).into();
            let py_res = self
                .inner
                .call_method1(py, "_run", (pym, pyc))
                .map_err(|e| LunaModelError::WithCause(Box::new(self.map_err(&e)), e.into()))?;
            let py_outcome: PyTransformationOutcome = py_res.extract(py).map_err(|e: PyErr| {
                LunaModelError::WithCause(Box::new(self.map_err(&e)), e.into())
            })?;
            Ok::<PyTransformationOutcome, LunaModelError>(py_outcome)
        })?;
        let outcome = py_outcome.try_into().map_err(|e| self.map_err(&e))?;
        Ok(outcome)
    }

    fn backwards(&self, solution: Solution, cache: &AnalysisCache) -> LunaModelResult<Solution> {
        let py_sol = Python::attach(|py| {
            let pysol: PySolution = solution.into();
            let pycache: PyAnalysisCache = cache.clone_py(py).into();
            let py_res = self
                .inner
                .call_method1(py, "_backwards", (pysol, pycache))
                .map_err(|e| LunaModelError::WithCause(Box::new(self.map_err(&e)), e.into()))?;
            let py_sol: PySolution = py_res.extract(py).map_err(|e: PyClassGuardError| {
                let mapped = self.map_err(&e);
                let pye: PyErr = e.into();
                LunaModelError::WithCause(Box::new(mapped), pye.into())
            })?;
            Ok::<PySolution, LunaModelError>(py_sol)
        })?;
        let sol: Solution = Arc::into_inner(py_sol.s)
            .ok_or(self.map_err(&"Solution reference leaked out of backwards scope."))?
            .into_inner();
        // unwrap();
        Ok(sol)
    }

    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl Debug for PyTransformationPassAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Clone for PyTransformationPassAdapter {
    fn clone(&self) -> Self {
        Python::attach(|py| PyTransformationPassAdapter {
            inner: self.inner.clone_ref(py),
        })
    }
}
