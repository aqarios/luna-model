use std::{collections::HashSet, sync::Arc};

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use crate::{
    Pipeline,
    analysis::AnalysisManager,
    context::PassContext,
    erased::{ErasedAnalysisPass, ErasedTransformPass},
    error::TransformationError,
    output::TransformationOutput,
    record::{PassEntry, TransformationRecord},
    step::PipelineStep,
};

#[derive(Default)]
pub struct PassManager {
    passes: Vec<PipelineStep>,
    // analysis_manager: AnalysisManager,
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

    pub fn add_pipeline(mut self, pipeline: Pipeline) -> Self {
        // self.passes.push(PipelineStep::Pipeline {
        //     name: pipeline.name.into(),
        //     passes: pipeline.steps,
        // });
        self.passes.push(PipelineStep::Pipeline(Arc::new(pipeline)));
        self
    }

    pub fn add_step(mut self, step: PipelineStep) -> Self {
        self.passes.push(step);
        self
    }

    pub fn steps(&self) -> &[PipelineStep] {
        &self.passes
    }

    pub fn run(&self, mut model: Model) -> LunaModelResult<TransformationOutput> {
        self.validate_requirements()?;
        let mut analysis = AnalysisManager::default();
        let record = execute_steps(&mut model, &self.passes, &mut analysis)?;
        Ok(TransformationOutput {
            record,
            model,
            analysis,
        })
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
                    for invalidated in pass.invalidates() {
                        satisfied.remove(invalidated);
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
                PipelineStep::ControlFlow(pass) => {
                    for requirement in pass.requires() {
                        if !satisfied.contains(requirement) {
                            return Err(TransformationError::UnsatisfiedRequirement {
                                pass_name: pass.name().to_string(),
                                requirement: requirement.to_string(),
                            }
                            .into());
                        }
                    }
                    for invalidated in pass.invalidates() {
                        satisfied.remove(invalidated);
                    }
                    satisfied.insert(pass.name().to_string());
                    satisfied.extend(pass.provides().to_owned());
                }
                PipelineStep::Composite(pass) => {
                    for requirement in pass.requires() {
                        if !satisfied.contains(requirement) {
                            return Err(TransformationError::UnsatisfiedRequirement {
                                pass_name: pass.name().to_string(),
                                requirement: requirement.to_string(),
                            }
                            .into());
                        }
                    }
                    for invalidated in pass.invalidates() {
                        satisfied.remove(invalidated);
                    }
                    satisfied.insert(pass.name().to_string());
                    satisfied.insert(pass.provides().to_string());
                }
                PipelineStep::Pipeline(p) => self.validate_steps(&p.steps, satisfied)?,
            }
        }
        Ok(())
    }
}

fn execute_steps(
    model: &mut Model,
    passes: &[PipelineStep],
    analysis_manager: &mut AnalysisManager,
) -> LunaModelResult<TransformationRecord> {
    let mut entries = Vec::new();
    for step in passes {
        match step {
            PipelineStep::Transform(pass) => {
                let ctx = PassContext::new(&analysis_manager);
                let artifact = pass.forward_erased(model, &ctx)?;
                entries.push(PassEntry::Transform {
                    pass_id: pass.id().to_string(),
                    pass_name: pass.name().to_string(),
                    artifact,
                });
                analysis_manager.invalidate_many(pass.invalidates());
            }
            PipelineStep::Analysis(pass) => {
                let analysis_snapshot = analysis_manager.clone();
                let ctx = PassContext::new(&analysis_snapshot);
                pass.run_erased(model, &ctx, analysis_manager)?;
                entries.push(PassEntry::Analysis {
                    pass_name: pass.name().to_string(),
                });
            }
            PipelineStep::ControlFlow(pass) => {
                let ctx = PassContext::new(&analysis_manager);
                let plan = pass.run_erased(model, &ctx)?;
                let sub_record = execute_steps(model, &plan.pipeline.steps, analysis_manager)?;
                entries.push(PassEntry::ControlFlow {
                    name: plan.pipeline.name,
                    pass_name: pass.name().to_string(),
                    record: sub_record,
                });
            }
            PipelineStep::Composite(pass) => {
                let analysis_snapshot = analysis_manager.clone();
                let ctx = PassContext::new(&analysis_snapshot);
                let artifact = pass.forward_erased(model, &ctx, analysis_manager)?;
                entries.push(PassEntry::Composite {
                    pass_id: pass.id().to_string(),
                    pass_name: pass.name().to_string(),
                    artifact,
                });
                analysis_manager.invalidate_many(pass.invalidates());
            }
            PipelineStep::Pipeline(p) => {
                let sub_record = execute_steps(model, &p.steps, analysis_manager)?;
                entries.push(PassEntry::Pipeline {
                    name: p.name.clone(),
                    record: sub_record,
                });
            }
        }
    }
    Ok(TransformationRecord { entries })
}
