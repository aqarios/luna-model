use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bound {
    Bounded(f64),
    Unbounded,
}

impl Bound {
    pub fn is_bounded(&self) -> bool {
        match self {
            Self::Bounded(_) => true,
            Self::Unbounded => false,
        }
    }
}

impl Display for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Bounded(v) => write!(f, "{}", v),
            Self::Unbounded => write!(f, "Unbounded"),
        }
    }
}
