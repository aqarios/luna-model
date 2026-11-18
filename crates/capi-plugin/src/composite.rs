//! Host-side adapter that drives a C plugin as a native composite pass.
//!
//! The C-ABI twin of `PyCompositePassAdapter`: it implements the transpiler [`CompositePass`] (and
//! its [`Reversible`] super-trait) by calling out through the plugin's C function pointers. A
//! composite fuses a transformation and an analysis — its `forward` both mutates the model (moved
//! in and back out, like a transform) and emits a serialized result deposited under `provides`
//! (like an analysis). Reversal routes by plugin id through the shared [`registry`], exactly like
//! [`super::pass::CPluginTransformPass`].

use std::os::raw::c_void;

use lunamodel_capi::plugin::{LmPassContext, LmPluginComposite};
use lunamodel_capi::{LmModel, LmSol, LmStatus};
use lunamodel_capi_handle::new_handle;
use lunamodel_core::{Model, Solution};
use lunamodel_transpiler::{
    AnalysisKey, AnalysisManager, CompositePass, PassContext, Reversible, SerializedAnalysis,
    TranspileErrorKind, TranspileKindResult,
};
use lunamodel_types::{EnumSet, ModelPart};

use crate::artifact::CPluginArtifact;
use crate::marshal::{
    borrow_cstr, byte_sink, collect_strings, model_parts, plugin_failure, reclaim,
};
use crate::registry;

/// A plugin composite pass wrapping its C descriptor.
pub struct CPluginCompositePass {
    desc: LmPluginComposite,
    id: String,
    name: String,
    provides: String,
    result_type_tag: String,
    requires: Vec<String>,
    modifies: EnumSet<ModelPart>,
    depends_on: EnumSet<ModelPart>,
}

// Safety: `desc` holds plugin-owned pointers opaque to luna-model. The plugin owns their
// thread-safety; luna-model only calls them from the thread running the pass.
unsafe impl Send for CPluginCompositePass {}
unsafe impl Sync for CPluginCompositePass {}

impl CPluginCompositePass {
    /// Builds an adapter from a plugin descriptor, caching its metadata up front. Takes ownership
    /// of `desc.user_data`.
    pub fn from_descriptor(desc: &LmPluginComposite) -> Self {
        crate::ensure_registered();
        let desc = *desc;
        let id = unsafe { borrow_cstr((desc.id)(desc.user_data)) };
        let name = unsafe { borrow_cstr((desc.name)(desc.user_data)) };
        let provides = unsafe { borrow_cstr((desc.provides)(desc.user_data)) };
        let result_type_tag = unsafe { borrow_cstr((desc.result_type_tag)(desc.user_data)) };
        let requires = unsafe { collect_strings(desc.user_data, desc.requires_analyses) };
        let modifies = model_parts(unsafe { (desc.modifies)(desc.user_data) });
        let depends_on = model_parts(unsafe { (desc.depends_on)(desc.user_data) });
        Self {
            desc,
            id,
            name,
            provides,
            result_type_tag,
            requires,
            modifies,
            depends_on,
        }
    }
}

impl Drop for CPluginCompositePass {
    fn drop(&mut self) {
        unsafe { (self.desc.free)(self.desc.user_data) }
    }
}

impl CompositePass for CPluginCompositePass {
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

    fn requires(&self) -> &[String] {
        &self.requires
    }

    fn modifies(&self) -> EnumSet<ModelPart> {
        self.modifies
    }

    fn depends_on(&self) -> EnumSet<ModelPart> {
        self.depends_on
    }

    fn key<T>() -> AnalysisKey<T> {
        // Consumers read a plugin composite's result by name (`require_analysis(name)`), never
        // through this typed key, so it is never invoked for a plugin pass.
        AnalysisKey::new(String::new())
    }

    fn forward(
        &self,
        model: &mut Model,
        ctx: &PassContext,
    ) -> TranspileKindResult<(Self::Artifact, Self::Result)> {
        // Move the model into an owned handle the plugin mutates via `lm_*`, then reclaim it.
        let handle = new_handle::<LmModel>(std::mem::take(model));
        let ctx_ptr = (ctx.manager() as *const AnalysisManager).cast::<LmPassContext>();
        // The plugin emits two independent payloads: the inverse artifact and the analysis result.
        let mut artifact_bytes: Vec<u8> = Vec::new();
        let mut result_bytes: Vec<u8> = Vec::new();
        // Cleared right before the call so a stale error from an unrelated earlier call is never
        // misattributed to this one; see marshal::plugin_error.
        lunamodel_capi::error::clear_last();

        let status = unsafe {
            (self.desc.forward)(
                self.desc.user_data,
                handle,
                ctx_ptr,
                byte_sink,
                (&mut artifact_bytes as *mut Vec<u8>).cast::<c_void>(),
                byte_sink,
                (&mut result_bytes as *mut Vec<u8>).cast::<c_void>(),
            )
        };
        *model = reclaim::<LmModel, Model>(handle, "model", "composite forward")?;
        if status != LmStatus::Ok {
            return Err(plugin_failure(
                &self.id,
                &format!("plugin composite '{}' forward returned {status:?}", self.id),
            ));
        }
        Ok((
            CPluginArtifact {
                plugin_pass_id: self.id.clone(),
                payload: artifact_bytes,
            },
            SerializedAnalysis {
                type_tag: self.result_type_tag.clone(),
                bytes: result_bytes,
            },
        ))
    }
}

impl Reversible for CPluginCompositePass {
    type Artifact = CPluginArtifact;

    const ID: &'static str = "luna_model::CPluginCompositePass";

    fn backward(artifact: &Self::Artifact, solution: Solution) -> TranspileKindResult<Solution> {
        let (func, user_data) = registry::lookup(&artifact.plugin_pass_id).ok_or(
            TranspileErrorKind::UnregisteredPass {
                name: artifact.plugin_pass_id.clone(),
            },
        )?;
        // Transfer ownership of the solution to the plugin, and reclaim the one it produces.
        let in_handle = new_handle::<LmSol>(solution);
        let mut out_handle: *mut LmSol = std::ptr::null_mut();
        // Cleared right before the call so a stale error from an unrelated earlier call is never
        // misattributed to this one; see marshal::plugin_error.
        lunamodel_capi::error::clear_last();

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
            return Err(plugin_failure(
                &artifact.plugin_pass_id,
                &format!(
                    "plugin composite '{}' backward returned {status:?}",
                    artifact.plugin_pass_id
                ),
            ));
        }
        reclaim::<LmSol, Solution>(out_handle, "solution", "composite backward")
    }
}
