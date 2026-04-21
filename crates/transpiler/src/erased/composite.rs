use std::any::Any;

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use crate::{AnalysisKey, AnalysisManager, CompositePass, ErasedArtifact, PassContext};

/// Object-safe erased composite pass used by the pipeline runtime.
pub trait ErasedCompositePass: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn requires(&self) -> &[String];
    fn provides(&self) -> &str;
    fn invalidates(&self) -> &[String];
    fn forward_erased(
        &self,
        model: &mut Model,
        ctx: &PassContext,
        analyses: &mut AnalysisManager,
    ) -> LunaModelResult<ErasedArtifact>;
    fn display(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

/// Typed pass can be wrapped into ErasedCompositePass.
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

    fn invalidates(&self) -> &[String] {
        self.invalidates()
    }

    fn forward_erased(
        &self,
        model: &mut Model,
        ctx: &PassContext,
        analyses: &mut AnalysisManager,
    ) -> LunaModelResult<ErasedArtifact> {
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
