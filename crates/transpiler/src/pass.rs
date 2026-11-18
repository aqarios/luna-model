//! Trait definitions for reversible transformation and analysis passes.

use lunamodel_core::Model;
use lunamodel_types::{EnumSet, ModelPart};

use crate::{AnalysisKey, Reversible, context::PassContext, error::TranspileKindResult};

/// A reversible transformation pass.
///
/// The forward pass transforms a model and produces an artifact.
/// The artifact encodes everything needed to invert the transformation.
pub trait TransformationPass: Send + Sync + Reversible {
    /// Name for this pass.
    fn name(&self) -> &str;

    /// Runs the forward transformation and returns the artifact needed for reversal.
    fn forward(&self, model: &mut Model, ctx: &PassContext) -> TranspileKindResult<Self::Artifact>;

    /// Which pass/analysis keys must be satisfied before this pass can execute?
    fn requires(&self) -> &[String] {
        &[]
    }

    /// Which model parts this pass may modify. Analyses depending on any of them are invalidated.
    fn modifies(&self) -> EnumSet<ModelPart> {
        EnumSet::all()
    }

    /// Human-readable display string used by pipeline visualization.
    fn display(&self) -> String {
        format!("⚙️ {}", self.name())
    }
}

/// An analysis pass computes information without transforming the model.
pub trait AnalysisPass: Send + Sync {
    /// The type of analysis result this pass produces
    type Result: Send + Sync + 'static;

    const PROVIDES: &'static str;

    /// Unique identifier for this analysis
    fn name(&self) -> &str;

    /// Stable key this analysis writes to in the `AnalysisManager`.
    fn provides(&self) -> &str {
        Self::PROVIDES
    }

    /// Returns the typed key associated with the analysis result this pass provides.
    fn key<T>() -> AnalysisKey<T>;

    /// Which pass/analysis keys must be satisfied before this analysis can execute?
    fn requires(&self) -> &[String] {
        &[]
    }

    /// Which model parts the result is computed from. It stays valid until one of them is modified.
    fn depends_on(&self) -> EnumSet<ModelPart> {
        EnumSet::all()
    }

    /// Computes the analysis result without mutating the model.
    fn run(&self, model: &Model, ctx: &PassContext) -> TranspileKindResult<Self::Result>;

    /// Human-readable display string used by pipeline visualization.
    fn display(&self) -> String {
        format!("🔎 {}", self.name())
    }
}
