//! Host-side adapter that drives a C plugin as a native analysis pass.
//!
//! The C-ABI twin of `PyAnalysisPassAdapter`: it implements the transpiler [`AnalysisPass`] trait by
//! calling out through the plugin's C function pointers. Its result type is [`SerializedAnalysis`],
//! so the transpiler's ordinary analysis machinery deposits the plugin's serialized bytes into the
//! run's [`AnalysisManager`] under the pass's `provides` name — exactly where a later plugin reads
//! them back with `pass_context_get_analysis`.

use std::os::raw::c_void;

use lunamodel_capi::plugin::{LmPassContext, LmPluginAnalysis};
use lunamodel_capi::{LmModel, LmStatus};
use lunamodel_capi_handle::new_handle;
use lunamodel_core::Model;
use lunamodel_transpiler::{
    AnalysisKey, AnalysisManager, AnalysisPass, PassContext, SerializedAnalysis,
    TranspileKindResult,
};

use crate::marshal::{borrow_cstr, byte_sink, collect_strings, plugin_error, reclaim};

/// A plugin analysis pass wrapping its C descriptor.
pub struct CPluginAnalysisPass {
    desc: LmPluginAnalysis,
    id: String,
    name: String,
    provides: String,
    result_type_tag: String,
    requires: Vec<String>,
}

// Safety: `desc` holds plugin-owned pointers opaque to luna-model. The plugin owns their
// thread-safety; luna-model only calls them from the thread running the pass.
unsafe impl Send for CPluginAnalysisPass {}
unsafe impl Sync for CPluginAnalysisPass {}

impl CPluginAnalysisPass {
    /// Builds an adapter from a plugin descriptor, caching its metadata up front. Takes ownership
    /// of `desc.user_data`.
    pub fn from_descriptor(desc: &LmPluginAnalysis) -> Self {
        let desc = *desc;
        let id = unsafe { borrow_cstr((desc.id)(desc.user_data)) };
        let name = unsafe { borrow_cstr((desc.name)(desc.user_data)) };
        let provides = unsafe { borrow_cstr((desc.provides)(desc.user_data)) };
        let result_type_tag = unsafe { borrow_cstr((desc.result_type_tag)(desc.user_data)) };
        let requires = unsafe { collect_strings(desc.user_data, desc.requires_analyses) };
        Self {
            desc,
            id,
            name,
            provides,
            result_type_tag,
            requires,
        }
    }
}

impl Drop for CPluginAnalysisPass {
    fn drop(&mut self) {
        unsafe { (self.desc.free)(self.desc.user_data) }
    }
}

impl AnalysisPass for CPluginAnalysisPass {
    type Result = SerializedAnalysis;

    // Unused: `provides()` is overridden to return the instance's dynamic name.
    const PROVIDES: &'static str = "";

    fn name(&self) -> &str {
        &self.name
    }

    fn provides(&self) -> &str {
        &self.provides
    }

    fn key<T>() -> AnalysisKey<T> {
        // Consumers of a plugin analysis read it by name (`require_analysis(name)`), never through
        // this typed key, so it is never invoked for a plugin pass.
        AnalysisKey::new(String::new())
    }

    fn requires(&self) -> &[String] {
        &self.requires
    }

    fn run(&self, model: &Model, ctx: &PassContext) -> TranspileKindResult<SerializedAnalysis> {
        // Lend a read-only clone as an owned handle; the plugin reads through it and relinquishes
        // it, then the host reclaims and drops the clone.
        let handle = new_handle::<LmModel>(model.clone());
        let ctx_ptr = (ctx.manager() as *const AnalysisManager).cast::<LmPassContext>();
        let mut payload: Vec<u8> = Vec::new();
        // Cleared right before the call so a stale error from an unrelated earlier call is never
        // misattributed to this one; see marshal::plugin_error.
        lunamodel_capi::error::clear_last();

        let status = unsafe {
            (self.desc.run)(
                self.desc.user_data,
                handle as *const LmModel,
                ctx_ptr,
                byte_sink,
                (&mut payload as *mut Vec<u8>).cast::<c_void>(),
            )
        };
        let _ = reclaim::<LmModel, Model>(handle, "model", "analysis")?;
        if status != LmStatus::Ok {
            return Err(plugin_error(&format!(
                "plugin analysis '{}' run returned {status:?}",
                self.id
            )));
        }
        Ok(SerializedAnalysis {
            type_tag: self.result_type_tag.clone(),
            bytes: payload,
        })
    }
}
