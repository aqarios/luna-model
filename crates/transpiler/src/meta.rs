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

    /// Returns the typed key associated with the analysis result this pass provides.
    fn key<T>() -> AnalysisKey<T>;

    /// Computes the analysis result from the remaining pipeline steps.
    fn run(&self, steps: &[PipelineStep]) -> LunaModelResult<Self::Result>;

    /// Human-readable display string used by pipeline visualization.
    fn display(&self) -> String {
        format!("🔭 {}", self.name())
    }
}
