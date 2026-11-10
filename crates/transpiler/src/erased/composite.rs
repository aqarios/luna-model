//! Object-safe runtime adapter for composite passes.

use std::any::Any;

use lunamodel_core::Model;
use lunamodel_types::{EnumSet, ModelPart};

use crate::{
    AnalysisKey, AnalysisManager, CompositePass, ErasedArtifact, PassContext,
    error::TranspileKindResult,
};

/// Object-safe erased composite pass used by the pipeline runtime.
pub trait ErasedCompositePass: Send + Sync {
    /// Stable pass id used for backward registry lookup.
    fn id(&self) -> &str;
    /// Human-readable pass name.
    fn name(&self) -> &str;
    /// Required pass/analysis names.
    fn requires(&self) -> &[String];
    /// Provided analysis key name.
    fn provides(&self) -> &str;
    /// Model parts the pass may modify.
    fn modifies(&self) -> EnumSet<ModelPart>;
    /// Model parts the provided result depends on.
    fn depends_on(&self) -> EnumSet<ModelPart>;
    /// Runs the forward pass and erases the produced artifact.
    fn forward_erased(
        &self,
        model: &mut Model,
        ctx: &PassContext,
        analyses: &mut AnalysisManager,
    ) -> TranspileKindResult<ErasedArtifact>;
    /// Human-readable display string.
    fn display(&self) -> String;
    /// Downcasts to the concrete pass type when needed.
    fn as_any(&self) -> &dyn Any;
}

/// Adapts a typed composite pass to the object-safe runtime interface.
impl<P> ErasedCompositePass for P
where
    P: CompositePass + Send + Sync + 'static,
{
    fn id(&self) -> &str {
        P::ID
    }

    fn name(&self) -> &str {
        self.name()
    }

    fn requires(&self) -> &[String] {
        self.requires()
    }

    fn provides(&self) -> &str {
        self.provides()
    }

    fn modifies(&self) -> EnumSet<ModelPart> {
        self.modifies()
    }

    fn depends_on(&self) -> EnumSet<ModelPart> {
        self.depends_on()
    }

    fn forward_erased(
        &self,
        model: &mut Model,
        ctx: &PassContext,
        analyses: &mut AnalysisManager,
    ) -> TranspileKindResult<ErasedArtifact> {
        let (artifact, result) = self.forward(model, ctx)?;
        let key = AnalysisKey::<P::Result>::new(self.provides().into());
        analyses.insert(&key, result);
        ErasedArtifact::new(&artifact)
    }

    fn display(&self) -> String {
        self.display()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
