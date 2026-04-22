mod analysis;
mod control_flow;
mod transform;
mod composite;
mod meta;

pub use analysis::ErasedAnalysisPass;
pub use control_flow::ErasedControlFlowPass;
pub use transform::ErasedTransformPass;
pub use composite::ErasedCompositePass;
pub use meta::ErasedMetaAnalysisPass;
