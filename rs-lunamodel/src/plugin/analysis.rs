//! The plugin-side mirror of the transpiler's analysis pass and its result.

use prost::Message;

use crate::Model;

use super::PassContext;

/// A result produced by a plugin analysis pass and read back by later passes through
/// [`PassContext`](super::PassContext).
///
/// A native analysis result is any `Send + Sync` value handed back by reference; a plugin's result
/// instead crosses the C ABI as bytes — serialized when the analysis deposits it, and deserialized
/// by each consumer into its own copy. `type_tag` guards that decode so a consumer only decodes a
/// result produced by the matching type (including one plugin reading another plugin's result).
///
/// The shape mirrors [`Artifact`](super::Artifact); a type used both as a transformation artifact
/// and as an analysis result can implement both with the same body.
pub trait AnalysisResult: Message + Default + Send + Sync + 'static {
    /// Unique type identifier for this result type.
    fn static_type_tag() -> &'static str;

    /// Unique type identifier for this result value.
    fn type_tag(&self) -> &'static str {
        Self::static_type_tag()
    }
}

/// An analysis pass: computes a result from a model without transforming it.
///
/// A 1:1 mirror of `lunamodel_transpiler::AnalysisPass` over the `luna-model-ffi` facade. The result
/// is deposited under [`PROVIDES`](Self::PROVIDES) in the run's analysis store, where later passes
/// read it back by that name with [`PassContext::require_analysis`](super::PassContext::require_analysis)
/// — including passes from a *different* plugin, as long as they share the [`Output`](Self::Output)
/// type. Authored in safe Rust; the SDK generates all C glue via [`descriptor_analysis`].
///
/// [`descriptor_analysis`]: super::descriptor_analysis
pub trait AnalysisPass: Send + Sync {
    /// The result type this analysis produces and consumers decode into.
    type Output: AnalysisResult;

    /// Unique, stable identifier for this pass (e.g. `"myplugin::coeff-widths-analysis"`).
    const ID: &'static str;

    /// Stable name the result is written under, and read back by consumers
    /// (e.g. `"myplugin::coeff-widths"`).
    const PROVIDES: &'static str;

    /// Human-readable pass name.
    fn name(&self) -> &str;

    /// Analysis names that must be satisfied before this pass runs.
    fn requires(&self) -> &[String] {
        &[]
    }

    /// Computes the analysis result from the model without mutating it.
    fn run(&self, model: &Model, ctx: &PassContext) -> crate::Result<Self::Output>;
}
