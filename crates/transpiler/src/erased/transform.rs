use std::any::Any;

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use crate::{ErasedArtifact, PassContext, TransformationPass};

/// Object-safe erased transform pass used by the pipeline runtime.
pub trait ErasedTransformPass: Send + Sync {
    /// Stable pass id used for backward registry lookup.
    fn id(&self) -> &str;
    /// Human-readable pass name.
    fn name(&self) -> &str;
    /// Required pass/analysis names.
    fn requires(&self) -> &[String];
    /// Invalidated analysis names.
    fn invalidates(&self) -> &[String];
    /// Runs the forward pass and erases the produced artifact.
    fn forward_erased(
        &self,
        model: &mut Model,
        ctx: &PassContext,
    ) -> LunaModelResult<ErasedArtifact>;
    /// Human-readable display string.
    fn display(&self) -> String;
    /// Downcasts to the concrete pass type when needed.
    fn as_any(&self) -> &dyn Any;
}

/// Adapts a typed transformation pass to the object-safe runtime interface.
impl<P> ErasedTransformPass for P
where
    P: TransformationPass + Send + Sync + 'static,
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

    fn invalidates(&self) -> &[String] {
        self.invalidates()
    }

    fn forward_erased(
        &self,
        model: &mut Model,
        ctx: &PassContext,
    ) -> LunaModelResult<ErasedArtifact> {
        let artifact = self.forward(model, ctx)?;
        ErasedArtifact::new(&artifact)
    }

    fn display(&self) -> String {
        self.display()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
