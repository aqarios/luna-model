#[cfg(feature = "py")]
use pyo3::prelude::pyclass;

#[cfg_attr(feature = "py", pyclass(name = "PyActionType"))]
#[derive(Clone, Debug)]
pub enum ActionType {
    DidTransform,
    DidAnalysis,
    DidAnalysisTransform,
    DidIfElse,
    DidPipeline,
    DidNothing,
}
