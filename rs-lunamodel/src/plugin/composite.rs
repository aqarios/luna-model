//! The plugin-side mirror of the transpiler's `CompositePass` trait.

use crate::Model;

use super::{AnalysisResult, PassContext, Reversible};

/// A composite pass: a fused transformation **and** analysis, authored in safe Rust.
///
/// A 1:1 mirror of `lunamodel_transpiler::CompositePass` over the `rs-lunamodel` facade. A single
/// `forward` both mutates the model — producing the [`Artifact`](Reversible::Artifact) needed for
/// reversal (via the [`Reversible`] super-trait) — and computes an analysis [`Output`](Self::Output)
/// deposited under [`PROVIDES`](Self::PROVIDES), where later passes read it back by that name
/// (including passes from a *different* plugin, as long as they share the [`Output`](Self::Output)
/// type). Implement this and hand the type to the SDK (`descriptor_composite` /
/// `register_composite_backward`, or the `rs-lunamodel` macros) — no `unsafe`, no C types.
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

    /// Analysis names invalidated after this pass runs.
    fn invalidates(&self) -> &[String] {
        &[]
    }

    /// Runs the fused forward: mutates the model and returns the inverse [`Artifact`](Reversible::Artifact)
    /// plus the analysis [`Output`](Self::Output).
    fn forward(
        &self,
        model: &mut Model,
        ctx: &PassContext,
    ) -> crate::Result<(Self::Artifact, Self::Output)>;
}
