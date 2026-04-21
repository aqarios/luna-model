use std::any::Any;

use lunamodel_error::LunaModelResult;

use crate::{AnalysisKey, AnalysisManager, MetaAnalysisPass, PipelineStep};

pub trait ErasedMetaAnalysisPass: Send + Sync {
    fn name(&self) -> &str;
    fn provides(&self) -> &str;
    fn run_erased(
        &self,
        steps: &[PipelineStep],
        analyses: &mut AnalysisManager,
    ) -> LunaModelResult<()>;
    fn display(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

impl<P> ErasedMetaAnalysisPass for P
where
    P: MetaAnalysisPass + Send + Sync + 'static,
{
    fn name(&self) -> &str {
        self.name()
    }

    fn provides(&self) -> &str {
        self.provides()
    }

    fn run_erased(
        &self,
        steps: &[PipelineStep],
        analyses: &mut AnalysisManager,
    ) -> LunaModelResult<()> {
        let value = self.run(steps)?;
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
