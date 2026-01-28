use strum_macros::{Display, EnumString};

#[cfg(feature = "py")]
use pyo3::prelude::pyclass;

#[derive(Display, Copy, PartialEq, Clone, Debug, Eq, EnumString)]
#[cfg_attr(feature = "py", pyclass(eq, eq_int, name = "PyActionType"))]
pub enum ActionType {
    DidTransform,
    DidAnalysis,
    DidAnalysisTransform,
    DidIfElse,
    DidPipeline,
    DidNothing,
}
