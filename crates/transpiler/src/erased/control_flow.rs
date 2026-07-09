//! Object-safe runtime adapter for control-flow passes.

use std::any::Any;

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use crate::{ControlFlowPass, ControlFlowPlan, PassContext, PipelineStep};

/// Object-safe erased control flow pass used by the pipeline runtime.
pub trait ErasedControlFlowPass: Send + Sync {
    /// Human-readable pass name.
    fn name(&self) -> &str;
    /// Required pass/analysis names.
    fn requires(&self) -> &[String];
    /// Provided names after execution.
    fn provides(&self) -> &[String];
    /// Invalidated analysis names.
    fn invalidates(&self) -> &[String];
    /// All possible branches (sub-steps) this pass may branch into, for static validation.
    fn branches(&self) -> Vec<&[PipelineStep]>;
    /// Runs the pass and returns the selected control-flow plan.
    fn run_erased(&self, model: &mut Model, ctx: &PassContext) -> LunaModelResult<ControlFlowPlan>;
    /// Human-readable display string.
    fn display(&self) -> String;
    /// Downcasts to the concrete pass type when needed.
    fn as_any(&self) -> &dyn Any;
}

/// Adapts a typed control-flow pass to the object-safe runtime interface.
impl<P> ErasedControlFlowPass for P
where
    P: ControlFlowPass + Send + Sync + 'static,
{
    fn name(&self) -> &str {
        self.name()
    }

    fn requires(&self) -> &[String] {
        self.requires()
    }

    fn provides(&self) -> &[String] {
        self.provides()
    }

    fn invalidates(&self) -> &[String] {
        self.invalidates()
    }

    fn branches(&self) -> Vec<&[PipelineStep]> {
        self.branches()
    }

    fn run_erased(&self, model: &mut Model, ctx: &PassContext) -> LunaModelResult<ControlFlowPlan> {
        self.run(model, ctx)
    }

    fn display(&self) -> String {
        self.display()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
