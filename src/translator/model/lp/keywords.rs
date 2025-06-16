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
    #[strum(to_string = "Minimize")]
    Minimize,
    #[strum(to_string = "Maximize")]
    Maximize,
    #[strum(to_string = "Minimum")]
    Minimum,
    #[strum(to_string = "Maximum")]
    Maximum,
    #[strum(to_string = "Min")]
    Min,
    #[strum(to_string = "Max")]
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
    #[strum(to_string = "Subject To")]
    SubjectTo,
    #[strum(to_string = "Such That")]
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
    #[strum(to_string = "Bounds")]
    Bounds,
}

impl BoundsKeywords {
    pub fn all() -> Vec<String> {
        vec![Self::Bounds.to_string()]
    }
}

#[derive(Display)]
pub enum VariableTypeKeywords {
    #[strum(to_string = "Binary")]
    Binary,
    #[strum(to_string = "Binaries")]
    Binaries,
    #[strum(to_string = "Bin")]
    Bin,
    #[strum(to_string = "General")]
    General,
    #[strum(to_string = "Generals")]
    Generals,
    #[strum(to_string = "Gen")]
    Gen,
    // #[strum(to_string = "Semi-Continuous")]
    // SemiContinuous,
    // #[strum(to_string = "Semis")]
    // Semis,
    // #[strum(to_string = "Semi")]
    // Semi,
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
    // pub fn all_semi() -> Vec<String> {
    //     vec![
    //         Self::SemiContinuous.to_string(),
    //         Self::Semis.to_string(),
    //         Self::Semi.to_string(),
    //     ]
    // }
}

#[derive(Copy, Display, Hash, Eq, PartialEq, Clone, Debug)]
pub enum VariableType {
    #[strum(to_string = "Binaries")]
    Binary,
    #[strum(to_string = "Generals")]
    General,
    #[strum(to_string = "Continuous")]
    Continuous,
    // #[strum(to_string = "Semi-Continuous")]
    // Semi,
}

impl Into<Vtype> for VariableType {
    fn into(self) -> Vtype {
        match self {
            Self::Binary => Vtype::Binary,
            Self::Continuous => Vtype::Real,
            Self::General => Vtype::Integer,
        }
    }
}

#[derive(Display)]
pub enum EndKeywords {
    #[strum(to_string = "End")]
    End,
}

impl EndKeywords {
    pub fn all() -> Vec<String> {
        vec![Self::End.to_string()]
    }
}
