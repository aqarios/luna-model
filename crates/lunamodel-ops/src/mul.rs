use lunamodel_error::LunaModelResult;

/// Custom Mul which might fail with an error.
pub trait MaybeMul<Rhs> {
    type Outitem;

    fn mul(self, rhs: Rhs) -> LunaModelResult<Self::Outitem>;
}

/// Custom RMul which might fail with an error.
pub trait MaybeRMul<Lhs> {
    type Outitem;

    fn rmul(self, lhs: Lhs) -> LunaModelResult<Self::Outitem>;
}

/// Custom MulAssign which might fail with an error.
pub trait MaybeMulAssign<Rhs> {
    fn mul_assign(&mut self, rhs: Rhs) -> LunaModelResult<()>;
}
