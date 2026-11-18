//! Trait definitions for composite passes.

use lunamodel_core::Model;
use lunamodel_types::{EnumSet, ModelPart};

use crate::{AnalysisKey, PassContext, error::TranspileKindResult, reversible::Reversible};

/// A composite pass.
///
/// The forward pass generates an artifact and a analysis result.
pub trait CompositePass: Send + Sync + Reversible {
    /// The type of analysis result this pass produces
    type Result: Send + Sync + 'static;

    const NAME: &'static str;
    const PROVIDES: &'static str;
    /// Name for this pass.
    fn name(&self) -> &str {
        Self::NAME
    }

    /// Forward composite: Model + PassContext -> TransformedModel + (Artifact, Result)
    fn forward(
        &self,
        model: &mut Model,
        ctx: &PassContext,
    ) -> TranspileKindResult<(Self::Artifact, Self::Result)>;

    /// Which pass/analysis keys must be satisfied before this pass can execute?
    fn requires(&self) -> &[String] {
        &[]
    }

    /// Stable key this analysis writes to in the `AnalysisManager`.
    fn provides(&self) -> &str {
        Self::PROVIDES
    }
    fn key<T>() -> AnalysisKey<T>;

    /// Which model parts this pass may modify. Analyses depending on any of them are invalidated.
    fn modifies(&self) -> EnumSet<ModelPart> {
        EnumSet::all()
    }

    /// Which model parts the provided result is computed from.
    fn depends_on(&self) -> EnumSet<ModelPart> {
        EnumSet::all()
    }

    /// Overridable to_string method for displaying the pass as human readble.
    fn display(&self) -> String {
        format!("🧩 {}", self.name())
    }
}
