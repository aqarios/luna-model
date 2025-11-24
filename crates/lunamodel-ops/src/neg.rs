use lunamodel_error::LunaModelResult;

/// Custom Mul which might fail with an error.
pub trait MaybeNeg {
    type Outitem;

    fn mul(self) -> LunaModelResult<Self::Outitem>;
}
