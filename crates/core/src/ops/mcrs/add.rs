//! Generated mixed-reference addition implementations.

#[macro_export]
macro_rules! adds {
    ($L:ty => $( $R:ty ) ,+ $(,)?) => {
        // std::ops::Add
        // L: owned + R: owned
        $( adds!(@add_oo $L, $R); )*
        // L: owned + R: borrowed
        $( adds!(@add_ob $L, $R); )*
        // L: borrowed + R: owned
        $( adds!(@add_bo $L, $R); )*
        // L: borrowed + R: borrowed
        $( adds!(@add_bb $L, $R); )*
        // std::ops::AddAssign
        $( adds!(@add_assign_o $L, $R); )*
    };

    // sub rules for each trait
    // L: owned + R: owned
    (@add_oo $L:ty, $R:ty) => {
        impl std::ops::Add<$R> for $L
        where
            for<'r> $L: $crate::ops::LmAddAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn add(mut self, rhs: $R) -> Self::Output {
                use $crate::ops::LmAddAssign;
                <$L as LmAddAssign<&$R>>::add_assign(&mut self, &rhs)?;
                Ok(self)
            }
        }
    };
    // L: owned + R: borrowed
    (@add_ob $L:ty, $R:ty) => {
        impl std::ops::Add<&$R> for $L
        where
            for<'r> $L: $crate::ops::LmAddAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn add(mut self, rhs: &$R) -> Self::Output {
                use $crate::ops::LmAddAssign;
                <$L as LmAddAssign<&$R>>::add_assign(&mut self, rhs)?;
                Ok(self)
            }
        }
    };
    // L: borrowed + R: owned
    (@add_bo $L:ty, $R:ty) => {
        impl std::ops::Add<$R> for &$L
        where
            for<'r> $L: $crate::ops::LmAddAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn add(self, rhs: $R) -> Self::Output {
                use $crate::ops::LmAddAssign;
                let mut new = self.clone();
                <$L as LmAddAssign<&$R>>::add_assign(&mut new, &rhs)?;
                Ok(new)
            }
        }
    };
    // L: borrowed + R: borrowed
    (@add_bb $L:ty, $R:ty) => {
        impl std::ops::Add<&$R> for &$L
        where
            for<'r> $L: $crate::ops::LmAddAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn add(self, rhs: &$R) -> Self::Output {
                use $crate::ops::LmAddAssign;
                let mut new = self.clone();
                <$L as LmAddAssign<&$R>>::add_assign(&mut new, rhs)?;
                Ok(new)
            }
        }
    };
    // LmAddAssign for owned R.
    (@add_assign_o $L:ty, $R:ty) => {
        impl $crate::ops::LmAddAssign<$R> for $L
        where
            for<'r> $L: $crate::ops::LmAddAssign<&'r $R>,
        {
            fn add_assign(&mut self, rhs: $R) -> lunamodel_error::LunaModelResult<()> {
                use $crate::ops::LmAddAssign;
                <$L as LmAddAssign<&$R>>::add_assign(self, &rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! radds {
    ($L:ty => $( $R:ty ) ,+ $(,)?) => {
        // std::ops::Add
        // L: owned + R: owned
        $( radds!(@add_oo $L, $R); )*
        // L: owned + R: borrowed
        $( radds!(@add_ob $L, $R); )*
        // L: borrowed + R: owned
        $( radds!(@add_bo $L, $R); )*
        // L: borrowed + R: borrowed
        $( radds!(@add_bb $L, $R); )*
    };

    // sub rules for each trait
    // L: owned + R: owned
    (@add_oo $L:ty, $R:ty) => {
        impl std::ops::Add<$L> for $R
        where
            for<'r> $L: $crate::ops::LmAddAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn add(self, mut rhs: $L) -> Self::Output {
                use $crate::ops::LmAddAssign;
                <$L as LmAddAssign<&$R>>::add_assign(&mut rhs, &self)?;
                Ok(rhs)
            }
        }
    };
    // L: owned + R: borrowed
    (@add_ob $L:ty, $R:ty) => {
        impl std::ops::Add<&$L> for $R
        where
            for<'r> $L: $crate::ops::LmAddAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn add(self, rhs: &$L) -> Self::Output {
                use $crate::ops::LmAddAssign;
                let mut new = rhs.clone();
                <$L as LmAddAssign<&$R>>::add_assign(&mut new, &self)?;
                Ok(new)
            }
        }
    };
    // L: borrowed + R: owned
    (@add_bo $L:ty, $R:ty) => {
        impl std::ops::Add<$L> for &$R
        where
            for<'r> $L: $crate::ops::LmAddAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn add(self, mut rhs: $L) -> Self::Output {
                use $crate::ops::LmAddAssign;
                <$L as LmAddAssign<&$R>>::add_assign(&mut rhs, &self)?;
                Ok(rhs)
            }
        }
    };
    // L: borrowed + R: borrowed
    (@add_bb $L:ty, $R:ty) => {
        impl std::ops::Add<&$L> for &$R
        where
            for<'r> $L: $crate::ops::LmAddAssign<&'r $R>,
        {
            type Output = lunamodel_error::LunaModelResult<$crate::expression::Expression>;
            fn add(self, rhs: &$L) -> Self::Output {
                use $crate::ops::LmAddAssign;
                let mut new = rhs.clone();
                <$L as LmAddAssign<&$R>>::add_assign(&mut new, self)?;
                Ok(new)
            }
        }
    };
}
