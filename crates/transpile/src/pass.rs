use lunamodel_core::{Model, Solution};
use lunamodel_error::LunaModelResult;

use crate::{artifact::Artifact, context::PassContext};

/// A reversible transformation pass.
///
/// The forward pass transforms a model and produces an artifact.
/// The artifact encodes everything needed to invert the transformation.
pub trait ReversiblePass: Send + Sync {
    /// The artifact type this pass produces.
    /// This is the "backwards IR" -- it encodes the inverse transformation.
    type Artifact: Artifact;

    /// Unique identifier for this pass.
    fn name(&self) -> &str;

    /// Forward transformation: Model -> TransformedModel + Artifact
    fn forward(&self, model: &mut Model, ctx: &PassContext) -> LunaModelResult<Self::Artifact>;

    /// Inverse transformation: Solution + Artifact -> BackwardTransformedSolution.
    /// All configuration is encoded in the Artifact itself.
    fn backward(artifact: &Self::Artifact, solution: Solution) -> LunaModelResult<Solution>;
}

/// An analysis pass computes information without transforming the model.
pub trait AnalysisPass: Send + Sync {
    /// The type of analysis result this pass produces
    type Result: Send + Sync + 'static;

    /// Unique identifier for this analysis
    fn name(&self) -> &str;

    /// Compute the analysis result
    fn run(&self, model: &Model, ctx: &PassContext) -> LunaModelResult<Self::Result>;

    /// Which analyses does this analysis depend on?
    fn required_analyses(&self) -> &[&'static str] {
        &[]
    }

    /// Is this analysis invalidated by the given pass?
    fn is_invalidated_by(&self, pass_name: &str) -> bool;
}
