use lunamodel_core::Solution;
use lunamodel_error::LunaModelResult;

use crate::Artifact;

/// Trait for passes that can invert their forward transformation on solutions.
pub trait Reversible {
    /// The artifact type this pass produces.
    /// This is the "backwards IR" -- it encodes the inverse transformation.
    type Artifact: Artifact;

    /// Unique identifier for this pass.
    const ID: &'static str;

    /// Applies the inverse transformation to a solution using the stored artifact.
    ///
    /// All configuration needed for reversal must already be encoded in the
    /// artifact produced during forward execution.
    fn backward(artifact: &Self::Artifact, solution: Solution) -> LunaModelResult<Solution>;
}
