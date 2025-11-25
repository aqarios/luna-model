use lunamodel_error::LunaModelResult;

/// Custom Add which might fail with an error.
pub trait MaybeAdd<Rhs = Self> {
    type Outitem;

    fn add(self, rhs: Rhs) -> LunaModelResult<Self::Outitem>;
}

/// Custom RAdd which might fail with an error.
pub trait MaybeRAdd<Lhs> {
    type Outitem;

    fn radd(self, lhs: Lhs) -> LunaModelResult<Self::Outitem>;
}

/// Custom AddAssign which might fail with an error.
pub trait MaybeAddAssign<Rhs = Self> {
    fn add_assign(&mut self, rhs: Rhs) -> LunaModelResult<()>;
}
