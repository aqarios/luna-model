#[macro_export]
macro_rules! muls {
    ($L:ty => $( $R:ty ) ,+ $(,)?) => {
        // std::ops::Mul
        // L: owned + R: owned
        $( muls!(@mul_oo $L, $R); )*
        // L: owned + R: borrowed
        $( muls!(@mul_ob $L, $R); )*
        // L: borrowed + R: owned
        $( muls!(@mul_bo $L, $R); )*
        // L: borrowed + R: borrowed
        $( muls!(@mul_bb $L, $R); )*
        // std::ops::MulAssign
        $( muls!(@mul_assign_o $L, $R); )*
    };

    // sub rules for each trait
    // L: owned + R: owned
    (@mul_oo $L:ty, $R:ty) => {
        impl std::ops::Mul<$R> for $L
        where
            for<'r> $L: $crate::ops::LmMulAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn mul(mut self, rhs: $R) -> Self::Output {
                use $crate::ops::LmMulAssign;
                <$L as LmMulAssign<&$R>>::mul_assign(&mut self, &rhs)?;
                Ok(self)
            }
        }
    };
    // L: owned + R: borrowed
    (@mul_ob $L:ty, $R:ty) => {
        impl std::ops::Mul<&$R> for $L
        where
            for<'r> $L: $crate::ops::LmMulAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn mul(mut self, rhs: &$R) -> Self::Output {
                use $crate::ops::LmMulAssign;
                <$L as LmMulAssign<&$R>>::mul_assign(&mut self, rhs)?;
                Ok(self)
            }
        }
    };
    // L: borrowed + R: owned
    (@mul_bo $L:ty, $R:ty) => {
        impl std::ops::Mul<$R> for &$L
        where
            for<'r> $L: $crate::ops::LmMulAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn mul(self, rhs: $R) -> Self::Output {
                use $crate::ops::LmMulAssign;
                let mut new = self.clone();
                <$L as LmMulAssign<&$R>>::mul_assign(&mut new, &rhs)?;
                Ok(new)
            }
        }
    };
    // L: borrowed + R: borrowed
    (@mul_bb $L:ty, $R:ty) => {
        impl std::ops::Mul<&$R> for &$L
        where
            for<'r> $L: $crate::ops::LmMulAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn mul(self, rhs: &$R) -> Self::Output {
                use $crate::ops::LmMulAssign;
                let mut new = self.clone();
                <$L as LmMulAssign<&$R>>::mul_assign(&mut new, rhs)?;
                Ok(new)
            }
        }
    };
    // LmMulAssign for owned R.
    (@mul_assign_o $L:ty, $R:ty) => {
        impl $crate::ops::LmMulAssign<$R> for $L
        where
            for<'r> $L: $crate::ops::LmMulAssign<&'r $R>,
        {
            fn mul_assign(&mut self, rhs: $R) -> lunamodel_error::LunaModelResult<()> {
                use $crate::ops::LmMulAssign;
                <$L as LmMulAssign<&$R>>::mul_assign(self, &rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! rmuls {
    ($L:ty => $( $R:ty ) ,+ $(,)?) => {
        // std::ops::Mul
        // L: owned + R: owned
        $( rmuls!(@mul_oo $L, $R); )*
        // L: owned + R: borrowed
        $( rmuls!(@mul_ob $L, $R); )*
        // L: borrowed + R: owned
        $( rmuls!(@mul_bo $L, $R); )*
        // L: borrowed + R: borrowed
        $( rmuls!(@mul_bb $L, $R); )*
    };

    // sub rules for each trait
    // L: owned + R: owned
    (@mul_oo $L:ty, $R:ty) => {
        impl std::ops::Mul<$L> for $R
        where
            for<'r> $L: $crate::ops::LmMulAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn mul(self, mut rhs: $L) -> Self::Output {
                use $crate::ops::LmMulAssign;
                <$L as LmMulAssign<&$R>>::mul_assign(&mut rhs, &self)?;
                Ok(rhs)
            }
        }
    };
    // L: owned + R: borrowed
    (@mul_ob $L:ty, $R:ty) => {
        impl std::ops::Mul<&$L> for $R
        where
            for<'r> $L: $crate::ops::LmMulAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn mul(self, rhs: &$L) -> Self::Output {
                use $crate::ops::LmMulAssign;
                let mut new = rhs.clone();
                <$L as LmMulAssign<&$R>>::mul_assign(&mut new, &self)?;
                Ok(new)
            }
        }
    };
    // L: borrowed + R: owned
    (@mul_bo $L:ty, $R:ty) => {
        impl std::ops::Mul<$L> for &$R
        where
            for<'r> $L: $crate::ops::LmMulAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn mul(self, mut rhs: $L) -> Self::Output {
                use $crate::ops::LmMulAssign;
                <$L as LmMulAssign<&$R>>::mul_assign(&mut rhs, &self)?;
                Ok(rhs)
            }
        }
    };
    // L: borrowed + R: borrowed
    (@mul_bb $L:ty, $R:ty) => {
        impl std::ops::Mul<&$L> for &$R
        where
            for<'r> $L: crate::ops::LmMulAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<crate::expression::Expression>;
            fn mul(self, rhs: &$L) -> Self::Output {
                use crate::ops::LmMulAssign;
                let mut new = rhs.clone();
                <$L as LmMulAssign<&$R>>::mul_assign(&mut new, self)?;
                Ok(new)
            }
        }
    };
}
