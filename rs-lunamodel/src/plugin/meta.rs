//! The plugin-side mirror of the transpiler's `MetaAnalysisPass` trait.

use super::{AnalysisResult, StepList};

/// A meta-analysis pass: computes a result *about the pipeline itself*, authored in safe Rust.
///
/// A 1:1 mirror of `lunamodel_transpiler::MetaAnalysisPass` over the `rs-lunamodel` facade. Unlike
/// an ordinary analysis it reads no model; instead it inspects the steps that follow it (via
/// [`StepList`]) and produces an [`Output`](Self::Output) deposited under [`PROVIDES`](Self::PROVIDES),
/// where later passes read it back by that name. It is not reversible. Implement this and hand the
/// type to the SDK (`descriptor_meta_analysis`, or the `rs-lunamodel` macros) — no `unsafe`, no C
/// types.
pub trait MetaAnalysisPass: Send + Sync {
    /// The result type this meta-analysis produces and consumers decode into.
    type Output: AnalysisResult;

    /// Stable name the result is written under, and read back by consumers.
    const PROVIDES: &'static str;

    /// Human-readable pass name.
    fn name(&self) -> &str;

    /// Computes the result from the read-only view of the steps that follow this pass.
    fn run(&self, steps: &StepList) -> crate::Result<Self::Output>;
}
