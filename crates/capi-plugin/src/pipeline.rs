//! Read-only view of pipeline steps lent to a meta-analysis plugin, and its introspection helpers.
//!
//! A meta-analysis pass receives the steps that follow it as a borrowed `&[PipelineStep]`. To lend
//! that across the C-ABI it is wrapped in a [`StepListView`] exposed as an opaque `*const LmStepList`;
//! the host `step_list_*` entry points cast the pointer back to a `StepListView` and answer
//! per-step queries. The view holds a raw slice and is valid only for the duration of the lending
//! `run` call — it is never retained.

use lunamodel_capi::plugin::LmPassKind;
use lunamodel_transpiler::{PipelineStep, PipelineStepMethods};

/// Borrowed, read-only view of a slice of pipeline steps, lent to a meta-analysis plugin as
/// `*const LmStepList`.
pub struct StepListView {
    ptr: *const PipelineStep,
    len: usize,
}

impl StepListView {
    /// Wraps a step slice. The view borrows `steps`; the caller must not let the erased pointer
    /// outlive the slice (it is lent only for the duration of the plugin `run` call).
    pub fn new(steps: &[PipelineStep]) -> Self {
        Self {
            ptr: steps.as_ptr(),
            len: steps.len(),
        }
    }

    fn steps(&self) -> &[PipelineStep] {
        if self.len == 0 {
            &[]
        } else {
            // Safety: built from a live slice in `new`; only dereferenced during the lending call.
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }

    /// Number of steps in the view.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the view is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Kind of the step at `index`, or `None` if out of bounds.
    pub fn kind(&self, index: usize) -> Option<LmPassKind> {
        self.steps().get(index).map(step_kind)
    }

    /// Name of the step at `index`, or `None` if out of bounds.
    pub fn name(&self, index: usize) -> Option<String> {
        self.steps().get(index).map(step_name)
    }

    /// Analysis names the step at `index` requires, or `None` if out of bounds.
    pub fn requires(&self, index: usize) -> Option<Vec<String>> {
        self.steps()
            .get(index)
            .map(|s| std::slice::from_ref(s).collect_requires().collect())
    }

    /// Analysis names the step at `index` provides, or `None` if out of bounds.
    pub fn provides(&self, index: usize) -> Option<Vec<String>> {
        self.steps()
            .get(index)
            .map(|s| std::slice::from_ref(s).collect_provides().collect())
    }

    /// Analysis names the step at `index` invalidates, or `None` if out of bounds.
    pub fn invalidates(&self, index: usize) -> Option<Vec<String>> {
        self.steps()
            .get(index)
            .map(|s| std::slice::from_ref(s).collect_invalidates().collect())
    }
}

/// Maps a step to its C-ABI kind selector (a nested pipeline reports [`LmPassKind::Pipeline`]).
fn step_kind(step: &PipelineStep) -> LmPassKind {
    match step {
        PipelineStep::Transform(_) => LmPassKind::Transformation,
        PipelineStep::Analysis(_) => LmPassKind::Analysis,
        PipelineStep::Composite(_) => LmPassKind::Composite,
        PipelineStep::ControlFlow(_) => LmPassKind::ControlFlow,
        PipelineStep::MetaAnalysis(_) => LmPassKind::MetaAnalysis,
        PipelineStep::Pipeline(_) => LmPassKind::Pipeline,
    }
}

/// The step's human-readable name (the sub-pipeline's name for a nested pipeline).
fn step_name(step: &PipelineStep) -> String {
    match step {
        PipelineStep::Transform(p) => p.name().to_string(),
        PipelineStep::Analysis(p) => p.name().to_string(),
        PipelineStep::Composite(p) => p.name().to_string(),
        PipelineStep::ControlFlow(p) => p.name().to_string(),
        PipelineStep::MetaAnalysis(p) => p.name().to_string(),
        PipelineStep::Pipeline(p) => p.name.clone(),
    }
}
