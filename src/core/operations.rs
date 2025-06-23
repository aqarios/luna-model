/// Custom Add to result in an Expression for a more fine grained control, especially for the
/// AddAssignToExpression trait which we require to be able to return an Error in
/// constrast to the default AddAssign trait that does not provide this flexibility.
pub trait AddToExpression<Rhs> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

/// Custom Mul to result in an Expression for a more fine grained control, especially for the
/// MulAssignToExpression trait which we require to be able to return an Error in
/// constrast to the default MulAssign trait that does not provide this flexibility.
pub trait MulToExpression<Rhs> {
    type Output;

    fn mul(self, rhs: Rhs) -> Self::Output;
}

/// Custom Sub to result in an Expression for a more fine grained control, especially for the
/// SubAssignToExpression trait which we require to be able to return an Error in
/// constrast to the default SubAssign trait that does not provide this flexibility.
pub trait SubToExpression<Rhs> {
    type Output;

    fn sub(self, rhs: Rhs) -> Self::Output;
}

/// Custom SubAssign to result in an Expression for a more fine grained control.
pub trait SubAssignToExpression<Rhs> {
    type Output;

    fn sub_assign(&mut self, rhs: Rhs) -> Self::Output;
}

/// Custom RSub to result in an Expression for a more fine grained control. This implements the
/// specific case that self is located on the right hand side of a subtraction.
pub trait RSubToExpression<Rhs> {
    type Output;

    fn rsub(self, rhs: Rhs) -> Self::Output;
}

/// Required to be able to return an Error in constrast to the default AddAssign trait
/// that does not provide this flexibility.
pub trait AddAssignToExpression<Rhs> {
    type Output;

    fn add_assign(&mut self, rhs: Rhs) -> Self::Output;
}

/// Required to be able to return an Error in constrast to the default MulAssign trait
/// that does not provide this flexibility.
pub trait MulAssignToExpression<Rhs> {
    type Output;

    fn mul_assign(&mut self, rhs: Rhs) -> Self::Output;
}

/// Custom Sub to result in an Expression for a more fine grained control, especially
/// regarding the Bias.
pub trait NegToExpression {
    type Output;

    fn neg(self) -> Self::Output;
}

// /// Custom Sub to result in an Expression for a more fine grained control, especially
// /// regarding the Bias.
// pub trait NegAssignToExpression<Index, Bias> {
//     fn neg_assign(self);
// }
