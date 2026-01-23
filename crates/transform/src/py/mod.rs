mod ir;
mod log;
mod passes;
mod pm;
mod cache;

use lunamodel_tpass::register_pytransformations;

use ir::PyIR;
use log::PyLogElement;
use pm::PyPassManager;

use passes::{
    PyBasePass, PyPass,
    analysis::{PyAnalysisPass, PyMetaAnalysisPass},
    special::{PyIfElsePass, PyPipeline},
    transformation::{PyTransformationPass, StructuredPyTransformationOutcome},
};

use crate::base::ActionType;
use crate::cache::PyAnalysisCache;
use crate::passes::{
    BinarySpinInfo, MaxBias, PyBinarySpinPass, PyChangeSensePass, PyMaxBiasAnalysis,
};

register_pytransformations!(
    specials = {PyAnalysisPass, PyTransformationPass, PyPipeline, PyMetaAnalysisPass},
    extras = {PyAnalysisCache, PyPassManager, ActionType, MaxBias, PyIR, PyLogElement, BinarySpinInfo, StructuredPyTransformationOutcome, PyBasePass},
    passes = {
        PyChangeSensePass, PyMaxBiasAnalysis, PyBinarySpinPass, PyIfElsePass
    },
);
