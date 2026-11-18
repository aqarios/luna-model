//! Pass-manager execution engine.

use std::sync::Arc;

use lunamodel_core::Model;
use lunamodel_types::EnumSet;

use crate::{
    Pipeline,
    analysis::AnalysisManager,
    context::PassContext,
    erased::{
        ErasedAnalysisPass, ErasedCompositePass, ErasedMetaAnalysisPass, ErasedTransformPass,
    },
    error::{TranspilerResult, attach_nested, record},
    output::TransformationOutput,
    record::{PassEntry, TransformationRecord},
    step::PipelineStep,
    validation::Availability,
    verify::Untouched,
};

/// Executes pipelines and manages analysis state across pass boundaries.
pub struct PassManager {
    passes: Vec<PipelineStep>,
    verify_modifies: bool,
}

impl Default for PassManager {
    fn default() -> Self {
        Self::from_steps(Vec::new())
    }
}

impl PassManager {
    /// Creates an empty pass manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a pass manager from an explicit step list.
    pub fn from_steps(steps: Vec<PipelineStep>) -> Self {
        Self {
            passes: steps,
            verify_modifies: cfg!(debug_assertions),
        }
    }

    /// Checks after every transformation that it only modified the model parts it declares in
    /// `modifies`. On by default in debug builds.
    pub fn verify_modifies(mut self, verify: bool) -> Self {
        self.verify_modifies = verify;
        self
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
        let record = execute_steps(
            &mut model,
            &self.passes,
            &mut analysis,
            self.verify_modifies,
        )?;
        Ok(TransformationOutput {
            record,
            model,
            analysis,
        })
    }

    /// Validates pass requirements against the configured execution order.
    fn validate_requirements(&self) -> TranspilerResult<()> {
        self.validate_steps(&self.passes, &mut Availability::default())
    }
}

/// Executes a step list and records the resulting reversible transformation history.
fn execute_steps(
    model: &mut Model,
    passes: &[PipelineStep],
    analysis_manager: &mut AnalysisManager,
    verify: bool,
) -> TranspilerResult<TransformationRecord> {
    record(|entries| {
        for (pos, step) in passes.iter().enumerate() {
            match step {
                PipelineStep::Transform(pass) => {
                    let untouched = verify.then(|| Untouched::snapshot(model, pass.modifies()));
                    let ctx = PassContext::new(analysis_manager);
                    let artifact = pass.forward_erased(model, &ctx)?;
                    entries.push(PassEntry::Transform {
                        pass_id: pass.id().to_string(),
                        pass_name: pass.name().to_string(),
                        artifact,
                    });
                    if let Some(untouched) = untouched {
                        untouched.check(model, pass.name())?;
                    }
                    analysis_manager.invalidate(pass.modifies());
                }
                PipelineStep::Analysis(pass) => {
                    let analysis_snapshot = analysis_manager.clone();
                    let ctx = PassContext::new(&analysis_snapshot);
                    pass.run_erased(model, &ctx, analysis_manager)?;
                    analysis_manager.set_depends_on(pass.provides(), pass.depends_on());
                    entries.push(PassEntry::Analysis {
                        pass_name: pass.name().to_string(),
                    });
                }
                PipelineStep::MetaAnalysis(pass) => {
                    pass.run_erased(&passes[pos..], analysis_manager)?;
                    analysis_manager.set_depends_on(pass.provides(), EnumSet::empty());
                    entries.push(PassEntry::MetaAnalysis {
                        pass_name: pass.name().to_string(),
                    });
                }
                PipelineStep::Composite(pass) => {
                    let untouched = verify.then(|| Untouched::snapshot(model, pass.modifies()));
                    let analysis_snapshot = analysis_manager.clone();
                    let ctx = PassContext::new(&analysis_snapshot);
                    // Invalidate before the pass deposits its own result, which is computed on
                    // the modified model.
                    analysis_manager.invalidate(pass.modifies());
                    let artifact = pass.forward_erased(model, &ctx, analysis_manager)?;
                    analysis_manager.set_depends_on(pass.provides(), pass.depends_on());
                    entries.push(PassEntry::Composite {
                        pass_id: pass.id().to_string(),
                        pass_name: pass.name().to_string(),
                        artifact,
                    });
                    if let Some(untouched) = untouched {
                        untouched.check(model, pass.name())?;
                    }
                }
                PipelineStep::ControlFlow(pass) => {
                    let ctx = PassContext::new(analysis_manager);
                    let plan = pass.run_erased(model, &ctx)?;
                    let res = execute_steps(model, &plan.pipeline.steps, analysis_manager, verify);
                    attach_nested(entries, res, |record| PassEntry::ControlFlow {
                        name: plan.pipeline.name,
                        pass_name: pass.name().to_string(),
                        record,
                    })?;
                }
                PipelineStep::Pipeline(p) => {
                    let res = execute_steps(model, &p.steps, analysis_manager, verify);
                    attach_nested(entries, res, |record| PassEntry::Pipeline {
                        name: p.name.clone(),
                        record,
                    })?;
                }
            }
        }
        Ok(())
    })
}
