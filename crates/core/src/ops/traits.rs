use lunamodel_error::LunaModelResult;

/// Custom Pow which might fail with an error.
pub trait LmPow {
    type Output;
    fn pow(self, sup: usize) -> LunaModelResult<Self::Output>;
}

/// Custom PowAssign which might fail with an error.
pub trait LmPowAssign {
    fn pow_assign(&mut self, sup: usize) -> LunaModelResult<()>;
}

/// Custom [std::ops::AddAssign] which might fail with an error.
pub trait LmAddAssign<Rhs> {
    fn add_assign(&mut self, rhs: Rhs) -> LunaModelResult<()>;
}

/// Custom [std::ops::MulAssign] which might fail with an error.
pub trait LmMulAssign<Rhs> {
    fn mul_assign(&mut self, rhs: Rhs) -> LunaModelResult<()>;
}

/// Custom [std::ops::SubAssign] which might fail with an error.
pub trait LmSubAssign<Rhs> {
    fn sub_assign(&mut self, rhs: Rhs) -> LunaModelResult<()>;
}

pub(super) mod internal {
    // pub trait PrvAdd<Rhs> {
    //     type Output;
    //     fn a(self, rhs: Rhs) -> Self::Output;
    // }
    pub trait PrvAddAssign<Rhs> {
        fn aa(&mut self, rhs: Rhs);
    }
    pub trait PrvMul<Rhs> {
        type Output;
        fn m(self, rhs: Rhs) -> Self::Output;
    }
    // pub trait PrvMulAssign<Rhs> {
    //     fn ma(&mut self, rhs: Rhs);
    // }
}
