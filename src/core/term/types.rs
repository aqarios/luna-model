#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct OneVarTerm<Index, Bias> {
    pub index: Index,
    pub bias: Bias,
}

pub trait OneVarTermConstruction<Index, Bias> {
    fn new(index: Index, bias: Bias) -> Self;
    fn new_default(index: Index) -> Self;
}

impl<Index, Bias> OneVarTermConstruction<Index, Bias> for OneVarTerm<Index, Bias>
where
    Bias: Default,
{
    fn new(index: Index, bias: Bias) -> Self {
        Self { index, bias }
    }

    fn new_default(index: Index) -> Self {
        Self {
            index,
            bias: Bias::default(),
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct OutOfRangeError(String);
//
// #[derive(Debug, Clone)]
// pub struct NoInteractionError(String);

pub enum ExpressionError<'a> {
    OutOfRange(&'a str),
    NoInteraction(&'a str),
    NoAdjacency(&'a str),
}

pub type ExpressionResult<'a, T> = Result<T, ExpressionError<'a>>;
