use lunamodel_error::LunaModelResult;

/// Custom Sub which might fail with an error.
pub trait MaybeSub<Rhs> {
    type Outitem;

    fn sub(self, rhs: Rhs) -> LunaModelResult<Self::Outitem>;
}

/// Custom RSub which might fail with an error.
pub trait MaybeRSub<Lhs> {
    type Outitem;

    fn rsub(self, lhs: Lhs) -> LunaModelResult<Self::Outitem>;
}

/// Custom SubAssign which might fail with an error.
pub trait MaybeSubAssign<Rhs> {
    fn sub_assign(&mut self, rhs: Rhs) -> LunaModelResult<()>;
}
