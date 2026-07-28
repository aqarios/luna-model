//! `#[no_mangle]` C entry points for building a control-flow execution plan (`LmControlFlowPlan`).

use std::ffi::c_char;
use std::sync::Arc;

use lunamodel_capi::LmStatus;
use lunamodel_capi::plugin::{
    LmControlFlowPlan, LmPluginAnalysis, LmPluginComposite, LmPluginControlFlow,
    LmPluginMetaAnalysis, LmPluginTransform,
};
use lunamodel_capi_handle::{drop_handle, handle_from_storage, handle_mut};
use lunamodel_capi_plugin::{
    CPluginAnalysisPass, CPluginCompositePass, CPluginControlFlowPass, CPluginMetaAnalysisPass,
    CPluginTransformPass,
};
use lunamodel_transpiler::{ControlFlowPlan, PipelineStep};

use crate::error::{CapiError, guard};
use crate::utils::str_from_c;

/// Creates a new control-flow plan with name `name`, written to `out_plan`.
///
/// # Safety
/// `name` must be a valid NUL-terminated C string. `out_plan` must be a valid writable pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_control_flow_plan_new(
    name: *const c_char,
    out_plan: *mut *mut LmControlFlowPlan,
) -> LmStatus {
    guard(|| {
        if out_plan.is_null() {
            return Err(CapiError::null_pointer("out_plan"));
        }
        let name = unsafe { str_from_c(name, "name") }?.to_owned();
        let plan = ControlFlowPlan::new(name, Vec::new());
        let handle = handle_from_storage::<LmControlFlowPlan>(plan);
        unsafe { *out_plan = handle };
        Ok(())
    })
}

/// Frees an unconsumed control-flow plan handle.
///
/// # Safety
/// `plan` must be a handle returned by `lm_control_flow_plan_new` that has not yet been consumed
/// or freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_control_flow_plan_free(plan: *mut LmControlFlowPlan) {
    unsafe { drop_handle::<LmControlFlowPlan>(plan) };
}

/// Appends a plugin transformation pass step to a control-flow plan.
///
/// # Safety
/// `plan` must be a live `*mut LmControlFlowPlan` handle. `desc` must point to a valid descriptor.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_control_flow_plan_add_transform(
    plan: *mut LmControlFlowPlan,
    desc: *const LmPluginTransform,
) -> LmStatus {
    guard(|| {
        let plan = unsafe { handle_mut::<LmControlFlowPlan>(plan) }
            .ok_or_else(|| CapiError::null_pointer("plan"))?;
        if desc.is_null() {
            return Err(CapiError::null_pointer("desc"));
        }
        let adapter = CPluginTransformPass::from_descriptor(unsafe { &*desc });
        plan.pipeline
            .steps
            .push(PipelineStep::Transform(Arc::new(adapter)));
        Ok(())
    })
}

/// Appends a plugin analysis pass step to a control-flow plan.
///
/// # Safety
/// `plan` must be a live `*mut LmControlFlowPlan` handle. `desc` must point to a valid descriptor.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_control_flow_plan_add_analysis(
    plan: *mut LmControlFlowPlan,
    desc: *const LmPluginAnalysis,
) -> LmStatus {
    guard(|| {
        let plan = unsafe { handle_mut::<LmControlFlowPlan>(plan) }
            .ok_or_else(|| CapiError::null_pointer("plan"))?;
        if desc.is_null() {
            return Err(CapiError::null_pointer("desc"));
        }
        let adapter = CPluginAnalysisPass::from_descriptor(unsafe { &*desc });
        plan.pipeline
            .steps
            .push(PipelineStep::Analysis(Arc::new(adapter)));
        Ok(())
    })
}

/// Appends a plugin composite pass step to a control-flow plan.
///
/// # Safety
/// `plan` must be a live `*mut LmControlFlowPlan` handle. `desc` must point to a valid descriptor.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_control_flow_plan_add_composite(
    plan: *mut LmControlFlowPlan,
    desc: *const LmPluginComposite,
) -> LmStatus {
    guard(|| {
        let plan = unsafe { handle_mut::<LmControlFlowPlan>(plan) }
            .ok_or_else(|| CapiError::null_pointer("plan"))?;
        if desc.is_null() {
            return Err(CapiError::null_pointer("desc"));
        }
        let adapter = CPluginCompositePass::from_descriptor(unsafe { &*desc });
        plan.pipeline
            .steps
            .push(PipelineStep::Composite(Arc::new(adapter)));
        Ok(())
    })
}

/// Appends a plugin control-flow pass step to a control-flow plan.
///
/// # Safety
/// `plan` must be a live `*mut LmControlFlowPlan` handle. `desc` must point to a valid descriptor.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_control_flow_plan_add_control_flow(
    plan: *mut LmControlFlowPlan,
    desc: *const LmPluginControlFlow,
) -> LmStatus {
    guard(|| {
        let plan = unsafe { handle_mut::<LmControlFlowPlan>(plan) }
            .ok_or_else(|| CapiError::null_pointer("plan"))?;
        if desc.is_null() {
            return Err(CapiError::null_pointer("desc"));
        }
        let adapter = CPluginControlFlowPass::from_descriptor(unsafe { &*desc });
        plan.pipeline
            .steps
            .push(PipelineStep::ControlFlow(Arc::new(adapter)));
        Ok(())
    })
}

/// Appends a plugin meta-analysis pass step to a control-flow plan.
///
/// # Safety
/// `plan` must be a live `*mut LmControlFlowPlan` handle. `desc` must point to a valid descriptor.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_control_flow_plan_add_meta_analysis(
    plan: *mut LmControlFlowPlan,
    desc: *const LmPluginMetaAnalysis,
) -> LmStatus {
    guard(|| {
        let plan = unsafe { handle_mut::<LmControlFlowPlan>(plan) }
            .ok_or_else(|| CapiError::null_pointer("plan"))?;
        if desc.is_null() {
            return Err(CapiError::null_pointer("desc"));
        }
        let adapter = CPluginMetaAnalysisPass::from_descriptor(unsafe { &*desc });
        plan.pipeline
            .steps
            .push(PipelineStep::MetaAnalysis(Arc::new(adapter)));
        Ok(())
    })
}
