use lunamodel_error::LunaModelResult;

use crate::{AnalysisKey, PipelineStep};

/// A meta analysis pass computes information about the pipeline steps following this pass.
pub trait MetaAnalysisPass: Send + Sync {
    /// The type of analysis result this pass produces
    type Result: Send + Sync + 'static;

    const NAME: &'static str;
    const PROVIDES: &'static str;

    /// Unique identifier for this analysis
    fn name(&self) -> &str {
        Self::NAME
    }

    /// Stable key this analysis writes to in the `AnalysisManager`.
    fn provides(&self) -> &str {
        Self::PROVIDES
    }

    fn key<T>() -> AnalysisKey<T>;

    /// Compute the analysis result
    fn run(&self, steps: &[PipelineStep]) -> LunaModelResult<Self::Result>;

    /// Overridable to_string method for displaying the pass as human readble.
    fn display(&self) -> String {
        format!("🔭 {}", self.name())
    }
}
