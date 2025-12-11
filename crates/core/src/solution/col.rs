use lunamodel_types::{BinaryAssignment, IntegerAssignment, RealAssignment, SpinAssignment, VarIdx};

#[derive(Debug, Clone, PartialEq)]
pub struct ColElement<T>(pub Vec<T>);

#[derive(Debug, Clone, PartialEq)]
pub enum Column {
    Binary(ColElement<BinaryAssignment>),
    Spin(ColElement<SpinAssignment>),
    Integer(ColElement<IntegerAssignment>),
    Real(ColElement<RealAssignment>),
}

impl Column {
    // pub fn get(&self, idx: usize) -> Option<>
}
