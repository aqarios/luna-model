pub trait AddToExpression<Index, Bias, Other> {
    type Output;

    fn add(self, rhs: Other) -> Self::Output;
}

pub trait AddAssignToExpression<Index, Bias, Other> {
    type Output;

    fn add_assign(&mut self, rhs: Other) -> Self::Output;
}

pub trait MulToExpression<Index, Bias, Other> {
    type Output;

    fn mul(self, rhs: Other) -> Self::Output;
}

pub trait MulAssignToExpression<Index, Bias, Other> {
    type Output;

    fn mul_assign(&mut self, rhs: Other) -> Self::Output;
}
