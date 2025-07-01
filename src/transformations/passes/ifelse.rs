#[cfg(feature = "py")]
use aqm_macros::py_pass;
use pyo3::exceptions::PyTypeError;
use pyo3::IntoPyObjectExt;

#[cfg(feature = "py")]
use crate::py_bindings::AnyPass;

use crate::core::Model;
use crate::core::Solution;
use crate::transformations::analysis_cache::AnalysisCache;
use crate::transformations::analysis_cache::PyAnalysisCache;
use crate::transformations::base_passes::{
    BasePass, Pass, TransformationPass, TransformationPassResult,
};
use crate::transformations::errors::CompilationError;
use crate::transformations::errors::TransformationPassError;
use crate::transformations::pass_manager::PassManager;

pub type RustCallback = fn(&AnalysisCache) -> bool;

#[cfg_attr(feature = "py", pyclass(unsendable))]
struct RustCallbackWrapper {
    callback: RustCallback,
}

#[cfg(feature = "py")]
#[pymethods]
impl RustCallbackWrapper {
    fn __call__(&self, cache_py: &PyAnalysisCache) -> PyResult<bool> {
        Ok((self.callback)(&cache_py.0))
    }
}

#[derive(Debug)]
pub enum Condition {
    RsCallback(RustCallback),
    #[cfg(feature = "py")]
    PyCallback(Py<PyAny>),
}

impl Condition {
    fn call(&self, cache: &AnalysisCache) -> Result<bool, CompilationError> {
        match self {
            Self::RsCallback(rs_fn) => Ok(rs_fn(cache)),
            #[cfg(feature = "py")]
            Self::PyCallback(py_fn) => Python::with_gil(|py| {
                let r = py_fn
                    .call1(py, (PyAnalysisCache::new(cache.clone_py(py)),))
                    .map_err(|e| CompilationError(e.to_string()))?;
                let b = r
                    .extract::<bool>(py)
                    .map_err(|e| CompilationError(e.to_string()))?;
                Ok(b)
            }),
        }
    }
}

#[cfg(feature = "py")]
impl<'py> IntoPyObject<'py> for Condition {
    type Error = PyErr;
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            Condition::PyCallback(cb) => Ok(cb.into_pyobject(py)?),
            Condition::RsCallback(rs) => {
                let wrapper = Py::new(py, RustCallbackWrapper { callback: rs })?;
                Ok(wrapper.into_py_any(py)?.into_pyobject(py)?)
            }
        }
    }
}

#[cfg(feature = "py")]
impl<'py> FromPyObject<'py> for Condition {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let py = ob.py();
        if ob.is_callable() {
            let cb: Py<PyAny> = ob.into_py_any(py)?;
            Ok(Condition::PyCallback(cb))
        } else {
            Err(PyTypeError::new_err("Condition must be callable"))
        }
    }
}

impl Clone for Condition {
    fn clone(&self) -> Self {
        match self {
            Self::RsCallback(inner) => Self::RsCallback(inner.clone()),
            #[cfg(feature = "py")]
            Self::PyCallback(pyany) => Self::PyCallback(Python::with_gil(|py| pyany.clone_ref(py))),
        }
    }
}

#[cfg_attr(feature = "py", py_pass(pass_variant = "Transformation"))]
#[derive(Debug, Clone)]
pub struct IfElsePass {
    required: Vec<String>,
    condition: Condition,
    then: Vec<AnyPass>,
    otherwise: Vec<AnyPass>,
}

impl IfElsePass {
    pub fn new(
        required: Vec<String>,
        condition: Condition,
        then: Vec<AnyPass>,
        otherwise: Vec<AnyPass>,
    ) -> Self {
        IfElsePass {
            required,
            condition,
            then,
            otherwise,
        }
    }
}

impl BasePass for IfElsePass {
    fn name(&self) -> String {
        String::from("if-else-pass")
    }

    fn requires(&self) -> Vec<String> {
        self.required.clone()
    }
}

impl TransformationPass for IfElsePass {
    fn invalidates(&self) -> &[&str] {
        &[]
    }

    fn run(&self, model: Model, cache: &AnalysisCache) -> TransformationPassResult {
        let is_condition = self
            .condition
            .call(cache)
            .map_err(|err| TransformationPassError(self.name(), err.to_string()))?;
        if is_condition {
            // todo: change once pipelines are available.
            let mapped = self
                .then
                .iter()
                .map(|y| y.clone().as_pass())
                .collect::<PyResult<Vec<_>>>()
                .map_err(|e| TransformationPassError(self.name(), e.to_string()))?;
            let pm = PassManager::new(Some(mapped));
            pm.run(model)
        } else {
            // todo: change once pipelines are available.
            let mapped = self
                .otherwise
                .iter()
                .map(|y| y.clone().as_pass())
                .collect::<PyResult<Vec<_>>>()
                .map_err(|e| TransformationPassError(self.name(), e.to_string()))?;
            let pm = PassManager::new(Some(mapped));
            pm.run(model)
        }
    }

    fn backwards(&self, solution: Solution, cache: &AnalysisCache) -> Solution {
        todo!()
    }
}
