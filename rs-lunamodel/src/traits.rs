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
    fn deep_clone(&self, target: &Self::Target) -> Result<Self>;
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

pub trait TryAddAssign<Rhs = Self> {
    fn try_add_assign(&mut self, rhs: Rhs) -> Result<()>;
}

pub trait TrySubAssign<Rhs = Self> {
    fn try_sub_assign(&mut self, rhs: Rhs) -> Result<()>;
}

pub trait TryMulAssign<Rhs = Self> {
    fn try_mul_assign(&mut self, rhs: Rhs) -> Result<()>;
}

pub trait TryToString {
    fn try_to_string(self) -> Result<String>;
}

pub trait TryToVec<T> {
    fn try_to_vec(self) -> Result<Vec<T>>;
}

pub trait LmFrom<T> {
    fn lm_from(value: T) -> Self;
}

impl<T, U> LmInto<U> for T
where
    U: LmFrom<T>,
{
    fn lm_into(self) -> U {
        U::lm_from(self)
    }
}

pub trait LmInto<T> {
    fn lm_into(self) -> T;
}
