mod unicode;

mod base;
mod cache;
mod execution;
mod ir;
mod log;
mod pass_manager;

pub mod passes;
pub use base::Pass;
pub use base::{BasePass, TransformationOutcome, TransformationPass, TransformationPassResult, ActionType, AnalysisPass, AnalysisPassResult};
pub use cache::{AnalysisCache, AnalysisCacheElement};
pub use ir::IR;
pub use pass_manager::PassManager;

// #[cfg(feature = "py")]
// pub mod py;
//
// use lunamodel_tpass::register_pytransformations;

// #[cfg(feature = "py")]
// register_pytransformations!(
//     specials = {PyAnalysisPass, PyTransformationPass, PyPipeline, PyMetaAnalysisPass},
//     extras = {PyAnalysisCache, PyPassManager, ActionType, MaxBias, PyIR, PyLogElement, BinarySpinInfo, StructuredPyTransformationOutcome, PyBasePass},
//     passes = {
//         PyChangeSensePass, PyMaxBiasAnalysis, PyBinarySpinPass, PyIfElsePass
//     },
// );
