pub trait AddToExpression<Index, Bias, Other> {
    type Output;

    fn add(self, rhs: Other) -> Self::Output;
}
