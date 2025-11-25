use enumset::{EnumSet, EnumSetType};

pub trait EnumSetFromVec<T: EnumSetType> {
    fn to_enumset(&self) -> EnumSet<T>;
}
