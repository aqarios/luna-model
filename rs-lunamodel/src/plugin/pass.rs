//! The plugin-side mirror of the transpiler's `TransformationPass` trait.

use crate::Model;
use crate::types::{EnumSet, ModelPart};

use super::{PassContext, Reversible};

/// A reversible transformation pass, authored in safe Rust.
///
/// A 1:1 mirror of `lunamodel_transpiler::TransformationPass`: `forward` mutates the model and
/// produces the [`Artifact`](Reversible::Artifact); reversal is provided by the [`Reversible`]
/// super-trait. Implement this and hand the type to the SDK (`descriptor` / `register_backward`,
/// or the `luna-model-ffi` macros) — no `unsafe`, no C types.
pub trait TransformationPass: Send + Sync + Reversible {
    /// Name for this pass.
    fn name(&self) -> &str;

    /// Runs the forward transformation and returns the artifact needed for reversal.
    fn forward(&self, model: &mut Model, ctx: &PassContext) -> crate::Result<Self::Artifact>;

    /// Analysis names that must be satisfied before this pass runs.
    fn requires(&self) -> &[String] {
        &[]
    }

    /// Model parts this pass may modify. Analyses depending on any of them are invalidated after it
    /// runs. Defaults to every part.
    fn modifies(&self) -> EnumSet<ModelPart> {
        EnumSet::all()
    }
}
