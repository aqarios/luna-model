use std::any::Any;

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use crate::{AnalysisKey, AnalysisManager, AnalysisPass, PassContext};

pub trait ErasedAnalysisPass: Send + Sync {
    fn name(&self) -> &str;
    fn provides(&self) -> &str;
    fn requires(&self) -> &[String];
    fn run_erased(
        &self,
        model: &Model,
        ctx: &PassContext,
        analyses: &mut AnalysisManager,
    ) -> LunaModelResult<()>;
    fn display(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

impl<P> ErasedAnalysisPass for P
where
    P: AnalysisPass + Send + Sync + 'static,
{
    fn name(&self) -> &str {
        self.name()
    }

    fn provides(&self) -> &str {
        self.provides()
    }

    fn requires(&self) -> &[String] {
        self.requires()
    }

    fn run_erased(
        &self,
        model: &Model,
        ctx: &PassContext,
        analyses: &mut AnalysisManager,
    ) -> LunaModelResult<()> {
        let value = self.run(model, ctx)?;
        let key = AnalysisKey::<P::Result>::new(self.provides().into());
        analyses.insert(&key, value);
        Ok(())
    }

    fn display(&self) -> String {
        self.display()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
