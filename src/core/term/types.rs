#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OneVarTerm<Index, Bias> {
    pub index: Index,
    pub bias: Bias,
}

pub type SizeType = usize;

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
