#[macro_export]
macro_rules! subs {
    ($L:ty => $( $R:ty ) ,+ $(,)?) => {
        // std::ops::Sub
        // L: owned + R: owned
        $( subs!(@add_oo $L, $R); )*
        // L: owned + R: borrowed
        $( subs!(@add_ob $L, $R); )*
        // L: borrowed + R: owned
        $( subs!(@add_bo $L, $R); )*
        // L: borrowed + R: borrowed
        $( subs!(@add_bb $L, $R); )*
        // std::ops::SubAssign
        $( subs!(@add_assign_o $L, $R); )*
    };

    // sub rules for each trait
    // L: owned + R: owned
    (@sub_oo $L:ty, $R:ty) => {
        impl std::ops::Sub<$R> for $L
        where
            for<'r> $L: crate::ops::LmSubAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<crate::expression::Expression>;
            fn sub(mut self, rhs: $R) -> Self::Output {
                use crate::ops::LmSubAssign;
                <$L as LmSubAssign<&$R>>::sub_assign(&mut self, &rhs)?;
                Ok(self)
            }
        }
    };
    // L: owned + R: borrowed
    (@sub_ob $L:ty, $R:ty) => {
        impl std::ops::Sub<&$R> for $L
        where
            for<'r> $L: crate::ops::LmSubAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<crate::expression::Expression>;
            fn sub(mut self, rhs: &$R) -> Self::Output {
                use crate::ops::LmSubAssign;
                <$L as LmSubAssign<&$R>>::sub_assign(&mut self, rhs)?;
                Ok(self)
            }
        }
    };
    // L: borrowed + R: owned
    (@sub_bo $L:ty, $R:ty) => {
        impl std::ops::Sub<$R> for &$L
        where
            for<'r> $L: crate::ops::LmSubAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<crate::expression::Expression>;
            fn sub(self, rhs: $R) -> Self::Output {
                use crate::ops::LmSubAssign;
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
            for<'r> $L: crate::ops::LmSubAssign<&'r $R>,
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
            for<'r> $L: crate::ops::LmSubAssign<&'r $R>,
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
        $( rsubs!(@add_oo $L, $R); )*
        // L: owned + R: borrowed
        $( rsubs!(@add_ob $L, $R); )*
        // L: borrowed + R: owned
        $( rsubs!(@add_bo $L, $R); )*
        // L: borrowed + R: borrowed
        $( rsubs!(@add_bb $L, $R); )*
    };

    // sub rules for each trait
    // L: owned + R: owned
    (@sub_oo $L:ty, $R:ty) => {
        impl std::ops::Sub<$L> for $R
        where
            for<'r> $L: crate::ops::LmSubAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<crate::expression::Expression>;
            fn sub(self, mut rhs: $L) -> Self::Output {
                use crate::ops::LmSubAssign;
                <$L as LmSubAssign<&$R>>::sub_assign(&mut rhs, &self)?;
                Ok(rhs)
            }
        }
    };
    // L: owned + R: borrowed
    (@sub_ob $L:ty, $R:ty) => {
        impl std::ops::Sub<&$L> for $R
        where
            for<'r> $L: crate::ops::LmSubAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<crate::expression::Expression>;
            fn sub(self, rhs: &$L) -> Self::Output {
                use crate::ops::LmSubAssign;
                let mut new = rhs.clone();
                <$L as LmSubAssign<&$R>>::sub_assign(&mut new, &self)?;
                Ok(new)
            }
        }
    };
    // L: borrowed + R: owned
    (@sub_bo $L:ty, $R:ty) => {
        impl std::ops::Sub<$L> for &$R
        where
            for<'r> $L: crate::ops::LmSubAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<crate::expression::Expression>;
            fn sub(self, mut rhs: $L) -> Self::Output {
                use crate::ops::LmSubAssign;
                <$L as LmSubAssign<&$R>>::sub_assign(&mut rhs, &self)?;
                Ok(rhs)
            }
        }
    };
    // L: borrowed + R: borrowed
    (@sub_bb $L:ty, $R:ty) => {
        impl std::ops::Sub<&$L> for &$R
        where
            for<'r> $L: crate::ops::LmSubAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<crate::expression::Expression>;
            fn sub(self, rhs: &$L) -> Self::Output {
                use crate::ops::LmSubAssign;
                let mut new = rhs.clone();
                <$L as LmSubAssign<&$R>>::sub_assign(&mut new, self)?;
                Ok(new)
            }
        }
    };
}
