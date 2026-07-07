use crate::error::Result;

pub trait TryClone: Sized {
    /// Clones into a new handle.
    ///
    /// Named `try_clone` because the fallible C call cannot satisfy `Clone`.
    fn try_clone(&self) -> Result<Self>;
}

pub trait TryDeepClone<'a>: Sized {
    /// Context needed to deep-clone this value.
    ///
    /// Some LunaModel types, such as models and environments, can deep-clone
    /// themselves directly. Expression-like values need a target environment.
    type Target: ?Sized;

    /// Deep-clones into a new handle.
    fn try_deep_clone(&self, target: Self::Target) -> Result<Self>;
}

pub trait ContentEquality: Sized {
    /// Content equality, ignoring identity details where the core type does.
    fn equal_contents(&self, other: &Self) -> Result<bool>;
}

pub trait Serialization<'s>: Sized {
    type Payload;

    /// Serializes to bytes.
    fn serialize(&self) -> Result<Vec<u8>>;
    /// Deserializes from bytes.
    fn deserialize(data: &[u8], env: Self::Payload) -> Result<Self>;
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

pub trait LmTryInto<T> {
    fn lm_try_into(self) -> Result<T>;
}

pub trait LmTryFrom<T>: Sized {
    fn lm_try_from(value: T) -> Result<Self>;
}

impl<T, U> LmTryInto<U> for T
where
    U: LmTryFrom<T>,
{
    fn lm_try_into(self) -> Result<U> {
        U::lm_try_from(self)
    }
}

/// Identity conversion so that scalar ABI out-values (e.g. `f64`, `bool`,
/// `usize`) pass through `out!`'s `lm_try_into()` unchanged.
impl<T> LmTryFrom<T> for T {
    fn lm_try_from(value: T) -> Result<Self> {
        Ok(value)
    }
}

/// Converts an optional value into a nullable C pointer:
/// `Some(v)` yields a pointer to `v`, `None` yields null.
pub(crate) trait AsOptPtr<T> {
    fn as_opt_ptr(&self) -> *const T;
}

impl<T> AsOptPtr<T> for Option<T> {
    fn as_opt_ptr(&self) -> *const T {
        match self {
            Some(v) => std::ptr::from_ref(v),
            None => std::ptr::null(),
        }
    }
}

/// Converts a value into an owned C string.
///
/// Returns the owning [`CString`], not a raw pointer: the bytes must stay alive
/// across the FFI call, so the caller keeps the owner. The `!` macro sigil does
/// this by calling `.as_ptr()` on the temporary it produces, whose lifetime
/// extends to the end of the enclosing ABI-call statement.
pub trait TryToCString {
    fn try_to_cstring(self) -> Result<std::ffi::CString>;
}

/// Optional variant of [`TryToCString`]; a `None` input maps to a null pointer
/// at the call site (via the `?!` macro sigil).
pub(crate) trait TryToOptCString {
    fn try_to_opt_cstring(self) -> Result<Option<std::ffi::CString>>;
}

/// A contiguous array of `*const T` C pointers, plus any owned backing the
/// pointers borrow from (e.g. the `CString`s for a `&[String]`).
///
/// Built by [`ToPtrList`] and kept alive as a statement-scoped temporary across
/// the FFI call; dropping it frees both the pointer array and its backing.
pub struct PtrList<T> {
    ptrs: Vec<*const T>,
    /// Owned C strings the pointers borrow from; empty when the elements are
    /// handles (whose pointers are owned by the borrowed slice instead).
    _owners: Vec<std::ffi::CString>,
}

impl<T> PtrList<T> {
    pub(crate) fn from_ptrs(ptrs: Vec<*const T>, owners: Vec<std::ffi::CString>) -> Self {
        Self {
            ptrs,
            _owners: owners,
        }
    }

    /// Pointer to the start of the `*const T` array. The length is passed
    /// separately by the `*` sigil, taken from the source slice's `.len()`.
    pub fn as_ptr(&self) -> *const *const T {
        self.ptrs.as_ptr()
    }
}

/// Marshals a slice into a [`PtrList`] of C pointers. Used by the `*` macro
/// sigil, which passes the list's `.as_ptr()` and the slice's `.len()` as two
/// consecutive ABI arguments.
pub trait ToPtrList {
    /// The pointee type of the produced pointers (e.g. `LmVref`, `c_char`).
    type Elem;
    fn to_ptr_list(self) -> Result<PtrList<Self::Elem>>;
}
