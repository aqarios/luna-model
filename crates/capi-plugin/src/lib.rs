use std::sync::Once;

mod analysis;
mod artifact;
mod composite;
mod control_flow;
mod marshal;
mod meta_analysis;
mod pass;
mod pipeline;
mod registry;

pub use analysis::CPluginAnalysisPass;
pub use artifact::CPluginArtifact;
pub use composite::CPluginCompositePass;
pub use control_flow::CPluginControlFlowPass;
pub use meta_analysis::CPluginMetaAnalysisPass;
pub use pass::CPluginTransformPass;
pub use pipeline::StepListView;
pub use registry::register;

static INIT: Once = Once::new();

/// Registers the plugin adapter types with the transpiler backward registry.
/// Idempotent; called from every plugin entry point.
pub fn ensure_registered() {
    INIT.call_once(|| {
        lunamodel_transpiler::register_backward::<CPluginTransformPass>();
        lunamodel_transpiler::register_backward::<CPluginCompositePass>();
    });
}
