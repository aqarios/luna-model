use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ValueSource {
    Raw,
    Obj,
}
impl Display for ValueSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw => f.write_str("raw_energies"),
            Self::Obj => f.write_str("obj_values"),
        }
    }
}
