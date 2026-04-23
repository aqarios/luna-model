#[macro_export]
macro_rules! subs {
    ($L:ty => $( $R:ty ) ,+ $(,)?) => {
        // std::ops::Sub
        // L: owned + R: owned
        $( subs!(@sub_oo $L, $R); )*
        // L: owned + R: borrowed
        $( subs!(@sub_ob $L, $R); )*
        // L: borrowed + R: owned
        $( subs!(@sub_bo $L, $R); )*
        // L: borrowed + R: borrowed
        $( subs!(@sub_bb $L, $R); )*
        // std::ops::SubAssign
        $( subs!(@sub_assign_o $L, $R); )*
    };

    // sub rules for each trait
    // L: owned + R: owned
    (@sub_oo $L:ty, $R:ty) => {
        impl std::ops::Sub<$R> for $L
        where
            for<'r> $L: $crate::ops::LmSubAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn sub(mut self, rhs: $R) -> Self::Output {
                use $crate::ops::LmSubAssign;
                <$L as LmSubAssign<&$R>>::sub_assign(&mut self, &rhs)?;
                Ok(self)
            }
        }
    };
    // L: owned + R: borrowed
    (@sub_ob $L:ty, $R:ty) => {
        impl std::ops::Sub<&$R> for $L
        where
            for<'r> $L: $crate::ops::LmSubAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn sub(mut self, rhs: &$R) -> Self::Output {
                use $crate::ops::LmSubAssign;
                <$L as LmSubAssign<&$R>>::sub_assign(&mut self, rhs)?;
                Ok(self)
            }
        }
    };
    // L: borrowed + R: owned
    (@sub_bo $L:ty, $R:ty) => {
        impl std::ops::Sub<$R> for &$L
        where
            for<'r> $L: $crate::ops::LmSubAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn sub(self, rhs: $R) -> Self::Output {
                use $crate::ops::LmSubAssign;
                let mut new = self.clone();
                <$L as LmSubAssign<&$R>>::sub_assign(&mut new, &rhs)?;
                Ok(new)
            }
        }
    };
    // L: borrowed + R: borrowed
    (@sub_bb $L:ty, $R:ty) => {
        impl std::ops::Sub<&$R> for &$L
        where
            for<'r> $L: $crate::ops::LmSubAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<crate::expression::Expression>;
            fn sub(self, rhs: &$R) -> Self::Output {
                use crate::ops::LmSubAssign;
                let mut new = self.clone();
                <$L as LmSubAssign<&$R>>::sub_assign(&mut new, rhs)?;
                Ok(new)
            }
        }
    };
    // LmSubAssign for owned R.
    (@sub_assign_o $L:ty, $R:ty) => {
        impl crate::ops::LmSubAssign<$R> for $L
        where
            for<'r> $L: $crate::ops::LmSubAssign<&'r $R>,
        {
            fn sub_assign(&mut self, rhs: $R) -> lunamodel_error::LunaModelResult<()> {
                use crate::ops::LmSubAssign;
                <$L as LmSubAssign<&$R>>::sub_assign(self, &rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! rsubs {
    ($L:ty => $( $R:ty ) ,+ $(,)?) => {
        // std::ops::Sub
        // L: owned + R: owned
        $( rsubs!(@sub_oo $L, $R); )*
        // L: owned + R: borrowed
        $( rsubs!(@sub_ob $L, $R); )*
        // L: borrowed + R: owned
        $( rsubs!(@sub_bo $L, $R); )*
        // L: borrowed + R: borrowed
        $( rsubs!(@sub_bb $L, $R); )*
    };

    // sub rules for each trait
    // L: owned + R: owned
    // R - L = R + (-L)
    (@sub_oo $L:ty, $R:ty) => {
        impl std::ops::Sub<$L> for $R
        where
            for<'r> $L: $crate::ops::LmAddAssign<&'r $R> + std::ops::Neg,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn sub(self, lhs: $L) -> Self::Output {
                let mut tmp = -lhs;
                <$L as LmAddAssign<&$R>>::add_assign(&mut tmp, &self)?;
                Ok(tmp)
            }
        }
    };
    // L: owned + R: borrowed
    // R - L = R + (-L)
    (@sub_ob $L:ty, $R:ty) => {
        impl std::ops::Sub<&$L> for $R
        where
            for<'r> $L: $crate::ops::LmAddAssign<&'r $R> + std::ops::Neg,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn sub(self, lhs: &$L) -> Self::Output {
                let mut new = -lhs;
                <$L as LmAddAssign<&$R>>::add_assign(&mut new, &self)?;
                Ok(new)
            }
        }
    };
    // L: borrowed + R: owned
    // R - L = R + (-L)
    (@sub_bo $L:ty, $R:ty) => {
        impl std::ops::Sub<$L> for &$R
        where
            for<'r> $L: $crate::ops::LmAddAssign<&'r $R> + std::ops::Neg,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn sub(self, lhs: $L) -> Self::Output {
                let mut tmp = -lhs;
                <$L as LmAddAssign<&$R>>::add_assign(&mut tmp, &self)?;
                Ok(tmp)
            }
        }
    };
    // L: borrowed + R: borrowed
    // R - L = R + (-L)
    (@sub_bb $L:ty, $R:ty) => {
        impl std::ops::Sub<&$L> for &$R
        where
            for<'r> $L: $crate::ops::LmAddAssign<&'r $R> + std::ops::Neg,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn sub(self, rhs: &$L) -> Self::Output {
                let mut new = -rhs;
                <$L as LmAddAssign<&$R>>::add_assign(&mut new, self)?;
                Ok(new)
            }
        }
    };
}
