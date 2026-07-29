//! Control-flow plugin pass trait and its execution-plan builder.

use std::ffi::CString;

use lunamodel_capi::plugin::LmControlFlowPlan;

use crate::Model;
use crate::api::api;
use crate::error::{Error, Status, check};

use super::{
    AnalysisPass, CompositePass, MetaAnalysisPass, PassContext, TransformationPass, descriptor,
    descriptor_analysis, descriptor_composite, descriptor_control_flow, descriptor_meta_analysis,
};

/// A control-flow pass, authored in safe Rust — a 1:1 mirror of
/// `lunamodel_transpiler::ControlFlowPass`.
///
/// At `run`, the pass inspects the read-only model (and the analyses available so far) and builds a
/// [`ControlFlowPlan`]: the pipeline of steps to execute next. It is not itself reversible — the
/// steps it selects carry their own reversal. Implement this and hand the type to the SDK
/// (`descriptor_control_flow`, or the `rs-lunamodel` macros) — no `unsafe`, no C types.
pub trait ControlFlowPass: Send + Sync + 'static {
    /// Human-readable pass name.
    fn name(&self) -> &str;

    /// Analysis names this pass requires.
    fn requires(&self) -> &[String] {
        &[]
    }

    /// Analysis names this pass provides.
    fn provides(&self) -> &[String] {
        &[]
    }

    /// Analysis names this pass invalidates.
    fn invalidates(&self) -> &[String] {
        &[]
    }

    /// Executes control flow against the lent read-only model and context, building a plan.
    fn run(&self, model: &Model, ctx: &PassContext) -> crate::Result<ControlFlowPlan>;
}

/// The pipeline of steps a control-flow pass selects, built up with the `add_*` methods.
///
/// A 1:1 facade over the native `ControlFlowPlan`, across the C ABI: the host owns the plan behind
/// an opaque handle, and each `add_*` wraps a pass (of any kind — including a nested control-flow
/// pass) into the next step, exactly as building a native pipeline. Reversible steps still register
/// their own backward handler, as any plugin pass does.
pub struct ControlFlowPlan {
    raw: *mut LmControlFlowPlan,
}

impl ControlFlowPlan {
    /// Creates a new, empty plan named `name`.
    pub fn new(name: &str) -> crate::Result<Self> {
        let cname = CString::new(name).map_err(|_| interior_nul())?;
        let mut raw = std::ptr::null_mut();
        check(unsafe { (api().control_flow_plan_new)(cname.as_ptr(), &mut raw) })?;
        Ok(Self { raw })
    }

    /// Appends a transformation pass as the next step.
    pub fn add_transform<P: TransformationPass>(&mut self, pass: P) -> crate::Result<()> {
        let desc = descriptor(pass)?;
        check(unsafe { (api().control_flow_plan_add_transform)(self.raw, &desc) })
    }

    /// Appends an analysis pass as the next step.
    pub fn add_analysis<A: AnalysisPass>(&mut self, pass: A) -> crate::Result<()> {
        let desc = descriptor_analysis(pass)?;
        check(unsafe { (api().control_flow_plan_add_analysis)(self.raw, &desc) })
    }

    /// Appends a composite pass as the next step.
    pub fn add_composite<C: CompositePass>(&mut self, pass: C) -> crate::Result<()> {
        let desc = descriptor_composite(pass)?;
        check(unsafe { (api().control_flow_plan_add_composite)(self.raw, &desc) })
    }

    /// Appends a (nested) control-flow pass as the next step.
    pub fn add_control_flow<C: ControlFlowPass>(&mut self, pass: C) -> crate::Result<()> {
        let desc = descriptor_control_flow(pass)?;
        check(unsafe { (api().control_flow_plan_add_control_flow)(self.raw, &desc) })
    }

    /// Appends a meta-analysis pass as the next step.
    pub fn add_meta_analysis<M: MetaAnalysisPass>(&mut self, pass: M) -> crate::Result<()> {
        let desc = descriptor_meta_analysis(pass)?;
        check(unsafe { (api().control_flow_plan_add_meta_analysis)(self.raw, &desc) })
    }

    /// Consumes the wrapper, handing the owned raw handle to the host (which then owns and frees
    /// it). Used by the `run` trampoline to return the plan across the ABI.
    pub(crate) fn into_raw(self) -> *mut LmControlFlowPlan {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }
}

impl Drop for ControlFlowPlan {
    fn drop(&mut self) {
        // Frees an unconsumed plan (e.g. if the pass errors after building one). Once handed back
        // via `into_raw`, `forget` skips this and the host owns it.
        if !self.raw.is_null() {
            unsafe { (api().control_flow_plan_free)(self.raw) };
        }
    }
}

fn interior_nul() -> Error {
    Error::new(
        Status::InvalidArgument,
        "plan name contains an interior NUL byte",
    )
}
