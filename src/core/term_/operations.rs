pub trait TermCreation {
    fn from_other(base: &Self) -> Self;
}

pub trait TermAddition<T> {
    /// Adding `Self` and `rhs` producing a new value of type `Self`
    fn add(&self, rhs: &T) -> Self
    where
        Self: Sized + TermCreation,
    {
        let mut out = TermCreation::from_other(self);
        TermAddition::add_assign(&mut out, rhs);
        out
    }
    /// Adding `rhs` to `self`
    fn add_assign(&mut self, rhs: &T);
}

// pub trait TermEnvAddition<T> {
//     /// Adding `Self` and `rhs` producing a new value of type `Self`.
//     /// This trait should be used when information from the environment
//     /// is required.
//     fn add(&self, rhs: &T, env: Environment) -> Self
//     where
//         Self: Sized + TermCreation,
//     {
//         let mut out = TermCreation::from_other(self);
//         TermEnvAddition::add_assign(&mut out, rhs);
//         out
//     }
//     /// Adding `rhs` to `self`
//     fn add_assign(&mut self, rhs: &T);
// }

pub trait TermSubtraction<T> {
    /// Subtacting `Self` and `rhs` producing a new value of type `Self`
    fn sub(&self, rhs: &T) -> Self
    where
        Self: Sized + TermCreation,
    {
        let mut out = TermCreation::from_other(self);
        TermSubtraction::sub_assign(&mut out, rhs);
        out
    }

    /// Subtracting `rhs` to `self`
    fn sub_assign(&mut self, rhs: &T);
}

pub trait TermMultiplication<T> {
    /// Multiplying `Self` and `rhs` producing a new value of type `Self`
    fn mul(&self, rhs: &T) -> Self
    where
        Self: Sized + TermCreation,
    {
        let mut out = TermCreation::from_other(self);
        TermMultiplication::mul_assign(&mut out, rhs);
        out
    }

    /// Multiplying `rhs` to `self`
    fn mul_assign(&mut self, rhs: &T);
}

// pub trait TermBaseOperations<T: TermAddition<T> + TermSubtraction<T>> {}
