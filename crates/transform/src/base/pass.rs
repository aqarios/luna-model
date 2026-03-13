use std::fmt::Display;

use super::analysis::AnalysisPass;
use super::transformation::TransformationPass;
use crate::{
    base::BasePass,
    passes::special::{AbstractPipeline, IfElsePass, MetaAnalysisPass},
};

#[derive(Debug)]
pub enum Pass {
    Transformation(Box<dyn TransformationPass>),
    Analysis(Box<dyn AnalysisPass>),
    IfElse(IfElsePass),
    Pipeline(Box<dyn AbstractPipeline>),
    MetaAnalysis(Box<dyn MetaAnalysisPass>),
}

impl Pass {
    pub fn name(&self) -> String {
        match self {
            Self::Analysis(x) => x.name(),
            Self::Transformation(x) => x.name(),
            Self::IfElse(x) => x.name(),
            Self::Pipeline(x) => x.name(),
            Self::MetaAnalysis(x) => x.name(),
        }
    }

    pub fn requires(&self) -> Vec<String> {
        match self {
            Self::Analysis(x) => x.requires(),
            Self::Transformation(x) => x.requires(),
            Self::IfElse(x) => x.requires(),
            Self::Pipeline(x) => x.requires(),
            Self::MetaAnalysis(x) => x.requires(),
        }
    }
}

impl Clone for Pass {
    fn clone(&self) -> Self {
        match self {
            Self::IfElse(x) => Self::IfElse(x.clone()),
            Self::Pipeline(x) => Self::Pipeline(x.clone()),
            Self::Transformation(x) => Self::Transformation(x.clone()),
            Self::Analysis(x) => Self::Analysis(x.clone()),
            Self::MetaAnalysis(x) => Self::MetaAnalysis(x.clone()),
        }
    }
}

impl Display for Pass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transformation(x) => write!(f, "{}", x),
            Self::Analysis(x) => write!(f, "{}", x),
            Self::IfElse(x) => write!(f, "{}", x),
            Self::Pipeline(x) => write!(f, "{}", x),
            Self::MetaAnalysis(x) => write!(f, "{}", x),
        }
    }
}

// impl IntoAnyPass for Pass {
//     fn as_anypass(&self) -> AnyPass {
//         match self {
//             Self::Transformation(x) => x.as_anypass(),
//             Self::Analysis(x) => x.as_anypass(),
//             Self::IfElse(x) => x.as_anypass(),
//             Self::Pipeline(x) => x.as_anypass(),
//             Self::MetaAnalysis(x) => x.as_anypass(),
//         }
//     }
// }

// #[cfg(feature = "py")]
// impl AsPyPass for Pass {
//     type PyPass = ;
//
//     fn as_pypass(&self) -> PyPass {
//         match self {
//             Self::Transformation(x) => x.as_pypass(),
//             Self::Analysis(x) => x.as_pypass(),
//             Self::IfElse(x) => x.as_pypass(),
//             Self::Pipeline(x) => x.as_pypass(),
//             Self::MetaAnalysis(x) => x.as_pypass(),
//         }
//     }
// }
