use strum_macros::Display;

#[derive(Debug, Display, Hash, PartialEq)]
pub enum TranslationTarget {
    Qubo,
    Lp,
    Bqm,
    Cqm,
}
