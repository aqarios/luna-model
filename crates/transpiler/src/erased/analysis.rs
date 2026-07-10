//! Object-safe runtime adapter for analysis passes.

use std::any::Any;

use lunamodel_core::Model;

use crate::{AnalysisKey, AnalysisManager, AnalysisPass, PassContext, error::TranspileKindResult};

/// Object-safe wrapper for [`crate::AnalysisPass`].
pub trait ErasedAnalysisPass: Send + Sync {
    /// Pass name.
    fn name(&self) -> &str;
    /// Provided analysis key name.
    fn provides(&self) -> &str;
    /// Required pass/analysis names.
    fn requires(&self) -> &[String];
    /// Runs the pass and stores the result in the provided analysis manager.
    fn run_erased(
        &self,
        model: &Model,
        ctx: &PassContext,
        analyses: &mut AnalysisManager,
    ) -> TranspileKindResult<()>;
    /// Human-readable display string.
    fn display(&self) -> String;
    /// Downcasts to the concrete pass type when needed.
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
    ) -> TranspileKindResult<()> {
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
