//! Fallible operator traits used throughout the core algebra APIs.

use lunamodel_error::LunaModelResult;

/// Fallible exponentiation used throughout LunaModel's algebra layer.
///
/// Standard `Pow`-style traits do not exist in `std`, and the project needs
/// the operation to return a [`LunaModelResult`] because exponentiation can
/// fail for environment or type reasons.
pub trait LmPow {
    type Output;
    fn pow(self, exp: usize) -> LunaModelResult<Self::Output>;
}

/// Fallible in-place exponentiation.
pub trait LmPowAssign {
    fn pow_assign(&mut self, exp: usize) -> LunaModelResult<()>;
}

/// Fallible analogue of [`std::ops::AddAssign`].
///
/// Arithmetic on LunaModel objects often needs to validate that all operands
/// belong to the same environment and still point to live variables, so the
/// standard infallible trait is not expressive enough.
pub trait LmAddAssign<Rhs> {
    fn add_assign(&mut self, rhs: Rhs) -> LunaModelResult<()>;
}

/// Fallible analogue of [`std::ops::MulAssign`].
pub trait LmMulAssign<Rhs> {
    fn mul_assign(&mut self, rhs: Rhs) -> LunaModelResult<()>;
}

/// Fallible analogue of [`std::ops::SubAssign`].
pub trait LmSubAssign<Rhs> {
    fn sub_assign(&mut self, rhs: Rhs) -> LunaModelResult<()>;
}

pub(super) mod internal {
    /// Infallible internal addition helper used when higher-level validation
    /// has already happened and the implementation just needs to accumulate raw
    /// sparse term fragments.
    pub trait PrvAddAssign<Rhs> {
        fn aa(&mut self, rhs: Rhs);
    }

    /// Infallible internal multiplication helper.
    ///
    /// These impls work on low-level storage types and return intermediate
    /// fragments rather than full user-facing expressions. The public
    /// `LmMulAssign` / `Mul` pathways build on top of this layer.
    pub trait PrvMul<Rhs> {
        type Output;
        fn m(self, rhs: Rhs) -> Self::Output;
    }
}
