//! The plugin-side mirror of the transpiler's `CompositePass` trait.

use crate::Model;
use crate::types::{EnumSet, ModelPart};

use super::{AnalysisResult, PassContext, Reversible};

/// A composite pass: a fused transformation **and** analysis, authored in safe Rust.
///
/// A 1:1 mirror of `lunamodel_transpiler::CompositePass` over the `luna-model-ffi` facade. A single
/// `forward` both mutates the model — producing the [`Artifact`](Reversible::Artifact) needed for
/// reversal (via the [`Reversible`] super-trait) — and computes an analysis [`Output`](Self::Output)
/// deposited under [`PROVIDES`](Self::PROVIDES), where later passes read it back by that name
/// (including passes from a *different* plugin, as long as they share the [`Output`](Self::Output)
/// type). Implement this and hand the type to the SDK (`descriptor_composite` /
/// `register_composite_backward`, or the `luna-model-ffi` macros) — no `unsafe`, no C types.
pub trait CompositePass: Send + Sync + Reversible {
    /// The analysis result this pass produces and consumers decode into.
    type Output: AnalysisResult;

    /// Stable name the result is written under, and read back by consumers
    /// (e.g. `"myplugin::coeff-widths"`).
    const PROVIDES: &'static str;

    /// Human-readable pass name.
    fn name(&self) -> &str;

    /// Analysis names that must be satisfied before this pass runs.
    fn requires(&self) -> &[String] {
        &[]
    }

    /// Model parts this pass may modify. Analyses depending on any of them are invalidated after it
    /// runs. Defaults to every part.
    fn modifies(&self) -> EnumSet<ModelPart> {
        EnumSet::all()
    }

    /// Model parts the result is computed from. It stays valid until a transformation modifies one
    /// of them. Defaults to every part.
    fn depends_on(&self) -> EnumSet<ModelPart> {
        EnumSet::all()
    }

    /// Runs the fused forward: mutates the model and returns the inverse [`Artifact`](Reversible::Artifact)
    /// plus the analysis [`Output`](Self::Output).
    fn forward(
        &self,
        model: &mut Model,
        ctx: &PassContext,
    ) -> crate::Result<(Self::Artifact, Self::Output)>;
}
