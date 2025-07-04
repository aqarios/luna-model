use aqm_macros::analysis_cache;

#[cfg(feature = "py")]
use {
    crate::transformations::analysis_cache::PyAnalysisCache, pyo3::exceptions::PyTypeError,
    pyo3::prelude::*, pyo3::IntoPyObjectExt,
};

use crate::core::Model;
use crate::core::Solution;
use crate::transformations::analysis_cache::AnalysisCache;
use crate::transformations::base_passes::{
    BasePass, Pass, TransformationPassResult,
};
use crate::transformations::errors::CompilationError;
use crate::transformations::errors::TransformationPassError;

pub type RustCallback = fn(&AnalysisCache) -> bool;

#[cfg(feature = "py")]
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

#[derive(Debug, Clone)]
#[analysis_cache]
pub struct IfElseInfo {
    fulfilled_condition: bool
}

// #[cfg_attr(feature = "py", py_pass(pass_variant = "Transformation"))]
#[derive(Debug)]
pub struct IfElsePass {
    required: Vec<String>,
    condition: Condition,
    then: Vec<Pass>,
    otherwise: Vec<Pass>,
}

#[cfg(feature = "py")]
impl IfElsePass {
    pub fn as_pass(self) -> PyResult<Pass> {
        Ok(Pass::IfElse(self))
    }
}

impl IfElsePass {
    pub fn new(
        required: Vec<String>,
        condition: Condition,
        then: Vec<Pass>,
        otherwise: Vec<Pass>,
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



impl IfElsePass {
    pub fn invalidates(&self) -> Vec<String> {
        Vec::default()
    }

    pub fn run(&self, _model: Model, cache: &AnalysisCache) -> TransformationPassResult {
        let is_condition = self
            .condition
            .call(cache)
            .map_err(|err| TransformationPassError(self.name(), err.to_string()))?;
        if is_condition {
            // todo: change once pipelines are available.
            // let mapped = self
            //     .then
            //     .iter()
            //     // .map(|y| y.clone().as_pass())
            //     // .collect::<PyResult<Vec<_>>>()
            //     .map_err(|e| TransformationPassError(self.name(), e.to_string()))?;
            // let pm = PassManager::new(Some(self.then));
            // pm.run(model)
            todo!()
        } else {
            // todo: change once pipelines are available.
            // let mapped = self
            //     .otherwise
            //     .iter()
            //     .map(|y| y.clone().as_pass())
            //     .collect::<PyResult<Vec<_>>>()
            //     .map_err(|e| TransformationPassError(self.name(), e.to_string()))?;
            // let pm = PassManager::new(Some(self.otherwise));
            // pm.run(model)
            todo!()
        }
    }

    pub fn backwards(&self, _solution: Solution, _cache: &AnalysisCache) -> Solution {
        todo!()
    }
}
