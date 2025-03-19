use crate::core::expression::{AssignmentConstraints, BiasConstraints};
use crate::core::solution::timing::Timing;

/// A view into a certain sample of a solution and its corresponding metadata.
#[derive(Debug, Clone)]
pub struct ResView<'a, Assignment, Bias>
where
    Assignment: AssignmentConstraints,
    Bias: BiasConstraints,
{
    /// The solution bitstring.
    pub sample: &'a Vec<Assignment>,
    /// How often this result's sample occurred in the solution.
    pub num_occurrences: usize,
    /// The objective value computed from an AqModel. If not present, a raw value from the solver
    /// may be used. None, if none of these are present.
    pub obj_value: Option<Bias>,
    /// Boolean flag for each single constraint whether it's satisfied.
    pub constraint_satisfaction: Option<&'a Vec<bool>>,
    /// Whether all constraints are satisfied.
    pub feasible: Option<bool>,
}

impl<'a, Assignment, Bias> ResView<'a, Assignment, Bias>
where
    Assignment: AssignmentConstraints,
    Bias: BiasConstraints,
{
    pub fn new(
        sample: &'a Vec<Assignment>,
        num_occurrences: usize,
        obj_value: Option<Bias>,
        constraint_satisfaction: Option<&'a Vec<bool>>,
        feasible: Option<bool>,
    ) -> Self {
        Self {
            sample,
            num_occurrences,
            obj_value,
            constraint_satisfaction,
            feasible,
        }
    }
}

pub struct Res<Assignment, Bias> {
    /// The solution bitstring.
    pub sample: Vec<Assignment>,
    /// The objective value computed from an AqModel. If not present, a raw value from the solver
    /// may be used. None, if none of these are present.
    pub obj_value: Option<Bias>,
    /// Boolean flag for each single constraint whether it's satisfied.
    pub constraint_satisfaction: Option<Vec<bool>>,
    /// Whether all constraints are satisfied.
    pub feasible: Option<bool>,
}

/// The solutions object for AQMs. It doesn't have any knowledge about the corresponding AQM or
/// about the environment the model was created in. Instead, for each sample, we expect the indices
/// of the solution to be aligned with the variable indices of the model's environment.
#[derive(Debug, Clone, Default)]
pub struct Solution<Assignment, Bias>
where
    Assignment: AssignmentConstraints,
    Bias: BiasConstraints,
{
    /// A collection of samples. Each inner vec corresponds to a single sample, i.e., an assignment
    /// of a value to each model variable. `samples.len()` can be expected to always correspond
    /// exactly to the number of results available in the solution.
    pub samples: Vec<Vec<Assignment>>,
    /// How often each result occurs in the solution. `num_occurrences.len()` can be expected to
    /// always be equal to `samples.len()`
    pub num_occurrences: Vec<usize>,
    /// Objetive values as computed by the corresponding AQM. May be empty for solutions that
    /// haven't yet been evaluated.
    pub obj_values: Vec<Bias>,
    /// Objetive values as computed by the solver. May be empty if the solver does not provide
    /// energies in its solution format. May be different from `obj_values`, e.g., because an offset
    /// was neglected, or the AQM was transformed before being solved.
    pub raw_energies: Vec<Bias>,
    /// Boolean flag for each single constraint whether it's satisfied. Each inner vec corresponds
    /// to one sample, i.e., `constraints[i]` corresponds to `samples[0]`. May be empty for
    /// solutions that haven't yet been evaluated.
    constraints: Vec<Vec<bool>>,
    /// Boolean flag for each sample whether it's feasible, i.e., whether all constraints are
    /// satisfied. In other words, `feasible[i]` iff. `all(constraints[i])`. May be empty for
    /// solutions that haven't yet been evaluated.
    feasible: Vec<bool>,
    // /// Metadata that may be useful for explaining why a constraint is not satisfied, e.g., the eval
    // /// of a lhs.
    best_sample_idx: Option<usize>,
    /// Runtime metrics of the solution.
    pub timing: Option<Timing>,
}

impl<Assignment, Bias> Solution<Assignment, Bias>
where
    Assignment: AssignmentConstraints,
    Bias: BiasConstraints,
{
    pub fn position(&self, sample: &Vec<Assignment>) -> Option<usize> {
        // TODO: find out whether this check is efficient enough or there's sth better
        self.samples.iter().position(|x| x == sample)
    }

    /// Extend a solution with a sample, without computing any objective values or similar.
    /// `num_occurrences` means how often this value occurs.
    pub fn extend(&mut self, sample: Vec<Assignment>, num_occurrences: usize) -> &mut Self {
        if let Some(idx) = self.position(&sample) {
            self.num_occurrences[idx] += num_occurrences;
        } else {
            self.samples.push(sample);
            self.num_occurrences.push(num_occurrences);
        }
        self
    }

    /// Extend a solution with a sample, without computing any objective values or similar.
    /// `num_occurrences` means how often this value occurs.
    /// In contrast to `extend`, this method does not check whether the sample is already part of
    /// the solution.
    pub fn extend_no_agg(&mut self, sample: Vec<Assignment>, num_occurrences: usize) -> &mut Self {
        self.samples.push(sample);
        self.num_occurrences.push(num_occurrences);
        self
    }

    /// Iterate over the single results of the solution
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = ResView<Assignment, Bias>> + use<'_, Assignment, Bias> {
        (0..self.samples.len()).map(|i| self.get_result(i).unwrap())
    }

    pub fn get_result(&self, index: usize) -> Option<ResView<Assignment, Bias>> {
        if index >= self.samples.len() {
            return None;
        }
        let obj_value = match (self.obj_values.get(index), self.raw_energies.get(index)) {
            (Some(&bias), _) => Some(bias),
            (_, Some(&bias)) => Some(bias),
            (_, _) => None,
        };
        let constraints = self.constraints.get(index);
        let feasible = match &self.feasible.get(index) {
            None => None,
            Some(&feas) => Some(feas),
        };

        Some(ResView::new(
            &self.samples[index],
            self.num_occurrences[index],
            obj_value,
            constraints,
            feasible,
        ))
    }
}
