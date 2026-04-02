use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use crate::{
    analysis::AnalysisManager,
    artifact::ErasedArtifact,
    context::PassContext,
    pass::ReversiblePass,
    record::{CompilationRecord, PassEntry},
};

/// Object-safe erased transform pass used by the pipeline runtime.
pub trait ErasedTransformPass: Send + Sync {
    fn name<'a>(&'a self) -> &'a str;
    fn forward_erased(
        &self,
        model: &mut Model,
        ctx: &PassContext,
    ) -> LunaModelResult<ErasedArtifact>;
}

/// Typed pass can be wrapped into ErasedTransformPass.
impl<P> ErasedTransformPass for P
where
    P: ReversiblePass + Send + Sync + 'static,
{
    fn name<'a>(&'a self) -> &'a str {
        self.name()
    }

    fn forward_erased(
        &self,
        model: &mut Model,
        ctx: &PassContext,
    ) -> LunaModelResult<ErasedArtifact> {
        let artifact = self.forward(model, ctx)?;
        ErasedArtifact::new(&artifact)
    }
}

pub trait ErasedAnalysisPass: Send + Sync {
    fn name(&self) -> &'static str;
    fn run_erased(
        &self,
        model: &Model,
        ctx: &PassContext,
        analyses: &mut AnalysisManager,
    ) -> LunaModelResult<()>;
}

#[derive(Clone)]
pub enum PipelineStep {
    Transform(Arc<dyn ErasedTransformPass>),
    Analysis(Arc<dyn ErasedAnalysisPass>),
    Pipeline {
        name: String,
        passes: Vec<PipelineStep>,
    },
}

// Note: PipelineStep is intentionally Arc-backed so `from_steps(steps.clone())`
// is cheap and does not require cloning non-cloneable closures or trait objects.

pub struct PassManager {
    passes: Vec<PipelineStep>,
    analysis_manager: AnalysisManager,
    invalidates_by_pass: HashMap<&'static str, HashSet<&'static str>>,
}

impl PassManager {
    pub fn run(&mut self, model: &mut Model) -> LunaModelResult<CompilationRecord> {
        let mut entries = Vec::new();
        for step in &self.passes {
            match step {
                PipelineStep::Transform(pass) => {
                    let ctx = PassContext::new(&self.analysis_manager);
                    let artifact = pass.forward_erased(model, &ctx)?;
                    entries.push(PassEntry::Transform {
                        pass_name: pass.name().to_string(),
                        artifact,
                    });
                    self.analysis_manager
                        .invalidate(pass.name(), &self.invalidates_by_pass);
                }
                PipelineStep::Analysis(pass) => {
                    let analysis_snapshot = self.analysis_manager.clone();
                    let ctx = PassContext::new(&analysis_snapshot);
                    pass.run_erased(model, &ctx, &mut self.analysis_manager)?;
                    entries.push(PassEntry::Analysis {
                        pass_name: pass.name().to_string(),
                    });
                }
                PipelineStep::Pipeline { name, passes } => {
                    let mut sub_manager = PassManager {
                        passes: passes.clone(),
                        analysis_manager: self.analysis_manager.clone(),
                        invalidates_by_pass: self.invalidates_by_pass.clone(),
                    };
                    let sub_record = sub_manager.run(model)?;
                    entries.push(PassEntry::Pipeline {
                        name: name.clone(),
                        record: Box::new(sub_record),
                    });
                }
            }
        }
        Ok(CompilationRecord { entries })
    }
}
