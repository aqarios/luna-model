use std::sync::Once;

mod analysis;
mod artifact;
mod marshal;
mod pass;
mod registry;

pub use analysis::CPluginAnalysisPass;
pub use artifact::CPluginArtifact;
pub use pass::CPluginTransformPass;
pub use registry::register_transform;

static INIT: Once = Once::new();

/// Registers the plugin adapter types with the transpiler backward registry.
/// Idempotent; called from every plugin entry point.
pub fn ensure_registered() {
    INIT.call_once(|| {
        lunamodel_transpiler::register_backward::<CPluginTransformPass>();
    });
}
