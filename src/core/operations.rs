/// Custom Add to result in an Expression for a more fine grained control, especially for the
/// AddAssignToExpression trait which we require to be able to return an Error in
/// constrast to the default AddAssign trait that does not provide this flexibility.
pub trait AddToExpression<Index, Bias, Other> {
    type Output;

    fn add(self, rhs: Other) -> Self::Output;
}

/// Custom Mul to result in an Expression for a more fine grained control, especially for the
/// MulAssignToExpression trait which we require to be able to return an Error in
/// constrast to the default MulAssign trait that does not provide this flexibility.
pub trait MulToExpression<Index, Bias, Other> {
    type Output;

    fn mul(self, rhs: Other) -> Self::Output;
}

/// Custom Sub to result in an Expression for a more fine grained control, especially for the
/// SubAssignToExpression trait which we require to be able to return an Error in
/// constrast to the default SubAssign trait that does not provide this flexibility.
pub trait SubToExpression<Index, Bias, Other> {
    type Output;

    fn sub(self, rhs: Other) -> Self::Output;
}

/// Custom RSub to result in an Expression for a more fine grained control. This implements the
/// specific case that self is located on the right hand side of a subtraction.
pub trait RSubToExpression<Index, Bias, Other> {
    type Output;

    fn rsub(self, rhs: Other) -> Self::Output;
}

/// Required to be able to return an Error in constrast to the default AddAssign trait
/// that does not provide this flexibility.
pub trait AddAssignToExpression<Index, Bias, Other> {
    type Output;

    fn add_assign(&mut self, rhs: Other) -> Self::Output;
}

/// Required to be able to return an Error in constrast to the default MulAssign trait
/// that does not provide this flexibility.
pub trait MulAssignToExpression<Index, Bias, Other> {
    type Output;

    fn mul_assign(&mut self, rhs: Other) -> Self::Output;
}
