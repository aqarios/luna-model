use crate::core::VarRef;

#[derive(Debug, Clone, Copy)]
pub enum PrintLayout {
    Row,
    Col,
}

#[derive(Debug, Clone, Copy)]
pub enum ShowMetadata {
    Before,
    After,
    Hide,
}

pub enum VarKey<'a> {
    Name(String),
    Var(&'a VarRef),
}
