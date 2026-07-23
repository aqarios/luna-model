//! Host-side adapter that drives a C plugin as a native transformation pass.
//!
//! The C-ABI twin of `PyTransformationPassAdapter`: it implements the transpiler
//! `TransformationPass`/`Reversible` traits by calling out through the plugin's C function
//! pointers. The model crosses as a live handle — moved in, moved back out (no clone, no
//! serialization).

use std::os::raw::c_void;

use lunamodel_capi_handle::new_handle;
use lunamodel_capi::plugin::{LmPassContext, LmPluginTransform};
use lunamodel_capi::{LmModel, LmSol, LmStatus};
use lunamodel_core::{Model, Solution};
use lunamodel_transpiler::{
    AnalysisManager, PassContext, Reversible, TransformationPass, TranspileErrorKind,
    TranspileKindResult,
};

use crate::artifact::CPluginArtifact;
use crate::marshal::{borrow_cstr, byte_sink, collect_strings, plugin_error, reclaim};
use crate::registry;

/// A plugin transformation pass wrapping its C descriptor.
pub struct CPluginTransformPass {
    desc: LmPluginTransform,
    id: String,
    name: String,
    requires: Vec<String>,
    invalidates: Vec<String>,
}

// Safety: `desc` holds plugin-owned pointers opaque to luna-model. The plugin owns their
// thread-safety; luna-model only calls them from the thread running the pass.
unsafe impl Send for CPluginTransformPass {}
unsafe impl Sync for CPluginTransformPass {}

impl CPluginTransformPass {
    /// Builds an adapter from a plugin descriptor, caching its metadata up front (mirrors
    /// `PyTransformationPassAdapter::new`). Takes ownership of `desc.user_data`.
    pub fn from_descriptor(desc: &LmPluginTransform) -> Self {
        crate::ensure_registered();
        let desc = *desc;
        let id = unsafe { borrow_cstr((desc.id)(desc.user_data)) };
        let name = unsafe { borrow_cstr((desc.name)(desc.user_data)) };
        let requires = unsafe { collect_strings(desc.user_data, desc.requires) };
        let invalidates = unsafe { collect_strings(desc.user_data, desc.invalidates) };
        Self {
            desc,
            id,
            name,
            requires,
            invalidates,
        }
    }
}

impl Drop for CPluginTransformPass {
    fn drop(&mut self) {
        unsafe { (self.desc.free)(self.desc.user_data) }
    }
}

impl TransformationPass for CPluginTransformPass {
    fn name(&self) -> &str {
        &self.name
    }

    fn requires(&self) -> &[String] {
        &self.requires
    }

    fn invalidates(&self) -> &[String] {
        &self.invalidates
    }

    fn forward(&self, model: &mut Model, ctx: &PassContext) -> TranspileKindResult<Self::Artifact> {
        // Move the model into an owned handle the plugin mutates via `lm_*`, then reclaim it.
        let handle = new_handle::<LmModel>(std::mem::take(model));
        // `LmPassContext` is represented as the analysis store; lend the manager this context
        // borrows so the plugin can read analyses through `pass_context_get_analysis`.
        let ctx_ptr = (ctx.manager() as *const AnalysisManager).cast::<LmPassContext>();
        let mut payload: Vec<u8> = Vec::new();
        let status = unsafe {
            (self.desc.forward)(
                self.desc.user_data,
                handle,
                ctx_ptr,
                byte_sink,
                (&mut payload as *mut Vec<u8>).cast::<c_void>(),
            )
        };
        *model = reclaim::<LmModel, Model>(handle, "model", "forward")?;
        if status != LmStatus::Ok {
            return Err(plugin_error(&format!(
                "plugin pass '{}' forward returned {status:?}",
                self.id
            )));
        }
        Ok(CPluginArtifact {
            plugin_pass_id: self.id.clone(),
            payload,
        })
    }
}

impl Reversible for CPluginTransformPass {
    type Artifact = CPluginArtifact;

    const ID: &'static str = "luna_model::CPluginTransformPass";

    fn backward(artifact: &Self::Artifact, solution: Solution) -> TranspileKindResult<Solution> {
        let (func, user_data) = registry::lookup_transform(&artifact.plugin_pass_id).ok_or(
            TranspileErrorKind::UnregisteredPass {
                name: artifact.plugin_pass_id.clone(),
            },
        )?;
        // Transfer ownership of the solution to the plugin, and reclaim the one it produces.
        let in_handle = new_handle::<LmSol>(solution);
        let mut out_handle: *mut LmSol = std::ptr::null_mut();
        let status = unsafe {
            func(
                user_data,
                artifact.payload.as_ptr(),
                artifact.payload.len(),
                in_handle,
                &mut out_handle,
            )
        };
        if status != LmStatus::Ok {
            return Err(plugin_error(&format!(
                "plugin pass '{}' backward returned {status:?}",
                artifact.plugin_pass_id
            )));
        }
        reclaim::<LmSol, Solution>(out_handle, "solution", "backward")
    }
}
