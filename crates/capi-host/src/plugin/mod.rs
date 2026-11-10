//! C-ABI plugin registration entry points.
//!
//! The adapter types (`CPluginTransformPass`, `CPluginArtifact`) live in the leaf crate
//! `lunamodel-capi-plugin`, shared with `lunamodel-python`; this module holds only the C entry
//! point that fills the API table and re-exports the adapters for convenience.

pub mod capi;
pub mod control_flow_plan;
pub mod pass_context;
pub mod step_list;

pub use capi::lm_register_plugin_backward;
pub use control_flow_plan::{
    lm_control_flow_plan_add_analysis, lm_control_flow_plan_add_composite,
    lm_control_flow_plan_add_control_flow, lm_control_flow_plan_add_meta_analysis,
    lm_control_flow_plan_add_transform, lm_control_flow_plan_free, lm_control_flow_plan_new,
};
pub use lunamodel_capi_plugin::{
    CPluginAnalysisPass, CPluginArtifact, CPluginCompositePass, CPluginControlFlowPass,
    CPluginMetaAnalysisPass, CPluginTransformPass,
};
pub use pass_context::lm_pass_context_get_analysis;
pub use step_list::{
    lm_step_list_kind, lm_step_list_len, lm_step_list_modifies, lm_step_list_name,
    lm_step_list_provides, lm_step_list_requires,
};
