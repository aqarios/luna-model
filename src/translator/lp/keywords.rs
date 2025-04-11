use strum_macros::Display;

use crate::core::Vtype;

#[derive(Display)]
pub enum CommentKeywords {
    #[strum(to_string = "\\")]
    Backslash,
}

impl CommentKeywords {
    pub fn all() -> Vec<String> {
        vec![Self::Backslash.to_string()]
    }
}

#[derive(Display)]
pub enum ObjectiveKeywords {
    #[strum(to_string = "minimize")]
    Minimize,
    #[strum(to_string = "maximize")]
    Maximize,
    #[strum(to_string = "minimum")]
    Minimum,
    #[strum(to_string = "maximum")]
    Maximum,
    #[strum(to_string = "min")]
    Min,
    #[strum(to_string = "max")]
    Max,
}

impl ObjectiveKeywords {
    pub fn all_min() -> Vec<String> {
        vec![
            Self::Minimize.to_string(),
            Self::Minimum.to_string(),
            Self::Min.to_string(),
        ]
    }
    pub fn all_max() -> Vec<String> {
        vec![
            Self::Maximize.to_string(),
            Self::Maximum.to_string(),
            Self::Max.to_string(),
        ]
    }
}

#[derive(Display)]
pub enum ConstraintsKeywords {
    #[strum(to_string = "subject to")]
    SubjectTo,
    #[strum(to_string = "such that")]
    SuchThat,
    #[strum(to_string = "st")]
    St,
    #[strum(to_string = "s.t.")]
    Sdtd,
}

impl ConstraintsKeywords {
    pub fn all() -> Vec<String> {
        vec![
            Self::SubjectTo.to_string(),
            Self::SuchThat.to_string(),
            Self::St.to_string(),
            Self::Sdtd.to_string(),
        ]
    }
}

#[derive(Display)]
pub enum BoundsKeywords {
    #[strum(to_string = "bounds")]
    Bounds,
}

impl BoundsKeywords {
    pub fn all() -> Vec<String> {
        vec![Self::Bounds.to_string()]
    }
}

#[derive(Display)]
pub enum VariableTypeKeywords {
    #[strum(to_string = "binary")]
    Binary,
    #[strum(to_string = "binaries")]
    Binaries,
    #[strum(to_string = "bin")]
    Bin,
    #[strum(to_string = "general")]
    General,
    #[strum(to_string = "generals")]
    Generals,
    #[strum(to_string = "gen")]
    Gen,
    #[strum(to_string = "semi-continuous")]
    SemiContinuous,
    #[strum(to_string = "semis")]
    Semis,
    #[strum(to_string = "semi")]
    Semi,
}

impl VariableTypeKeywords {
    pub fn all_bin() -> Vec<String> {
        vec![
            Self::Binary.to_string(),
            Self::Binaries.to_string(),
            Self::Bin.to_string(),
        ]
    }
    pub fn all_gen() -> Vec<String> {
        vec![
            Self::General.to_string(),
            Self::Generals.to_string(),
            Self::Gen.to_string(),
        ]
    }
    pub fn all_semi() -> Vec<String> {
        vec![
            Self::SemiContinuous.to_string(),
            Self::Semis.to_string(),
            Self::Semi.to_string(),
        ]
    }
}

#[derive(Copy, Display, Hash, Eq, PartialEq, Clone, Debug)]
pub enum VariableType {
    Binary,
    General,
    Semi,
}

impl Into<Vtype> for VariableType {
    fn into(self) -> Vtype {
        match self {
            Self::Binary => Vtype::Binary,
            Self::Semi => Vtype::Real,
            Self::General => Vtype::Integer,
        }
    }
}

#[derive(Display)]
pub enum EndKeywords {
    #[strum(to_string = "end")]
    End,
}

impl EndKeywords {
    pub fn all() -> Vec<String> {
        vec![Self::End.to_string()]
    }
}
