use std::ops::Index;

use lunamodel_types::Bias;

use crate::solution::Solution;
use crate::solution::sample::SampleView;

pub struct ResultView<'s> {
    sample: SampleView<'s>,
}

impl<'s> ResultView<'s> {
    pub fn new(sol: &'s Solution, index: usize) -> Self {
        SampleView::new(sol, index).into()
    }
}

impl<'s> From<SampleView<'s>> for ResultView<'s> {
    fn from(sample: SampleView<'s>) -> Self {
        Self { sample }
    }
}

impl<'s> From<(&'s Solution, usize)> for ResultView<'s> {
    fn from(value: (&'s Solution, usize)) -> Self {
        let (sol, idx) = value;
        Self::new(sol, idx)
    }
}

impl<'s> Index<&str> for ResultView<'s> {
    type Output = Bias;

    fn index(&self, var: &str) -> &Self::Output {
        &self.sample[var]
    }
}
