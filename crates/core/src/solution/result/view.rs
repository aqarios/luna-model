use std::ops::Index;

use derive_more::Deref;
use lunamodel_types::Bias;
use std::collections::HashMap;

use crate::solution::Solution;
use crate::solution::sample::SampleView;

/// Borrowed view over one solution row together with result metadata.
///
/// `ResultView` dereferences to [`SampleView`], so code that only needs the
/// actual assignments can use it like a sample view while still having access to
/// counts, objective values, feasibility, and constraint/bound summaries.
#[derive(Deref)]
pub struct ResultView<'s> {
    sample: SampleView<'s>,
}

impl<'s> ResultView<'s> {
    /// Creates a result view for the row at `index`.
    pub fn new(sol: &'s Solution, index: usize) -> Self {
        SampleView::new(sol, index).into()
    }

    /// Returns how often this row occurred in the original solution.
    pub fn counts(&self) -> usize {
        self.sample.sol.counts[self.idx]
    }

    /// Returns the model-evaluated objective value for this row, if available.
    pub fn obj_value(&self) -> Option<f64> {
        self.sample.sol.obj_values.as_ref().map(|v| v[self.idx])
    }

    /// Returns the raw solver-provided energy for this row, if available.
    pub fn raw_energy(&self) -> Option<f64> {
        self.sample.sol.raw_energies.as_ref().map(|v| v[self.idx])
    }

    /// Returns per-constraint feasibility flags keyed by constraint name.
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

    /// Returns per-variable bound checks keyed by variable name.
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

    /// Returns whether the row is globally feasible, if that field was evaluated.
    pub fn feasible(&self) -> Option<bool> {
        self.sample.sol.feasible.as_ref().map(|v| v[self.idx])
    }
}

impl<'s> From<SampleView<'s>> for ResultView<'s> {
    /// Upgrades a sample view into a result view over the same row.
    fn from(sample: SampleView<'s>) -> Self {
        Self { sample }
    }
}

impl<'s> From<(&'s Solution, usize)> for ResultView<'s> {
    /// Creates a result view from a `(solution, row)` pair.
    fn from(value: (&'s Solution, usize)) -> Self {
        let (sol, idx) = value;
        Self::new(sol, idx)
    }
}

impl<'s> Index<&str> for ResultView<'s> {
    type Output = Bias;

    /// Indexes the row by variable name.
    fn index(&self, var: &str) -> &Self::Output {
        &self.sample[var]
    }
}
