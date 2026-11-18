//! The plugin-side mirror of the transpiler's `Reversible` trait.

use crate::Solution;

use super::Artifact;

/// A pass that can invert its forward transformation on solutions.
///
/// A 1:1 mirror of `lunamodel_transpiler::Reversible`. `backward` is an associated function (no
/// `self`): reversal can run after the pass instance is gone or when only a record was reloaded,
/// so everything it needs must be encoded in the [`Artifact`].
pub trait Reversible {
    /// The artifact type this pass produces during `forward`.
    type Artifact: Artifact;

    /// Unique, stable identifier for this pass (e.g. `"myplugin::relabel"`).
    const ID: &'static str;

    /// Applies the inverse transformation to a solution using the stored artifact.
    fn backward(artifact: &Self::Artifact, solution: Solution) -> crate::Result<Solution>;
}
