use std::any::Any;

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use crate::{ErasedArtifact, PassContext, TransformationPass};

/// Object-safe erased transform pass used by the pipeline runtime.
pub trait ErasedTransformPass: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn requires(&self) -> &[String];
    fn invalidates(&self) -> &[String];
    fn forward_erased(
        &self,
        model: &mut Model,
        ctx: &PassContext,
    ) -> LunaModelResult<ErasedArtifact>;
    fn display(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

/// Typed pass can be wrapped into ErasedTransformPass.
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
