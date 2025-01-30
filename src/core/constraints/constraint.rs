use super::{expression::Expression, term::number::Number};

pub enum Comparator {
    Eq,
    Leq,
    Geq,
}

pub struct Constraint {
    lhs: Expression,
    rhs: Number,
    comparator: Comparator,
}

pub struct Constraints {
    constraints: Vec<Constraint>,
}

impl Constraints {
    pub fn empty() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }
}
