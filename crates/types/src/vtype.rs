//! Variable-type enum definitions.

use enumset::{EnumSet, EnumSetType};
use strum_macros::{Display, EnumIter, EnumString};

use crate::utils::EnumSetFromVec;

/// Enumeration of variables types supported by the optimization system.
#[derive(Debug, Display, Hash, EnumSetType, EnumIter, EnumString, Default)]
pub enum Vtype {
    /// A binary variable that can take values 0 or 1.
    #[default]
    Binary,
    /// An inverted binary variable (`!b == 1 - b`) that is not optimized itself and it's value
    /// depends on the value of the corresponding [Vtype::Binary] variable.
    InvertedBinary,
    /// Spin variable that can take values -1 or +1.
    Spin,
    /// Discrete integer-valued variable that take integer values within bounds.
    Integer,
    /// Continuous real-valued variable that take any value within given bounds.
    Real,
}

impl EnumSetFromVec<Vtype> for Vec<Vtype> {
    /// Converts a list of variable types into an [`EnumSet`].
    fn to_enumset(&self) -> EnumSet<Vtype> {
        let mut es = EnumSet::default();
        for entry in self.iter() {
            _ = es.insert(*entry);
        }
        es
    }
}
