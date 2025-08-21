use crate::{
    transformations::{intermediate_representation::ExecutionLog, pass_manager::PassManager},
    unicode::{BALLOT_X, CHECK_MARK, D_AND_L, H_BAR, U_AND_R, V_AND_R},
};
use std::fmt::Display;

use aqm_macros::analysis_cache;
use global_counter::primitive::exact::CounterU64;
use pad::PadStr;

#[cfg(feature = "py")]
use {
    crate::transformations::analysis_cache::PyAnalysisCache, pyo3::exceptions::PyTypeError,
    pyo3::prelude::*, pyo3::IntoPyObjectExt,
};

use super::pipeline::AbstractPipeline;
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
#[cfg_attr(feature = "py", pymethods)]
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
    requires: Vec<String>,
    condition: Condition,
    then: Box<dyn AbstractPipeline>,
    otherwise: Box<dyn AbstractPipeline>,
    // #[py_pass(init_ignore)]
    name: String,
}

impl IfElsePass {
    pub fn new(
        requires: Vec<String>,
        condition: Condition,
        then: Box<dyn AbstractPipeline>,
        otherwise: Box<dyn AbstractPipeline>,
        name: Option<String>,
    ) -> Self {
        let mut requires = requires;
        requires.append(&mut then.requires());
        requires.append(&mut otherwise.requires());
        IfElsePass {
            requires,
            condition,
            then,
            otherwise,
            name: name.unwrap_or_else(|| format!("if-else-{}", IF_ELSE_COUNTER.inc())),
        }
    }
}

impl BasePass for IfElsePass {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn requires(&self) -> Vec<String> {
        self.requires.clone()
    }
}

// impl IntoAnyPass for IfElsePass {}

impl IfElsePass {
    pub fn run(
        &self,
        model: Model,
        cache: &AnalysisCache,
        executor: &PassManager,
    ) -> IfElsePassResult {
        let is_condition = self
            .condition
            .call(cache)
            .map_err(|err| IfElsePassError(err.to_string()))?;
        let ir = if is_condition {
            self.then
                .run(model, &cache, executor)
                .map_err(|err| IfElsePassError(err.to_string()))
        } else {
            self.otherwise
                .run(model, &cache, executor)
                .map_err(|err| IfElsePassError(err.to_string()))
        }?;
        Ok(IfElseOutcome {
            ir,
            analysis: AnalysisCacheElement::IfElseInfoAnalysis(IfElseInfo {
                fulfilled_condition: is_condition,
            }),
        })
    }

    pub fn backwards(
        &self,
        mut solution: Solution,
        ir: &IntermediateRepresentation,
        log: &ExecutionLog,
    ) -> Solution {
        match ir.cache.get(&self.name) {
            Some(AnalysisCacheElement::IfElseInfoAnalysis(cache)) => {
                if cache.fulfilled_condition {
                    solution = self.then.backwards(solution, ir, log)
                } else {
                    solution = self.otherwise.backwards(solution, ir, log)
                }
            }
            _ => {}
        }
        solution
    }
}

impl Display for IfElsePass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if (self.then.len() == 0) && (self.otherwise.len() == 0) {
            write!(f, "❔ {} (empty)", self.name)?;
            return Ok(());
        }

        let str_then = self.then.content_string();
        let str_otherwise = self.otherwise.content_string();

        let mut then: Vec<_> = str_then.split("\n").collect();
        let mut otherwise: Vec<_> = str_otherwise.split("\n").collect();

        let maybe_then_width_max = then.iter().max_by(|a, b| a.len().cmp(&b.len()));
        if maybe_then_width_max.is_none() {
            return write!(f, "");
        }
        let target_width = maybe_then_width_max.unwrap().len();

        if then.len() > otherwise.len() {
            otherwise.resize(then.len(), "");
        } else if then.len() < otherwise.len() {
            then.resize(otherwise.len(), "");
        }

        let final_then: Vec<_> = then.iter().map(|s| s.pad_to_width(target_width)).collect();
        let final_otherwise: Vec<_> = otherwise.iter().map(|s| s.to_string()).collect();

        let title_then = format!("{CHECK_MARK}  {}", self.then.name())
            .pad_to_width_with_alignment(target_width - 1, pad::Alignment::Left);
        let title_otherwise = format!("{BALLOT_X} {}", self.otherwise.name())
            .pad_to_width_with_alignment(target_width, pad::Alignment::Left);

        let ext_then = format!("{U_AND_R} {title_then}");
        let ext_a_else = format!(
            "{}{D_AND_L}",
            H_BAR.repeat(target_width - self.name.len() + 6)
        );
        let ext_b_else = format!("  {title_otherwise}");

        write!(f, "❔ {} {ext_a_else}\n", self.name)?;
        write!(f, "   {ext_then}   {ext_b_else}\n")?;
        for (i, (t, o)) in final_then.iter().zip(&final_otherwise).enumerate() {
            let end = if i < final_then.len() - 1 { "\n" } else { "" };
            let limiter_a = match (t.is_empty(), &final_then.get(i + 1)) {
                (false, Some(x)) => {
                    if x.is_empty() {
                        U_AND_R
                    } else {
                        V_AND_R
                    }
                }
                (false, None) => U_AND_R,
                (true, None) => "",
                _ => "",
            };
            let limiter_b = match (o.is_empty(), &final_otherwise.get(i + 1)) {
                (false, Some(x)) => {
                    if x.is_empty() {
                        U_AND_R
                    } else {
                        V_AND_R
                    }
                }
                (false, None) => U_AND_R,
                (true, None) => "",
                _ => "",
            };
            write!(f, "        {limiter_a} {t}  {limiter_b} {o}{end}")?;
        }
        Ok(())
    }
}
