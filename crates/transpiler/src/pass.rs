use lunamodel_core::{Model, Solution};
use lunamodel_error::LunaModelResult;

use crate::{AnalysisKey, artifact::Artifact, context::PassContext};

/// A reversible transformation pass.
///
/// The forward pass transforms a model and produces an artifact.
/// The artifact encodes everything needed to invert the transformation.
pub trait ReversiblePass: Send + Sync {
    /// The artifact type this pass produces.
    /// This is the "backwards IR" -- it encodes the inverse transformation.
    type Artifact: Artifact;

    /// Unique identifier for this pass.
    const ID: &'static str;

    /// Name for this pass.
    fn name(&self) -> &str;

    /// Forward transformation: Model -> TransformedModel + Artifact
    fn forward(&self, model: &mut Model, ctx: &PassContext) -> LunaModelResult<Self::Artifact>;

    /// Inverse transformation: Solution + Artifact -> BackwardTransformedSolution.
    /// All configuration is encoded in the Artifact itself.
    fn backward(artifact: &Self::Artifact, solution: Solution) -> LunaModelResult<Solution>;

    /// Which pass/analysis keys must be satisfied before this pass can execute?
    fn requires(&self) -> &[String] {
        &[]
    }

    /// Which pass/analysis keys are invalidated after this pass was executed.
    fn invalidates(&self) -> &[String] {
        &[]
    }

    /// Overridable to_string method for displaying the pass as human readble.
    fn display(&self) -> String {
        format!("⚙️ {}", self.name())
    }
}

/// An analysis pass computes information without transforming the model.
pub trait AnalysisPass: Send + Sync {
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

    /// Which pass/analysis keys must be satisfied before this analysis can execute?
    fn requires(&self) -> &[String] {
        &[]
    }

    /// Compute the analysis result
    fn run(&self, model: &Model, ctx: &PassContext) -> LunaModelResult<Self::Result>;

    /// Overridable to_string method for displaying the pass as human readble.
    fn display(&self) -> String {
        format!("🔎 {}", self.name())
    }
}
