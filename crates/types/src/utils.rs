use enumset::{EnumSet, EnumSetType};

/// Helper trait for converting collections of enums into [`EnumSet`]s.
pub trait EnumSetFromVec<T: EnumSetType> {
    /// Builds the corresponding enum set.
    fn to_enumset(&self) -> EnumSet<T>;
}
