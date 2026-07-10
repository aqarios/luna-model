//! Pass-manager execution engine.

use std::{collections::HashSet, sync::Arc};

use lunamodel_core::Model;

use crate::{
    Pipeline,
    analysis::AnalysisManager,
    context::PassContext,
    erased::{
        ErasedAnalysisPass, ErasedCompositePass, ErasedMetaAnalysisPass, ErasedTransformPass,
    },
    error::{TranspileErrorKind, TranspilerResult, record},
    output::TransformationOutput,
    record::{PassEntry, TransformationRecord},
    step::PipelineStep,
};

/// Executes pipelines and manages analysis state across pass boundaries.
#[derive(Default)]
pub struct PassManager {
    passes: Vec<PipelineStep>,
    // analysis_manager: AnalysisManager,
}

impl PassManager {
    /// Creates an empty pass manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a pass manager from an explicit step list.
    pub fn from_steps(steps: Vec<PipelineStep>) -> Self {
        Self { passes: steps }
    }

    /// Appends a transformation pass.
    pub fn add_transform<T>(mut self, pass: T) -> Self
    where
        T: ErasedTransformPass + 'static,
    {
        self.passes.push(PipelineStep::Transform(Arc::new(pass)));
        self
    }

    /// Appends an analysis pass.
    pub fn add_analysis<A>(mut self, pass: A) -> Self
    where
        A: ErasedAnalysisPass + 'static,
    {
        self.passes.push(PipelineStep::Analysis(Arc::new(pass)));
        self
    }

    /// Appends a composite pass.
    pub fn add_composite<C>(mut self, pass: C) -> Self
    where
        C: ErasedCompositePass + 'static,
    {
        self.passes.push(PipelineStep::Composite(Arc::new(pass)));
        self
    }

    /// Appends a meta-analysis pass.
    pub fn add_meta_analysis<M>(mut self, pass: M) -> Self
    where
        M: ErasedMetaAnalysisPass + 'static,
    {
        self.passes.push(PipelineStep::MetaAnalysis(Arc::new(pass)));
        self
    }

    /// Appends a nested pipeline as a single step.
    pub fn add_pipeline(mut self, pipeline: Pipeline) -> Self {
        self.passes.push(PipelineStep::Pipeline(Arc::new(pipeline)));
        self
    }

    /// Appends a pre-built pipeline step.
    pub fn add_step(mut self, step: PipelineStep) -> Self {
        self.passes.push(step);
        self
    }

    /// Returns the configured steps in execution order.
    pub fn steps(&self) -> &[PipelineStep] {
        &self.passes
    }

    /// Validates and executes the configured steps against a model.
    pub fn run(&self, mut model: Model) -> TranspilerResult<TransformationOutput> {
        self.validate_requirements()?;
        let mut analysis = AnalysisManager::default();
        let record = execute_steps(&mut model, &self.passes, &mut analysis)?;
        Ok(TransformationOutput {
            record,
            model,
            analysis,
        })
    }

    /// Validates pass requirements against the configured execution order.
    fn validate_requirements(&self) -> TranspilerResult<()> {
        let mut satisfied: HashSet<String> = HashSet::new();
        self.validate_steps(&self.passes, &mut satisfied)
    }

    fn validate_steps(
        &self,
        steps: &[PipelineStep],
        satisfied: &mut HashSet<String>,
    ) -> TranspilerResult<()> {
        for step in steps {
            match step {
                PipelineStep::Transform(pass) => {
                    for requirement in pass.requires() {
                        if !satisfied.contains(requirement) {
                            return Err(TranspileErrorKind::UnsatisfiedRequirement {
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
                            return Err(TranspileErrorKind::UnsatisfiedRequirement {
                                pass_name: pass.name().to_string(),
                                requirement: requirement.to_string(),
                            }
                            .into());
                        }
                    }
                    satisfied.insert(pass.name().to_string());
                    satisfied.insert(pass.provides().to_string());
                }
                PipelineStep::MetaAnalysis(pass) => {
                    satisfied.insert(pass.name().to_string());
                    satisfied.insert(pass.provides().to_string());
                }
                PipelineStep::ControlFlow(pass) => {
                    for requirement in pass.requires() {
                        if !satisfied.contains(requirement) {
                            return Err(TranspileErrorKind::UnsatisfiedRequirement {
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
                            return Err(TranspileErrorKind::UnsatisfiedRequirement {
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

/// Executes a step list and records the resulting reversible transformation history.
fn execute_steps(
    model: &mut Model,
    passes: &[PipelineStep],
    analysis_manager: &mut AnalysisManager,
) -> TranspilerResult<TransformationRecord> {
    record(|entries| {
        for (pos, step) in passes.iter().enumerate() {
            match step {
                PipelineStep::Transform(pass) => {
                    let ctx = PassContext::new(analysis_manager);
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
                PipelineStep::MetaAnalysis(pass) => {
                    pass.run_erased(&passes[pos..], analysis_manager)?;
                    entries.push(PassEntry::MetaAnalysis {
                        pass_name: pass.name().to_string(),
                    });
                }
                PipelineStep::ControlFlow(pass) => {
                    let ctx = PassContext::new(analysis_manager);
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
        Ok(())
    })
}
