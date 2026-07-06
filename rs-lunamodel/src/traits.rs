use crate::error::Result;

pub trait TryClone: Sized {
    /// Clones into a new handle.
    ///
    /// Named `try_clone` because the fallible C call cannot satisfy `Clone`.
    fn try_clone(&self) -> Result<Self>;
}

pub trait TryDeepClone: Sized {
    /// Context needed to deep-clone this value.
    ///
    /// Some LunaModel types, such as models and environments, can deep-clone
    /// themselves directly. Expression-like values need a target environment.
    type Target: ?Sized;

    /// Deep-clones into a new handle.
    fn try_deep_clone(&self, target: &Self::Target) -> Result<Self>;
}

pub trait ContentEquality: Sized {
    /// Content equality, ignoring identity details where the core type does.
    fn equal_contents(&self, other: &Self) -> Result<bool>;
}

pub trait Serialization: Sized {
    type Payload;

    /// Serializes to bytes.
    fn serialize(&self) -> Result<Vec<u8>>;
    /// Deserializes from bytes.
    fn deserialize(data: &[u8], env: &Self::Payload) -> Result<Self>;
}
