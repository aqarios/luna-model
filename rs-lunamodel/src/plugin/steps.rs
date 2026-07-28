//! The plugin-side mirror of the pipeline-step view a meta-analysis pass reads.

use std::ffi::{CStr, c_char, c_void};

use lunamodel_capi::LmStatus;
use lunamodel_capi::plugin::{LmPassKind, LmStepList, LmStrSink};

use crate::api::api;
use crate::error::check;

/// The kind of a pipeline step — a 1:1 facade over the C `LmPassKind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepKind {
    /// A transformation pass.
    Transformation,
    /// An analysis pass.
    Analysis,
    /// A composite (fused transform + analysis) pass.
    Composite,
    /// A control-flow pass.
    ControlFlow,
    /// A meta-analysis pass.
    MetaAnalysis,
    /// A nested sub-pipeline.
    Pipeline,
}

impl From<LmPassKind> for StepKind {
    fn from(kind: LmPassKind) -> Self {
        match kind {
            LmPassKind::Transformation => StepKind::Transformation,
            LmPassKind::Analysis => StepKind::Analysis,
            LmPassKind::Composite => StepKind::Composite,
            LmPassKind::ControlFlow => StepKind::ControlFlow,
            LmPassKind::MetaAnalysis => StepKind::MetaAnalysis,
            LmPassKind::Pipeline => StepKind::Pipeline,
        }
    }
}

/// Read-only view of the pipeline steps that follow a meta-analysis pass, lent to it during
/// [`run`](super::MetaAnalysisPass::run).
///
/// A 1:1 mirror of the step slice `lunamodel_transpiler::MetaAnalysisPass::run` receives, exposed
/// over the C ABI: the count, and per step its [`StepKind`], name, and the analysis names it
/// requires / provides / invalidates. Borrowed for the duration of the call only — do not retain it.
pub struct StepList {
    raw: *const LmStepList,
}

// SAFETY: used transiently within one `run` call; a detached one holds only a null pointer, and
// neither is shared across threads while pointing at live host state.
unsafe impl Send for StepList {}
unsafe impl Sync for StepList {}

impl StepList {
    /// Wraps a step-list pointer lent by the host for the duration of a call (does not own it).
    pub(crate) fn from_raw(raw: *const LmStepList) -> Self {
        Self { raw }
    }

    /// Number of steps following the pass.
    pub fn len(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        let mut out = 0usize;
        let status = unsafe { (api().step_list_len)(self.raw, &mut out) };
        if status == LmStatus::Ok { out } else { 0 }
    }

    /// Whether there are no following steps.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The step at `index`, or `None` if out of range.
    pub fn get(&self, index: usize) -> Option<Step<'_>> {
        if index < self.len() {
            Some(Step { list: self, index })
        } else {
            None
        }
    }

    /// Iterates the following steps in order.
    pub fn iter(&self) -> impl Iterator<Item = Step<'_>> {
        (0..self.len()).map(move |index| Step { list: self, index })
    }
}

/// A single step in a [`StepList`]. Its accessors dispatch across the C ABI, so each returns a
/// [`crate::Result`].
pub struct Step<'a> {
    list: &'a StepList,
    index: usize,
}

impl Step<'_> {
    /// The step's kind.
    pub fn kind(&self) -> crate::Result<StepKind> {
        let mut out = LmPassKind::Transformation;
        check(unsafe { (api().step_list_kind)(self.list.raw, self.index, &mut out) })?;
        Ok(out.into())
    }

    /// The step's human-readable name.
    pub fn name(&self) -> crate::Result<String> {
        let names = collect(|sink, ctx| unsafe {
            (api().step_list_name)(self.list.raw, self.index, sink, ctx)
        })?;
        Ok(names.into_iter().next().unwrap_or_default())
    }

    /// The analysis names this step requires.
    pub fn requires(&self) -> crate::Result<Vec<String>> {
        collect(|sink, ctx| unsafe {
            (api().step_list_requires)(self.list.raw, self.index, sink, ctx)
        })
    }

    /// The analysis names this step provides.
    pub fn provides(&self) -> crate::Result<Vec<String>> {
        collect(|sink, ctx| unsafe {
            (api().step_list_provides)(self.list.raw, self.index, sink, ctx)
        })
    }

    /// The analysis names this step invalidates.
    pub fn invalidates(&self) -> crate::Result<Vec<String>> {
        collect(|sink, ctx| unsafe {
            (api().step_list_invalidates)(self.list.raw, self.index, sink, ctx)
        })
    }
}

/// Drives a string-emitting C call, collecting the emitted strings into a `Vec`.
fn collect(call: impl FnOnce(LmStrSink, *mut c_void) -> LmStatus) -> crate::Result<Vec<String>> {
    let mut out: Vec<String> = Vec::new();
    let status = call(
        collect_string,
        (&mut out as *mut Vec<String>).cast::<c_void>(),
    );
    check(status)?;
    Ok(out)
}

/// `LmStrSink`: pushes each emitted string into a `Vec<String>`.
unsafe extern "C" fn collect_string(ctx: *mut c_void, s: *const c_char) {
    if ctx.is_null() || s.is_null() {
        return;
    }
    let out = unsafe { &mut *(ctx as *mut Vec<String>) };
    out.push(unsafe { CStr::from_ptr(s) }.to_string_lossy().into_owned());
}
