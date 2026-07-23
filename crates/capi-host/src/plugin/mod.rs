//! C-ABI plugin registration entry points.
//!
//! The adapter types (`CPluginTransformPass`, `CPluginArtifact`) live in the leaf crate
//! `lunamodel-capi-plugin`, shared with `lunamodel-python`; this module holds only the C entry
//! point that fills the API table and re-exports the adapters for convenience.

pub mod capi;
pub mod pass_context;

pub use capi::lm_register_plugin_backward;
pub use lunamodel_capi_plugin::{CPluginAnalysisPass, CPluginArtifact, CPluginTransformPass};
pub use pass_context::lm_pass_context_get_analysis;
