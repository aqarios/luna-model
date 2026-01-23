#[cfg(feature = "py")]
use pyo3::prelude::pyclass;

#[cfg_attr(
    feature = "py",
    pyclass(name = "ActionType", module = "luna_model._core")
)]
#[derive(Clone, Debug)]
pub enum ActionType {
    DidTransform,
    DidAnalysis,
    DidAnalysisTransform,
    DidIfElse,
    DidPipeline,
    DidNothing,
}
