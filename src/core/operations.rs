pub trait AddToExpression<Index, Bias, Other> {
    type Output;

    fn add(self, rhs: Other) -> Self::Output;
}

pub trait AddAssignToExpression<Index, Bias, Other> {
    type Output;

    fn add_assign(&mut self, rhs: Other) -> Self::Output;
}
