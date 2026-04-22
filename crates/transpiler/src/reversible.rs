use lunamodel_core::Solution;
use lunamodel_error::LunaModelResult;

use crate::Artifact;

pub trait Reversible {
    /// The artifact type this pass produces.
    /// This is the "backwards IR" -- it encodes the inverse transformation.
    type Artifact: Artifact;

    /// Unique identifier for this pass.
    const ID: &'static str;

    /// Inverse transformation: Solution + Artifact -> BackwardTransformedSolution.
    /// All configuration is encoded in the Artifact itself.
    fn backward(artifact: &Self::Artifact, solution: Solution) -> LunaModelResult<Solution>;
}
