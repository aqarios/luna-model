//! Object-safe runtime adapter for meta-analysis passes.

use std::any::Any;

use crate::{
    AnalysisKey, AnalysisManager, MetaAnalysisPass, PipelineStep, error::TranspileKindResult,
};

/// Object-safe wrapper for [`crate::MetaAnalysisPass`].
pub trait ErasedMetaAnalysisPass: Send + Sync {
    /// Pass name.
    fn name(&self) -> &str;
    /// Provided analysis key name.
    fn provides(&self) -> &str;
    /// Runs the pass and stores the result in the provided analysis manager.
    fn run_erased(
        &self,
        steps: &[PipelineStep],
        analyses: &mut AnalysisManager,
    ) -> TranspileKindResult<()>;
    /// Human-readable display string.
    fn display(&self) -> String;
    /// Downcasts to the concrete pass type when needed.
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
    ) -> TranspileKindResult<()> {
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
