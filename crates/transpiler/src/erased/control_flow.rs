use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use crate::{ControlFlowPass, ControlFlowPlan, PassContext};

/// Object-safe erased control flow pass used by the pipeline runtime.
pub trait ErasedControlFlowPass: Send + Sync {
    fn name(&self) -> &str;
    fn requires(&self) -> &[String];
    fn provides(&self) -> &[String];
    fn invalidates(&self) -> &[String];
    fn run_erased(&self, model: &mut Model, ctx: &PassContext) -> LunaModelResult<ControlFlowPlan>;
    fn display(&self) -> String;
}

/// Typed pass can be wrapped into ErasedTransformPass.
impl<P> ErasedControlFlowPass for P
where
    P: ControlFlowPass + Send + Sync + 'static,
{
    fn name(&self) -> &str {
        &self.name()
    }

    fn requires(&self) -> &[String] {
        &self.requires()
    }

    fn provides(&self) -> &[String] {
        self.provides()
    }

    fn invalidates(&self) -> &[String] {
        &self.invalidates()
    }

    fn run_erased(&self, model: &mut Model, ctx: &PassContext) -> LunaModelResult<ControlFlowPlan> {
        self.run(model, ctx)
    }

    fn display(&self) -> String {
        self.display()
    }
}
