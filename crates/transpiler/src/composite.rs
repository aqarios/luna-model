use lunamodel_core::{Model, Solution};
use lunamodel_error::LunaModelResult;

use crate::{AnalysisKey, Artifact, PassContext};

/// A composite pass.
///
/// The forward pass generates an artifact and a analysis result.
pub trait CompositePass: Send + Sync {
    /// The artifact type this pass produces.
    /// This is the "backwards IR" -- it encodes the inverse transformation.
    type Artifact: Artifact;
    /// The type of analysis result this pass produces
    type Result: Send + Sync + 'static;

    const NAME: &'static str;
    const PROVIDES: &'static str;
    /// Unique identifier for this pass.
    const ID: &'static str;

    /// Name for this pass.
    fn name(&self) -> &str {
        Self::NAME
    }

    /// Forward composite: Model + PassContext -> TransformedModel + (Artifact, Result)
    fn forward(
        &self,
        model: &mut Model,
        ctx: &PassContext,
    ) -> LunaModelResult<(Self::Artifact, Self::Result)>;

    /// Inverse transformation: Solution + Artifact -> BackwardTransformedSolution.
    /// All configuration is encoded in the Artifact itself.
    fn backward(artifact: &Self::Artifact, solution: Solution) -> LunaModelResult<Solution>;

    /// Which pass/analysis keys must be satisfied before this pass can execute?
    fn requires(&self) -> &[String] {
        &[]
    }

    /// Stable key this analysis writes to in the `AnalysisManager`.
    fn provides(&self) -> &str {
        Self::PROVIDES
    }
    fn key<T>() -> AnalysisKey<T>;

    /// Which pass/analysis keys might be invalidated after this pass was executed.
    fn invalidates(&self) -> &[String] {
        &[]
    }

    /// Overridable to_string method for displaying the pass as human readble.
    fn display(&self) -> String {
        format!("🧩 {}", self.name())
    }
}
