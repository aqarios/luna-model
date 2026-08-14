//! Host-side adapter that drives a C plugin as a native meta-analysis pass.
//!
//! The C-ABI twin of `PyMetaAnalysisPassAdapter`: it implements the transpiler [`MetaAnalysisPass`]
//! trait by calling out through the plugin's C function pointers. A meta-analysis reads no model;
//! instead it introspects the steps that follow it — lent as an [`LmStepList`] the plugin queries
//! through the `step_list_*` API-table entries — and its serialized result is deposited under
//! `provides` (result type [`SerializedAnalysis`]) where later passes read it back.

use std::os::raw::c_void;

use lunamodel_capi::LmStatus;
use lunamodel_capi::plugin::{LmPluginMetaAnalysis, LmStepList};
use lunamodel_transpiler::{
    AnalysisKey, MetaAnalysisPass, PipelineStep, SerializedAnalysis, TranspileKindResult,
};

use crate::marshal::{borrow_cstr, byte_sink, plugin_error};
use crate::pipeline::StepListView;

/// A plugin meta-analysis pass wrapping its C descriptor.
pub struct CPluginMetaAnalysisPass {
    desc: LmPluginMetaAnalysis,
    id: String,
    name: String,
    provides: String,
    result_type_tag: String,
}

// Safety: `desc` holds plugin-owned pointers opaque to luna-model. The plugin owns their
// thread-safety; luna-model only calls them from the thread running the pass.
unsafe impl Send for CPluginMetaAnalysisPass {}
unsafe impl Sync for CPluginMetaAnalysisPass {}

impl CPluginMetaAnalysisPass {
    /// Builds an adapter from a plugin descriptor, caching its metadata up front. Takes ownership
    /// of `desc.user_data`.
    pub fn from_descriptor(desc: &LmPluginMetaAnalysis) -> Self {
        let desc = *desc;
        let id = unsafe { borrow_cstr((desc.id)(desc.user_data)) };
        let name = unsafe { borrow_cstr((desc.name)(desc.user_data)) };
        let provides = unsafe { borrow_cstr((desc.provides)(desc.user_data)) };
        let result_type_tag = unsafe { borrow_cstr((desc.result_type_tag)(desc.user_data)) };
        Self {
            desc,
            id,
            name,
            provides,
            result_type_tag,
        }
    }
}

impl Drop for CPluginMetaAnalysisPass {
    fn drop(&mut self) {
        unsafe { (self.desc.free)(self.desc.user_data) }
    }
}

impl MetaAnalysisPass for CPluginMetaAnalysisPass {
    type Result = SerializedAnalysis;

    // Unused: `name`/`provides` are overridden to return the instance's dynamic values.
    const NAME: &'static str = "";
    const PROVIDES: &'static str = "";

    fn name(&self) -> &str {
        &self.name
    }

    fn provides(&self) -> &str {
        &self.provides
    }

    fn key<T>() -> AnalysisKey<T> {
        // Consumers read a plugin meta-analysis's result by name, never through this typed key.
        AnalysisKey::new(String::new())
    }

    fn run(&self, steps: &[PipelineStep]) -> TranspileKindResult<SerializedAnalysis> {
        // Lend the following steps as a read-only view; the plugin introspects them through the
        // `step_list_*` API entries and never retains the pointer.
        let view = StepListView::new(steps);
        let mut payload: Vec<u8> = Vec::new();
        // Cleared right before the call so a stale error from an unrelated earlier call is never
        // misattributed to this one; see marshal::plugin_error.
        lunamodel_capi::error::clear_last();

        let status = unsafe {
            (self.desc.run)(
                self.desc.user_data,
                (&view as *const StepListView).cast::<LmStepList>(),
                byte_sink,
                (&mut payload as *mut Vec<u8>).cast::<c_void>(),
            )
        };
        if status != LmStatus::Ok {
            return Err(plugin_error(&format!(
                "plugin meta-analysis '{}' run returned {status:?}",
                self.id
            )));
        }
        Ok(SerializedAnalysis {
            type_tag: self.result_type_tag.clone(),
            bytes: payload,
        })
    }
}
