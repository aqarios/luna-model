//! Host-side adapter that drives a C plugin as a native control-flow pass.

use lunamodel_capi::plugin::{LmControlFlowPlan, LmPassContext, LmPluginControlFlow};
use lunamodel_capi::{LmModel, LmStatus};
use lunamodel_capi_handle::{handle_into_storage, new_handle};
use lunamodel_core::Model;
use lunamodel_transpiler::{
    AnalysisManager, ControlFlowPass, ControlFlowPlan, PassContext, TranspileKindResult,
};

use crate::marshal::{borrow_cstr, collect_strings, plugin_failure, reclaim};

/// A plugin control-flow pass wrapping its C descriptor.
pub struct CPluginControlFlowPass {
    desc: LmPluginControlFlow,
    id: String,
    name: String,
    requires: Vec<String>,
    provides: Vec<String>,
}

// Safety: `desc` holds plugin-owned pointers opaque to luna-model. The plugin owns their
// thread-safety; luna-model only calls them from the thread running the pass.
unsafe impl Send for CPluginControlFlowPass {}
unsafe impl Sync for CPluginControlFlowPass {}

impl CPluginControlFlowPass {
    /// Builds an adapter from a plugin descriptor, caching its metadata up front. Takes ownership
    /// of `desc.user_data`.
    pub fn from_descriptor(desc: &LmPluginControlFlow) -> Self {
        let desc = *desc;
        let id = unsafe { borrow_cstr((desc.id)(desc.user_data)) };
        let name = unsafe { borrow_cstr((desc.name)(desc.user_data)) };
        let requires = unsafe { collect_strings(desc.user_data, desc.requires_analyses) };
        let provides = unsafe { collect_strings(desc.user_data, desc.provides) };
        Self {
            desc,
            id,
            name,
            requires,
            provides,
        }
    }
}

impl Drop for CPluginControlFlowPass {
    fn drop(&mut self) {
        unsafe { (self.desc.free)(self.desc.user_data) }
    }
}

impl ControlFlowPass for CPluginControlFlowPass {
    fn name(&self) -> &str {
        &self.name
    }

    fn requires(&self) -> &[String] {
        &self.requires
    }

    fn provides(&self) -> &[String] {
        &self.provides
    }

    fn run(&self, model: &Model, ctx: &PassContext) -> TranspileKindResult<ControlFlowPlan> {
        let handle = new_handle::<LmModel>(model.clone());
        let ctx_ptr = (ctx.manager() as *const AnalysisManager).cast::<LmPassContext>();
        let mut plan_ptr: *mut LmControlFlowPlan = std::ptr::null_mut();
        // Cleared right before the call so a stale error from an unrelated earlier call is never
        // misattributed to this one; see marshal::plugin_error.
        lunamodel_capi::error::clear_last();

        let status = unsafe {
            (self.desc.run)(
                self.desc.user_data,
                handle as *const LmModel,
                ctx_ptr,
                &mut plan_ptr,
            )
        };
        let _ = reclaim::<LmModel, Model>(handle, "model", "control_flow")?;
        if status != LmStatus::Ok || plan_ptr.is_null() {
            return Err(plugin_failure(
                &self.id,
                &format!("plugin control-flow '{}' run returned {status:?}", self.id),
            ));
        }
        // The plan is an owned handle the plugin built via `control_flow_plan_*` and handed back;
        // take ownership of its `ControlFlowPlan` value (mirrors freeing an unconsumed one).
        unsafe { handle_into_storage::<LmControlFlowPlan>(plan_ptr) }.ok_or_else(|| {
            plugin_failure(
                &self.id,
                &format!("plugin control-flow '{}' returned a null plan", self.id),
            )
        })
    }
}
