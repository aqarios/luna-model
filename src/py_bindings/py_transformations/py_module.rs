use aqm_macros::register_pytransformations;

use super::py_passes::{PyAnalysisPass, PyPass, PyTransformationPass};
use super::py_pass_manager::PyPassManager;

use crate::transformations::base_passes::Pass;
use crate::transformations::base_passes::TransformationType;
use crate::transformations::passes::change_sense::PyChangeSensePass;
use crate::transformations::passes::max_bias::{PyMaxBiasAnalysis, MaxBias};
use crate::transformations::analysis_cache::PyAnalysisCache;

register_pytransformations!(
    specials = {PyAnalysisPass, PyTransformationPass},
    extras = {PyAnalysisCache, PyPassManager, TransformationType, MaxBias},
    passes = {
        PyChangeSensePass, PyMaxBiasAnalysis
    },
);
