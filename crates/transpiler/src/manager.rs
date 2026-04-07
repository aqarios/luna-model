use std::{collections::HashSet, sync::Arc};

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use crate::{
    analysis::AnalysisManager,
    artifact::ErasedArtifact,
    context::PassContext,
    error::TransformationError,
    pass::{AnalysisPass, ReversiblePass},
    record::{CompilationRecord, PassEntry},
};

/// Object-safe erased transform pass used by the pipeline runtime.
pub trait ErasedTransformPass: Send + Sync {
    fn name(&self) -> &str;
    fn requires(&self) -> &[String];
    fn invalidates(&self) -> &[String];
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
    fn name(&self) -> &str {
        &self.name()
    }

    fn requires(&self) -> &[String] {
        &self.requires()
    }

    fn invalidates(&self) -> &[String] {
        &self.invalidates()
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
    fn name(&self) -> &str;
    fn provides(&self) -> &str;
    fn requires(&self) -> &[String];
    fn run_erased(
        &self,
        model: &Model,
        ctx: &PassContext,
        analyses: &mut AnalysisManager,
    ) -> LunaModelResult<()>;
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
        let key = crate::analysis::AnalysisKey::<P::Result>::new(self.provides().into());
        analyses.insert(&key, value);
        Ok(())
    }
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

#[derive(Default)]
pub struct PassManager {
    passes: Vec<PipelineStep>,
    analysis_manager: AnalysisManager,
}

impl PassManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_steps(steps: Vec<PipelineStep>) -> Self {
        Self {
            passes: steps,
            ..Self::default()
        }
    }

    pub fn add_transform<T>(mut self, pass: T) -> Self
    where
        T: ErasedTransformPass + 'static,
    {
        self.passes.push(PipelineStep::Transform(Arc::new(pass)));
        self
    }

    pub fn add_analysis<A>(mut self, pass: A) -> Self
    where
        A: ErasedAnalysisPass + 'static,
    {
        self.passes.push(PipelineStep::Analysis(Arc::new(pass)));
        self
    }

    pub fn add_pipeline(mut self, name: impl Into<String>, passes: Vec<PipelineStep>) -> Self {
        self.passes.push(PipelineStep::Pipeline {
            name: name.into(),
            passes,
        });
        self
    }

    fn validate_requirements(&self) -> LunaModelResult<()> {
        let mut satisfied: HashSet<String> = HashSet::new();
        self.validate_steps(&self.passes, &mut satisfied)
    }

    fn validate_steps(
        &self,
        steps: &[PipelineStep],
        satisfied: &mut HashSet<String>,
    ) -> LunaModelResult<()> {
        for step in steps {
            match step {
                PipelineStep::Transform(pass) => {
                    for requirement in pass.requires() {
                        if !satisfied.contains(requirement) {
                            return Err(TransformationError::UnsatisfiedRequirement {
                                pass_name: pass.name().to_string(),
                                requirement: requirement.to_string(),
                            }
                            .into());
                        }
                    }
                    satisfied.insert(pass.name().to_string());
                }
                PipelineStep::Analysis(pass) => {
                    for requirement in pass.requires() {
                        if !satisfied.contains(requirement) {
                            return Err(TransformationError::UnsatisfiedRequirement {
                                pass_name: pass.name().to_string(),
                                requirement: requirement.to_string(),
                            }
                            .into());
                        }
                    }
                    satisfied.insert(pass.name().to_string());
                    satisfied.insert(pass.provides().to_string());
                }
                PipelineStep::Pipeline { passes, .. } => {
                    self.validate_steps(passes, satisfied)?;
                }
            }
        }
        Ok(())
    }

    pub fn run(&mut self, model: &mut Model) -> LunaModelResult<CompilationRecord> {
        self.validate_requirements()?;
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
                    self.analysis_manager.invalidate_many(pass.invalidates());
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
