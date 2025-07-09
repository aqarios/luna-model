use aqm_macros::analysis_cache;
use global_counter::primitive::exact::CounterU64;

#[cfg(feature = "py")]
use {
    crate::transformations::analysis_cache::PyAnalysisCache, pyo3::exceptions::PyTypeError,
    pyo3::prelude::*, pyo3::IntoPyObjectExt,
};

use super::pipeline::Pipeline;
use crate::transformations::analysis_cache::AnalysisCache;
use crate::transformations::base_passes::BasePass;
use crate::transformations::{
    errors::CompilationError, intermediate_representation::IntermediateRepresentation,
};
use crate::{core::Model, transformations::errors::IfElsePassError};
use crate::{core::Solution, transformations::analysis_cache::AnalysisCacheElement};

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
    fulfilled_condition: bool,
}

pub struct IfElseOutcome {
    pub ir: IntermediateRepresentation,
    pub analysis: AnalysisCacheElement,
}

pub type IfElsePassResult = Result<IfElseOutcome, IfElsePassError>;

/// Counter to ensure multiple if-else branches can be used in the same pass.
pub static IF_ELSE_COUNTER: CounterU64 = CounterU64::new(0);

// #[cfg_attr(feature = "py", py_pass(pass_variant = "IfElse"))]
#[derive(Debug, Clone)]
pub struct IfElsePass {
    required: Vec<String>,
    condition: Condition,
    then: Pipeline,
    otherwise: Pipeline,
    // #[py_pass(init_ignore)]
    name: String,
}

impl IfElsePass {
    pub fn new(
        required: Vec<String>,
        condition: Condition,
        then: Pipeline,
        otherwise: Pipeline,
    ) -> Self {
        IfElsePass {
            required,
            condition,
            then,
            otherwise,
            name: format!("if-else-{}", IF_ELSE_COUNTER.inc()),
        }
    }
}

impl BasePass for IfElsePass {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn requires(&self) -> Vec<String> {
        self.required.clone()
    }
}

impl IfElsePass {
    pub fn run(&self, model: Model, cache: &AnalysisCache) -> IfElsePassResult {
        let is_condition = self
            .condition
            .call(cache)
            .map_err(|err| IfElsePassError(err.to_string()))?;
        let ir = if is_condition {
            self.then
                .run(model, &cache)
                .map_err(|err| IfElsePassError(err.to_string()))
        } else {
            self.otherwise
                .run(model, &cache)
                .map_err(|err| IfElsePassError(err.to_string()))
        }?;
        Ok(IfElseOutcome {
            ir,
            analysis: AnalysisCacheElement::IfElseInfoAnalysis(IfElseInfo {
                fulfilled_condition: is_condition,
            }),
        })
    }

    pub fn backwards(&self, mut solution: Solution, ir: &IntermediateRepresentation) -> Solution {
        match ir.cache.get(&self.name) {
            Some(AnalysisCacheElement::IfElseInfoAnalysis(cache)) => {
                if cache.fulfilled_condition {
                    solution = self.then.backwards(solution, ir)
                } else {
                    solution = self.otherwise.backwards(solution, ir)
                }
            }
            _ => {}
        }
        solution
    }
}
