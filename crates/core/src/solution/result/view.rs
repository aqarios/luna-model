use std::ops::Index;

use derive_more::Deref;
use std::collections::HashMap;
use lunamodel_types::Bias;

use crate::solution::Solution;
use crate::solution::sample::SampleView;

#[derive(Deref)]
pub struct ResultView<'s> {
    sample: SampleView<'s>,
}

impl<'s> ResultView<'s> {
    pub fn new(sol: &'s Solution, index: usize) -> Self {
        SampleView::new(sol, index).into()
    }

    pub fn counts(&self) -> usize {
        self.sample.sol.counts[self.idx]
    }

    pub fn obj_value(&self) -> Option<f64> {
        self.sample.sol.obj_values.as_ref().map(|v| v[self.idx])
    }

    pub fn raw_energy(&self) -> Option<f64> {
        self.sample.sol.raw_energies.as_ref().map(|v| v[self.idx])
    }

    pub fn constraints(&self) -> Option<HashMap<String, bool>> {
        let constr = &self.sample.sol.constraints;
        match constr.is_empty() {
            true => None,
            false => Some(
                constr
                    .iter()
                    .map(|(cname, cs)| (cname.clone(), cs[self.idx]))
                    .collect(),
            ),
        }
    }

    pub fn variable_bounds(&self) -> Option<HashMap<String, bool>> {
        let vbounds = &self.sample.sol.variable_bounds;
        match vbounds.is_empty() {
            true => None,
            false => Some(
                vbounds
                    .iter()
                    .map(|(cname, cs)| (cname.clone(), cs[self.idx]))
                    .collect(),
            ),
        }
    }

    pub fn feasible(&self) -> Option<bool> {
        self.sample.sol.feasible.as_ref().map(|v| v[self.idx])
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
