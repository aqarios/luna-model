use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use crate::{AnalysisKey, Reversible, context::PassContext};

/// A reversible transformation pass.
///
/// The forward pass transforms a model and produces an artifact.
/// The artifact encodes everything needed to invert the transformation.
pub trait TransformationPass: Send + Sync + Reversible {
    /// Name for this pass.
    fn name(&self) -> &str;

    /// Runs the forward transformation and returns the artifact needed for reversal.
    fn forward(&self, model: &mut Model, ctx: &PassContext) -> LunaModelResult<Self::Artifact>;

    /// Which pass/analysis keys must be satisfied before this pass can execute?
    fn requires(&self) -> &[String] {
        &[]
    }

    /// Which pass/analysis keys are invalidated after this pass was executed.
    fn invalidates(&self) -> &[String] {
        &[]
    }

    /// Human-readable display string used by pipeline visualization.
    fn display(&self) -> String {
        format!("⚙️ {}", self.name())
    }
}

/// An analysis pass computes information without transforming the model.
pub trait AnalysisPass: Send + Sync {
    /// The type of analysis result this pass produces
    type Result: Send + Sync + 'static;

    const PROVIDES: &'static str;

    /// Unique identifier for this analysis
    fn name(&self) -> &str;

    /// Stable key this analysis writes to in the `AnalysisManager`.
    fn provides(&self) -> &str {
        Self::PROVIDES
    }

    /// Returns the typed key associated with the analysis result this pass provides.
    fn key<T>() -> AnalysisKey<T>;

    /// Which pass/analysis keys must be satisfied before this analysis can execute?
    fn requires(&self) -> &[String] {
        &[]
    }

    /// Computes the analysis result without mutating the model.
    fn run(&self, model: &Model, ctx: &PassContext) -> LunaModelResult<Self::Result>;

    /// Human-readable display string used by pipeline visualization.
    fn display(&self) -> String {
        format!("🔎 {}", self.name())
    }
}
