use aqm_macros::register_pytransformations;

use super::py_ir::{PyIR, PyLogElement};
use super::py_pass_manager::PyPassManager;
use super::py_passes::{PyAnalysisPass, PyPass, PyTransformationPass};

use crate::transformations::analysis_cache::PyAnalysisCache;
use crate::transformations::base_passes::Pass;
use crate::transformations::base_passes::TransformationType;
use crate::transformations::passes::change_sense::PyChangeSensePass;
use crate::transformations::passes::max_bias::{MaxBias, PyMaxBiasAnalysis};

register_pytransformations!(
    specials = {PyAnalysisPass, PyTransformationPass},
    extras = {PyAnalysisCache, PyPassManager, TransformationType, MaxBias, PyIR, PyLogElement},
    passes = {
        PyChangeSensePass, PyMaxBiasAnalysis
    },
);
